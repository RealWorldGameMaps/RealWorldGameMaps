
use std::fs::File;
use std::io::Read;
use std::convert::TryInto;
use std::str;

pub struct FileReader {
    bytes: Vec<u8>,
    pub little_endian: bool,
}

impl FileReader {

    pub fn new(filepath: &str) -> FileReader {
        let mut file = File::open(&filepath).unwrap();
        let mut bytes = Vec::new();
        file.by_ref().read_to_end(&mut bytes).unwrap();
    
        FileReader {
            bytes,
            little_endian: true,
        }
    }

    // this function does not respect the endianess of the FileReader
    pub fn read_str(&self, from: usize, max_length: usize) -> &str {
        let length = &self.bytes[from .. from + max_length].iter().position(|&byte| byte == 0).unwrap_or(max_length);
        let spliced = &self.bytes[from .. from + length];
        str::from_utf8(spliced).unwrap()
    }

    pub fn read_u32(&self, from: usize) -> u32 {
        let spliced = &self.bytes[from .. from + 4];
        if self.little_endian {
            return u32::from_le_bytes(spliced.try_into().unwrap());
        } else {
            return u32::from_be_bytes(spliced.try_into().unwrap());
        }
    }

    pub fn read_i32(&self, from: usize) -> i32 {
        let spliced = &self.bytes[from .. from + 4];
        if self.little_endian {
            return i32::from_le_bytes(spliced.try_into().unwrap());
        } else {
            return i32::from_be_bytes(spliced.try_into().unwrap());
        }
    }

}