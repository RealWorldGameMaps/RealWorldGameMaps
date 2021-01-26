
extern crate bincode;

use bincode::serialize;

use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub struct FileWriter {
	file: File,
  pub little_endian: bool,
}

impl FileWriter {

  pub fn new(filepath: &str) -> FileWriter {
		let file = OpenOptions::new().read(true).write(true).truncate(true).create(true).open(filepath);
    FileWriter {
			file: file.unwrap(),
      little_endian: true,
    }
  }

	pub fn flush(&mut self) {
		self.file.flush().unwrap();
	}

  pub fn write_str(&mut self, value: &str, max_length: usize) {
		let mut bytes = Vec::with_capacity(max_length);

		for i in 0..value.len() {
			bytes.push(value.as_bytes()[i]);
		}
		for _i in value.len()..max_length {
			bytes.push(0);
		}

		self.file.write(&bytes).unwrap();
	}

	pub fn write_bytes(&mut self, value: Vec<u8>) {
		self.file.write(&value).unwrap();
	}

	pub fn write_u32(&mut self, value: u32) {
		let data = serialize(&value).unwrap();
		self.file.write(&data).unwrap();
	}

	pub fn write_i32(&mut self, value: i32) {
		let data = serialize(&value).unwrap();
		self.file.write(&data).unwrap();
	}

	pub fn write_u16(&mut self, value: u16) {
		let data = serialize(&value).unwrap();
		self.file.write(&data).unwrap();
	}

	pub fn write_i16(&mut self, value: i16) {
		let data = serialize(&value).unwrap();
		self.file.write(&data).unwrap();
	}

	pub fn write_u8(&mut self, value: u8) {
		let data = serialize(&value).unwrap();
		self.file.write(&data).unwrap();
	}

	pub fn write_i8(&mut self, value: i8) {
		let data = serialize(&value).unwrap();
		self.file.write(&data).unwrap();
	}


}
