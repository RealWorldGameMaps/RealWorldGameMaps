use std::fs::File;
use std::io::Read;
use std::str;
use std::convert::TryInto;

pub fn get_str_from_bytes(bytes: &Vec<u8>, from: usize, max_length: usize) -> &str {
    let length = bytes[from .. from + max_length].iter().position(|&byte| byte == 0).unwrap_or(max_length);
    str::from_utf8(&(bytes[from .. from + length])).unwrap()
}

pub fn get_u32_from_bytes(bytes: &Vec<u8>, from: usize) -> u32 {
    u32::from_le_bytes((&(bytes[from .. from + 4])).try_into().unwrap())
}

pub fn read_file(filepath: &str) -> Vec<u8> {
    let mut file = File::open(&filepath).unwrap();
    let mut bytes = Vec::new();
    file.by_ref().read_to_end(&mut bytes).unwrap();

    bytes
}