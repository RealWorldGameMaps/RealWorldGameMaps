
use std::fs::File;
use std::io::Read;
use std::convert::TryInto;
use std::str;

pub struct FileReader {
    bytes: Vec<u8>,
    pub little_endian: bool,
}

impl FileReader {

    // Read in the whole file
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

    // Read unsigned 32-bit integer
    pub fn read_u32(&self, from: usize) -> u32 {
        let spliced = &self.bytes[from .. from + 4];
        if self.little_endian {
            return u32::from_le_bytes(spliced.try_into().unwrap());
        } else {
            return u32::from_be_bytes(spliced.try_into().unwrap());
        }
    }

    // Read signed 32-bit integer
    pub fn read_i32(&self, from: usize) -> i32 {
        let spliced = &self.bytes[from .. from + 4];
        if self.little_endian {
            return i32::from_le_bytes(spliced.try_into().unwrap());
        } else {
            return i32::from_be_bytes(spliced.try_into().unwrap());
        }
    }

    // Read unsigned 16-bit integer from offset
    pub fn read_u16(&self, from: usize) -> u16 {
        let spliced = &self.bytes[from .. from + 2];
        if self.little_endian {
            return u16::from_le_bytes(spliced.try_into().unwrap());
        } else {
            return u16::from_be_bytes(spliced.try_into().unwrap());
        }
    }

    // Read signed 16-bit integer from offset
    pub fn read_i16(&self, from: usize) -> i16 {
        let spliced = &self.bytes[from .. from + 2];
        if self.little_endian {
            return i16::from_le_bytes(spliced.try_into().unwrap());
        } else {
            return i16::from_be_bytes(spliced.try_into().unwrap());
        }
    }

    // Read unsigned 8-bit integer from offset
    pub fn read_u8(&self, from: usize) -> u8 {
        let spliced = &self.bytes[from .. from + 1];
        if self.little_endian {
            return u8::from_le_bytes(spliced.try_into().unwrap());
        } else {
            return u8::from_be_bytes(spliced.try_into().unwrap());
        }
    }

    // Read signed 8-bit integer from offset
    pub fn read_i8(&self, from: usize) -> i8 {
        let spliced = &self.bytes[from .. from + 1];
        if self.little_endian {
            return i8::from_le_bytes(spliced.try_into().unwrap());
        } else {
            return i8::from_be_bytes(spliced.try_into().unwrap());
        }
    }

    // Read `length` bytes, starting at `from`
    pub fn read_bytes(&self, from: usize, length: usize) -> Vec<u8> {
        let spliced = &self.bytes[from .. from + length];
        spliced.try_into().unwrap()
    }

}