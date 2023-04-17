use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::io::{Read, Result, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use super::block::BlockId;
use super::config::*;
use super::page::Page;

pub struct FileManager {
    dir: PathBuf,
    open_files: HashMap<PathBuf, File>,
    is_new: bool,
}

impl FileManager {
    pub fn new(dir: impl AsRef<Path>) -> FileManager {
        let is_new = !dir.as_ref().exists();

        if is_new {
            fs::create_dir_all(dir.as_ref()).unwrap();
        }

        FileManager {
            dir: dir.as_ref().to_path_buf(),
            open_files: HashMap::new(),
            is_new,
        }
    }

    pub fn read(&mut self, blk: &BlockId, page: &mut Page) -> Result<()> {
        let file = self.get_file(&blk.get_file_name())?;
        file.seek(SeekFrom::Start(blk.get_number() * BLOCK_SIZE as u64))?;
        file.read_exact(page.as_mut_slice())?;
        Ok(())
    }

    pub fn write(&mut self, blk: &BlockId, page: &Page) -> Result<()> {
        let file = self.get_file(&blk.get_file_name())?;
        file.seek(SeekFrom::Start(blk.get_number() * BLOCK_SIZE as u64))?;
        file.write_all(page.as_slice())?;
        Ok(())
    }

    pub fn append(&mut self, file_name: impl AsRef<Path>) -> Result<BlockId> {
        let file = self.get_file(file_name.as_ref())?;
        let blk_num = file.metadata()?.len() / BLOCK_SIZE as u64;
        let blk = BlockId::new(blk_num, file_name.as_ref());

        file.seek(SeekFrom::Start(blk.get_number() * BLOCK_SIZE as u64))?;
        file.write_all(&[0; BLOCK_SIZE])?;

        Ok(blk)
    }

    pub fn get_file(&mut self, file_name: impl AsRef<Path>) -> Result<&mut File> {
        let path = file_name.as_ref();
        if !self.open_files.contains_key(path) {
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(self.dir.join(path))?;
            self.open_files.insert(path.to_path_buf(), file);
        }

        Ok(self.open_files.get_mut(path).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_new() {
        let fm = FileManager::new("test_is_new");
        assert_eq!(fm.is_new, true);
        let fm = FileManager::new("test_is_new");
        assert_eq!(fm.is_new, false);

        fs::remove_dir_all("test_is_new").unwrap();
    }

    #[test]
    fn test_io() {
        let mut fm = FileManager::new("test_io");
        let blk = BlockId::new(0, "test_io");
        let mut page = Page::new();
        page.set_int(0, 123);
        fm.write(&blk, &page).unwrap();
        let mut page = Page::new();
        fm.read(&blk, &mut page).unwrap();
        assert_eq!(page.get_int(0), 123);

        fs::remove_dir_all("test_io").unwrap();
    }

    #[test]
    fn test_append() {
        let mut fm = FileManager::new("test_append");

        assert_eq!(
            fm.get_file("test_append")
                .unwrap()
                .metadata()
                .unwrap()
                .len(),
            0
        );

        let blk = fm.append("test_append").unwrap();
        assert_eq!(
            fm.get_file("test_append")
                .unwrap()
                .metadata()
                .unwrap()
                .len(),
            BLOCK_SIZE as u64
        );

        fs::remove_dir_all("test_append").unwrap();
    }
}
