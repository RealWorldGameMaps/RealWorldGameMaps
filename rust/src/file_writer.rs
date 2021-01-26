
extern crate bincode;

use bincode::serialize;

use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub struct FileWriter {
	pub bytes: Vec<u8>,
  pub little_endian: bool,
}

impl FileWriter {

  pub fn new(initial_buffer_size: usize) -> FileWriter {
    FileWriter {
			bytes: Vec::with_capacity(initial_buffer_size),
      little_endian: true,
    }
	}

	pub fn write_to_file(&self, filepath: &str) {
		let mut file = OpenOptions::new().read(true).write(true).truncate(true).create(true).open(filepath).unwrap();
		file.write_all(&self.bytes[..]).unwrap();
		file.flush().unwrap();
	}

  pub fn write_str(&mut self, value: &str, max_length: usize) {
		for i in 0..value.len() {
			self.bytes.push(value.as_bytes()[i]);
		}
		for _i in value.len()..max_length {
			self.bytes.push(0);
		}
	}

	pub fn write_bytes(&mut self, value: Vec<u8>) {
		self.bytes.write_all(&value).unwrap();
	}

	pub fn write_u32(&mut self, value: u32) {
		let data = serialize(&value).unwrap();
		self.write_bytes(data);
	}

	pub fn write_i32(&mut self, value: i32) {
		let data = serialize(&value).unwrap();
		self.write_bytes(data);
	}

	pub fn write_u16(&mut self, value: u16) {
		let data = serialize(&value).unwrap();
		self.write_bytes(data);
	}

	pub fn write_i16(&mut self, value: i16) {
		let data = serialize(&value).unwrap();
		self.write_bytes(data);
	}

	pub fn write_u8(&mut self, value: u8) {
		let data = serialize(&value).unwrap();
		self.write_bytes(data);
	}

	pub fn write_i8(&mut self, value: i8) {
		let data = serialize(&value).unwrap();
		self.write_bytes(data);
	}


}
