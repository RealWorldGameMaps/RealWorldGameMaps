
pub struct FileWriter {
  bytes: Vec<u8>,
  filepath: &str,
  pub little_endian: bool,
}

impl FileWriter {

  pub fn new(filepath: &str) -> FileWriter {
    FileWriter {
      bytes: Vec::new(),
      filepath,
      little_endian: true,
    }
  }

  pub fn write(&self) {
    let file = File::open(self.filepath).unwrap();
    file.write_all(self.bytes).unwrap();
  }

  pub fn write_string(&self, from: usize, value: &str,max_length: usize) {

  }

  pub fn write_bytes(&self, from: usize, value: Vec<u8>) {

  }

  pub fn write_u8(&self, from: usize, value: u8) {

  }

}
