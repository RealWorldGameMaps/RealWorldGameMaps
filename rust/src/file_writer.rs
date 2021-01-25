
use std::fs::File;
use std::io::SeekFrom;
use std::fs::OpenOptions;

pub struct FileWriter {
  bytes: Vec<u8>,
  filepath: &str,
  pub little_endian: bool,
}

impl FileWriter {

  pub fn new(filepath: &str, initial_buffer_size: usize) -> FileWriter {
    FileWriter {
      bytes: Vec::with_capacity(initial_buffer_size),
      filepath,
      little_endian: true,
    }
  }

  pub fn write(&self) {
    let file = File::open(self.filepath).unwrap();

    file.write_all(self.bytes).unwrap();
  }

  pub fn write_str(&self, from: usize, value: &str, max_length: usize) {
		self.bytes.splice(from .. from + max_length, value.as_bytes().iter().cloned());
  }

  pub fn write_bytes(&self, from: usize, value: Vec<u8>) {
		self.bytes.splice(from .. from + value.len(), value);
	}

  pub fn write_u8(&self, from: usize, value: u8) {
    self.bytes[from] = value.as();
	}

  pub fn write_u16(&self, from: usize, value: u16) {
    if self.little_endian {
      self.bytes.splice(from .. from + 2, value.into_be_bytes());
    } else {
      self.bytes.splice(from .. from + 2, value);
    }
  }

}
