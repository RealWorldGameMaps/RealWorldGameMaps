// extern crate bincode;
// extern crate tiff;

mod warzone2100;

// use bincode::{serialize, deserialize};
// use tiff::decoder::{Decoder};

//use std::io;
//use std::io::prelude::*;
//use std::fs::File;

// const MAGIC: &str = "PSMP";
// const VERSION: u32 = 6;
// const HEADER_BYTES: u32 = 12;
// const TILES_PER_PATCH_SIDE: u32 = 16;


fn main() {
  // Dinit
  /*let filename = "../data/warzone2100/8c-Mero_SquaredV11/multiplay/maps/8c-Mero_SquaredV11/dinit.bjo";
  let dinit = warzone2100::parse_dinit_file(filename);
  println!("{:?}", dinit);*/

  // Game
  /*let filepath = "../data/warzone2100/8c-Mero_SquaredV11/multiplay/maps/8c-Mero_SquaredV11.gam";
  let game = warzone2100::parse_game_file(filepath);
  println!("{:?}", game);*/

  // Feat
  /*let filepath = "../data/warzone2100/8c-Mero_SquaredV11/multiplay/maps/8c-Mero_SquaredV11/feat.bjo";
  let feat = warzone2100::parse_feat_file(filepath);
  println!("{:?}", feat);*/

  // TType
  /*let filepath = "../data/warzone2100/8c-Mero_SquaredV11/multiplay/maps/8c-Mero_SquaredV11/ttypes.ttp";
  let ttype = warzone2100::parse_ttypes_file(filepath);
  println!("{:?}", ttype);*/

  // Struct
  /*let filepath = "../data/warzone2100/8c-Mero_SquaredV11/multiplay/maps/8c-Mero_SquaredV11/struct.bjo";
  let struct_obj = warzone2100::parse_struct_file(filepath);
  println!("{:?}", struct_obj);*/

  // Map
  /*let filepath = "../data/warzone2100/8c-Mero_SquaredV11/multiplay/maps/8c-Mero_SquaredV11/game.map";
  let map = warzone2100::parse_map_file(filepath);
  println!("{:?}", map);*/

  let map_reader = warzone2100::MapReader::new("../data/warzone2100/8c-Mero_SquaredV11.wz");
  let warzone2100_map = map_reader.read();
  //println!("{:?}", warzone2100_map); // this line crashes the terminal inside of VS Code


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
