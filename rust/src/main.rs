// extern crate bincode;
// extern crate tiff;

use std::str;
use std::convert::TryInto;
// use bincode::{serialize, deserialize};
// use tiff::decoder::{Decoder};

//use std::io;
//use std::io::prelude::*;
//use std::fs::File;

// const MAGIC: &str = "PSMP";
// const VERSION: u32 = 6;
// const HEADER_BYTES: u32 = 12;
// const TILES_PER_PATCH_SIDE: u32 = 16;

#[derive(Debug)]
struct Dinit {
  magic: String,
  droid_version: u32,
  num_droids: u32,
  droids: Vec<Droid>,
}
#[derive(Debug)]
struct Droid {
  name: String,
  id: u32,
  coordinate: Coordinate,
  direction: u32,
  player: u32,
  _dummy_in_fire: u32,
  _dummy_burn_start: u32,
  _dummy_burn_damage: u32,
}
#[derive(Debug)]
struct Coordinate {
  x: u32,
  y: u32,
  z: u32,
}

fn get_str_from_bytes(bytes: &Vec<u8>, from: usize, max_length: usize) -> &str {
  let length = bytes[from .. from + max_length].iter().position(|&byte| byte == 0).unwrap_or(max_length);
  str::from_utf8(&(bytes[from .. from + length])).unwrap()
}

fn get_u32_from_bytes(bytes: &Vec<u8>, from: usize) -> u32 {
  u32::from_le_bytes((&(bytes[from .. from + 4])).try_into().unwrap())
}

