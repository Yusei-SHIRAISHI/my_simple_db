use std::fmt::{Display, Formatter, Result};
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

#[derive(Eq, PartialEq, Debug)]
pub struct BlockId {
    number: u64,
    file_name: PathBuf,
}

impl BlockId {
    pub fn new(number: u64, file_name: impl AsRef<Path>) -> BlockId {
        BlockId {
            number,
            file_name: file_name.as_ref().to_path_buf(),
        }
    }

    pub fn get_number(&self) -> u64 {
        self.number
    }

    pub fn get_file_name(&self) -> &Path {
        &self.file_name
    }
}

impl Hash for BlockId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}

impl Display for BlockId {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "[file {}, block {}]",
            self.file_name.to_str().unwrap(),
            self.number
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block() {
        let block = BlockId::new(1, "file1");
        assert_eq!(block.get_number(), 1);
        assert_eq!(block.get_file_name(), Path::new("file1"));
    }

    #[test]
    fn test_block_hash() {
        use std::collections::HashSet;
        let block1 = BlockId::new(1, "file1");
        let block2 = BlockId::new(1, "file1");
        let block3 = BlockId::new(2, "file1");
        let block4 = BlockId::new(1, "file2");

        let mut set = HashSet::new();
        set.insert(block1);
        assert!(set.contains(&block2));
        assert!(!set.contains(&block3));
        assert!(!set.contains(&block4));
    }
}
