use byteorder::{BigEndian, ByteOrder};
use delegate::delegate;

use super::config::*;

pub struct Page {
    buf: [u8; BLOCK_SIZE],
}

impl Page {

    delegate! {
        to self.buf {
            pub fn as_slice(&self) -> &[u8];
            pub fn as_mut_slice(&mut self) -> &mut [u8];
        }
    }

    pub fn new() -> Page {
        Page {
            buf: [0; BLOCK_SIZE],
        }
    }

    pub fn get_bytes(&self, offset: usize, size: usize) -> &[u8] {
        &self.buf[offset..offset + size]
    }

    pub fn set_bytes(&mut self, offset: usize, val: &[u8]) {
        self.buf[offset..offset + val.len()].clone_from_slice(val);
    }

    pub fn get_int(&self, offset: usize) -> i32 {
        BigEndian::read_i32(&self.buf[offset..offset + std::mem::size_of::<i32>()])
    }

    pub fn set_int(&mut self, offset: usize, val: i32) {
        BigEndian::write_i32(
            &mut self.buf[offset..offset + std::mem::size_of::<i32>()],
            val,
        );
    }

    pub fn get_string(&self, offset: usize) -> String {
        let size = self.get_int(offset) as usize;
        let str_bytes = self.get_bytes(offset + std::mem::size_of::<i32>(), size);

        String::from_utf8(str_bytes.to_vec()).unwrap()
    }

    pub fn set_string(&mut self, offset: usize, val: &str) {
        self.set_int(offset, val.len() as i32);
        self.set_bytes(offset + std::mem::size_of::<i32>(), val.as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io_bytes() {
        let mut page = Page::new();
        let bytes = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        page.set_bytes(0, &bytes);
        assert_eq!(page.get_bytes(0, 10), bytes);
    }

    #[test]
    fn test_io_int() {
        let mut page = Page::new();
        let val = 123456789;
        page.set_int(0, val);
        assert_eq!(page.get_int(0), val);
    }

    #[test]
    fn test_io_string() {
        let mut page = Page::new();
        let val = "Hello, World!";
        page.set_string(0, val);
        assert_eq!(page.get_string(0), val);
    }
}