fn main() {
  let bytes = vec![
    0x64, 0x69, 0x6e, 0x74,
    0x08, 0x00, 0x00, 0x00,
    0x20, 0x00, 0x00, 0x00,
    0x43, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x75, 0x63,
    0x74, 0x69, 0x6f, 0x6e,
    0x44, 0x72, 0x6f, 0x69,
    0x64, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0xd0, 0x00, 0x00, 0x00,
    0xc0, 0x0d, 0x00, 0x00,
    0xc0, 0x0b, 0x00, 0x00,
    0x46, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x43, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x75, 0x63,
    0x74, 0x69, 0x6f, 0x6e,
    0x44, 0x72, 0x6f, 0x69,
    0x64, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0xd1, 0x00, 0x00, 0x00,
    0xc0, 0x0d, 0x00, 0x00,
    0x40, 0x0c, 0x00, 0x00,
    0x46, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x43, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x75, 0x63,
    0x74, 0x69, 0x6f, 0x6e,
    0x44, 0x72, 0x6f, 0x69,
    0x64, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0xd2, 0x00, 0x00, 0x00,
    0xc0, 0x0c, 0x00, 0x00,
    0x40, 0x0d, 0x00, 0x00,
    0x46, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x43, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x75, 0x63,
    0x74, 0x69, 0x6f, 0x6e,
    0x44, 0x72, 0x6f, 0x69,
    0x64, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0xd3, 0x00, 0x00, 0x00,
    0x40, 0x0c, 0x00, 0x00,
    0x40, 0x0d, 0x00, 0x00,
    0x46, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x43, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x75, 0x63,
    0x74, 0x69, 0x6f, 0x6e,
    0x44, 0x72, 0x6f, 0x69,
    0x64, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0xd5, 0x00, 0x00, 0x00,
    0x40, 0x31, 0x00, 0x00,
    0xc0, 0x09, 0x00, 0x00,
    0x46, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x43, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x75, 0x63,
    0x74, 0x69, 0x6f, 0x6e,
    0x44, 0x72, 0x6f, 0x69,
    0x64, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0xd6, 0x00, 0x00, 0x00,
    0xc0, 0x31, 0x00, 0x00,
    0xc0, 0x09, 0x00, 0x00,
    0x46, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x43, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x75, 0x63,
    0x74, 0x69, 0x6f, 0x6e,
    0x44, 0x72, 0x6f, 0x69,
    0x64, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0xd7, 0x00, 0x00, 0x00,
    0x40, 0x32, 0x00, 0x00,
    0xc0, 0x09, 0x00, 0x00,
    0x46, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x43, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x75, 0x63,
    0x74, 0x69, 0x6f, 0x6e,
    0x44, 0x72, 0x6f, 0x69,
    0x64, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0xd8, 0x00, 0x00, 0x00,
    0xc0, 0x32, 0x00, 0x00,
    0xc0, 0x09, 0x00, 0x00,
    0x46, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x43, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x75, 0x63,
    0x74, 0x69, 0x6f, 0x6e,
    0x44, 0x72, 0x6f, 0x69,
    0x64, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0xda, 0x00, 0x00, 0x00,
    0x40, 0x58, 0x00, 0x00,
    0xc0, 0x0d, 0x00, 0x00,
    0x46, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x02, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x43, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x75, 0x63,
    0x74, 0x69, 0x6f, 0x6e,
    0x44, 0x72, 0x6f, 0x69,
    0x64, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0xdb, 0x00, 0x00, 0x00,
    0xc0, 0x57, 0x00, 0x00,
    0xc0, 0x0d, 0x00, 0x00,
    0x46, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x02, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x43, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x75, 0x63,
    0x74, 0x69, 0x6f, 0x6e,
    0x44, 0x72, 0x6f, 0x69,
    0x64, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0xdc, 0x00, 0x00, 0x00,
    0xc0, 0x56, 0x00, 0x00,
    0xc0, 0x0c, 0x00, 0x00,
    0x46, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x02, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x43, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x75, 0x63,
    0x74, 0x69, 0x6f, 0x6e,
    0x44, 0x72, 0x6f, 0x69,
    0x64, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0xdd, 0x00, 0x00, 0x00,
    0xc0, 0x56, 0x00, 0x00,
    0x40, 0x0c, 0x00, 0x00,
    0x46, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x02, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x43, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x75, 0x63,
    0x74, 0x69, 0x6f, 0x6e,
    0x44, 0x72, 0x6f, 0x69,
    0x64, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0xdf, 0x00, 0x00, 0x00,
    0x40, 0x5a, 0x00, 0x00,
    0x40, 0x31, 0x00, 0x00,
    0x46, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x03, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x43, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x75, 0x63,
    0x74, 0x69, 0x6f, 0x6e,
    0x44, 0x72, 0x6f, 0x69,
    0x64, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0xe0, 0x00, 0x00, 0x00,
    0x40, 0x5a, 0x00, 0x00,
    0xc0, 0x31, 0x00, 0x00,
    0x46, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x03, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x43, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x75, 0x63,
    0x74, 0x69, 0x6f, 0x6e,
    0x44, 0x72, 0x6f, 0x69,
    0x64, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0xe1, 0x00, 0x00, 0x00,
    0x40, 0x5a, 0x00, 0x00,
    0x40, 0x32, 0x00, 0x00,
    0x46, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x03, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x43, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x75, 0x63,
    0x74, 0x69, 0x6f, 0x6e,
    0x44, 0x72, 0x6f, 0x69,
    0x64, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0xe2, 0x00, 0x00, 0x00,
    0x40, 0x5a, 0x00, 0x00,
    0xc0, 0x32, 0x00, 0x00,
    0x46, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x03, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x43, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x75, 0x63,
    0x74, 0x69, 0x6f, 0x6e,
    0x44, 0x72, 0x6f, 0x69,
    0x64, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0xe4, 0x00, 0x00, 0x00,
    0xc0, 0x09, 0x00, 0x00,
    0x40, 0x31, 0x00, 0x00,
    0x46, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x04, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x43, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x75, 0x63,
    0x74, 0x69, 0x6f, 0x6e,
    0x44, 0x72, 0x6f, 0x69,
    0x64, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0xe5, 0x00, 0x00, 0x00,
    0xc0, 0x09, 0x00, 0x00,
    0xc0, 0x31, 0x00, 0x00,
    0x46, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x04, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x43, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x75, 0x63,
    0x74, 0x69, 0x6f, 0x6e,
    0x44, 0x72, 0x6f, 0x69,
    0x64, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0xe6, 0x00, 0x00, 0x00,
    0xc0, 0x09, 0x00, 0x00,
    0x40, 0x32, 0x00, 0x00,
    0x46, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x04, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x43, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x75, 0x63,
    0x74, 0x69, 0x6f, 0x6e,
    0x44, 0x72, 0x6f, 0x69,
    0x64, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0xe7, 0x00, 0x00, 0x00,
    0xc0, 0x09, 0x00, 0x00,
    0xc0, 0x32, 0x00, 0x00,
    0x46, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x04, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x43, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x75, 0x63,
    0x74, 0x69, 0x6f, 0x6e,
    0x44, 0x72, 0x6f, 0x69,
    0x64, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0xe9, 0x00, 0x00, 0x00,
    0x40, 0x0c, 0x00, 0x00,
    0xc0, 0x56, 0x00, 0x00,
    0x46, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x05, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x43, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x75, 0x63,
    0x74, 0x69, 0x6f, 0x6e,
    0x44, 0x72, 0x6f, 0x69,
    0x64, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0xea, 0x00, 0x00, 0x00,
    0xc0, 0x0c, 0x00, 0x00,
    0xc0, 0x56, 0x00, 0x00,
    0x46, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x05, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x43, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x75, 0x63,
    0x74, 0x69, 0x6f, 0x6e,
    0x44, 0x72, 0x6f, 0x69,
    0x64, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0xeb, 0x00, 0x00, 0x00,
    0xc0, 0x0d, 0x00, 0x00,
    0xc0, 0x57, 0x00, 0x00,
    0x46, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x05, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x43, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x75, 0x63,
    0x74, 0x69, 0x6f, 0x6e,
    0x44, 0x72, 0x6f, 0x69,
    0x64, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0xec, 0x00, 0x00, 0x00,
    0xc0, 0x0d, 0x00, 0x00,
    0x40, 0x58, 0x00, 0x00,
    0x46, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x05, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x43, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x75, 0x63,
    0x74, 0x69, 0x6f, 0x6e,
    0x44, 0x72, 0x6f, 0x69,
    0x64, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0xee, 0x00, 0x00, 0x00,
    0x40, 0x31, 0x00, 0x00,
    0x40, 0x5a, 0x00, 0x00,
    0x46, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x06, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x43, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x75, 0x63,
    0x74, 0x69, 0x6f, 0x6e,
    0x44, 0x72, 0x6f, 0x69,
    0x64, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0xef, 0x00, 0x00, 0x00,
    0xc0, 0x31, 0x00, 0x00,
    0x40, 0x5a, 0x00, 0x00,
    0x46, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x06, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x43, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x75, 0x63,
    0x74, 0x69, 0x6f, 0x6e,
    0x44, 0x72, 0x6f, 0x69,
    0x64, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0xf0, 0x00, 0x00, 0x00,
    0x40, 0x32, 0x00, 0x00,
    0x40, 0x5a, 0x00, 0x00,
    0x46, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x06, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x43, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x75, 0x63,
    0x74, 0x69, 0x6f, 0x6e,
    0x44, 0x72, 0x6f, 0x69,
    0x64, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0xf1, 0x00, 0x00, 0x00,
    0xc0, 0x32, 0x00, 0x00,
    0x40, 0x5a, 0x00, 0x00,
    0x46, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x06, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x43, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x75, 0x63,
    0x74, 0x69, 0x6f, 0x6e,
    0x44, 0x72, 0x6f, 0x69,
    0x64, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0xf3, 0x00, 0x00, 0x00,
    0x40, 0x58, 0x00, 0x00,
    0xc0, 0x56, 0x00, 0x00,
    0x46, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x07, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x43, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x75, 0x63,
    0x74, 0x69, 0x6f, 0x6e,
    0x44, 0x72, 0x6f, 0x69,
    0x64, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0xf4, 0x00, 0x00, 0x00,
    0xc0, 0x57, 0x00, 0x00,
    0xc0, 0x56, 0x00, 0x00,
    0x46, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x07, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x43, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x75, 0x63,
    0x74, 0x69, 0x6f, 0x6e,
    0x44, 0x72, 0x6f, 0x69,
    0x64, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0xf5, 0x00, 0x00, 0x00,
    0xc0, 0x56, 0x00, 0x00,
    0xc0, 0x57, 0x00, 0x00,
    0x46, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x07, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x43, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x75, 0x63,
    0x74, 0x69, 0x6f, 0x6e,
    0x44, 0x72, 0x6f, 0x69,
    0x64, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0xf6, 0x00, 0x00, 0x00,
    0xc0, 0x56, 0x00, 0x00,
    0x40, 0x58, 0x00, 0x00,
    0x46, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x07, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00
  ];

  let magic = get_str_from_bytes(&bytes, 0, 4);
  let droid_version = get_u32_from_bytes(&bytes, 4);
  let num_droids = get_u32_from_bytes(&bytes, 8);

  let mut dinit = Dinit {
    magic: String::from(magic),
    droid_version: droid_version,
    num_droids: num_droids,
    droids: Vec::with_capacity(num_droids.try_into().unwrap()),
  };

  for i in 0..num_droids {
    let offset = 12 + 76 * (i as usize);

    let droid = Droid {
      name: String::from(get_str_from_bytes(&bytes, offset, 40)),
      id: get_u32_from_bytes(&bytes, offset + 40),
      coordinate: Coordinate {
        x: get_u32_from_bytes(&bytes, offset + 44),
        y: get_u32_from_bytes(&bytes, offset + 48),
        z: get_u32_from_bytes(&bytes, offset + 52)
      },
      direction: get_u32_from_bytes(&bytes, offset + 56),
      player: get_u32_from_bytes(&bytes, offset + 60),
      _dummy_in_fire: get_u32_from_bytes(&bytes, offset + 64),
      _dummy_burn_start: get_u32_from_bytes(&bytes, offset + 68),
      _dummy_burn_damage: get_u32_from_bytes(&bytes, offset + 72),
    };
    dinit.droids.push(droid);
  }

  println!("dinit: {:?}", dinit);

  // println!("num_droids: {:?} | {:?}", dinit.num_droids, dinit.droids.len());

//     let dec = Decoder {

//     };
//     let target: Option<&str>  = Some(MAGIC);

//     let encoded: Vec<u8>        = serialize(&target).unwrap();
//     let decoded: Option<String> = deserialize(&encoded[..]).unwrap();
// //    print!("{:?}", encoded);
// //    print!("{:?}", decoded);
//     assert_eq!(target.unwrap(), decoded.unwrap());

    // let data: [u8; 16] = [0x50, 0x53, 0x4d, 0x50, 0x06, 0x00, 0x00, 0x00, 0x84, 0x06, 0x0a, 0x00, 0x10, 0x00, 0x00, 0x00];
    // let psmp_bytes: &[u8] = &data[0..4];
    // let psmp_string: String = String::from_utf8(psmp_bytes.to_vec()).unwrap();
    // let psmp_back: &[u8] = psmp_string.as_bytes();
    // let slice: &[u8] = &data[4..8];
    // let version: u32 = u32::from_le_bytes(slice);
    // print!("{:?}", psmp_bytes);
//     print!("{:?}", psmp_string);
//     print!("{:?}", psmp_back);
//     print!("{:?}", version);

//    let map_size = 7;
//
//    let string_data = StringData {
//        length: 6,
//        data: String::from("abcdef"),
//    };
//
//    let tile = Tile {
//        texture1: 2,
//        texture2: 3,
//        priority: 0,
//    };
//
//    let patch = Patch {
//        tiles: vec![tile; (TILES_PER_PATCH_SIDE * TILES_PER_PATCH_SIDE) as usize],
//    };
//
//    let pmp = PMP {
//        magic: MAGIC.to_string(),
//        version: VERSION,
//        data_size: 123456,
//        map_size: map_size,
//        height_map: vec![1, ((map_size * TILES_PER_PATCH_SIDE - 1) * (map_size * TILES_PER_PATCH_SIDE - 1)) as u16],
//        num_terrain_textures: 4,
//        terrain_textures: vec![string_data; 4],
//        patches: vec![patch; (map_size * map_size) as usize],
//    };
//
////    println!("{:?}", pmp);
//
//    loadPmpFile("../../maps/Sporades Islands (2).pmp");
}

//fn loadPmpFile(filepath: &str) {
//    let f = File::open(filepath)?;
//    let mut reader = BufReader::new(f);
//}
//
//#[derive(Clone, Debug)]
//struct PMP {
//    magic: String, // "PSMP"
//    version: u32, // 6
//    data_size: u32,
//    map_size: u32,
//    height_map: Vec<u16>,
//    num_terrain_textures: u32,
//    terrain_textures: Vec<StringData>,
//    patches: Vec<Patch>,
//}
//
//#[derive(Clone, Debug)]
//struct Patch {
//    tiles: Vec<Tile>,
//}
//
//#[derive(Clone, Debug)]
//struct Tile {
//    texture1: u16,
//    texture2: u16,
//    priority: u32,
//}
//
//#[derive(Clone, Debug)]
//struct StringData {
//    length: u32,
//    data: String,
//}
