use std::convert::TryInto;

#[path="file_utils.rs"] mod file_utils;
use file_utils::get_str_from_bytes;
use file_utils::get_u32_from_bytes;

/*
#[derive(Debug)]
struct TType {
  magic: [char; 4], // "ttyp"
  terrain_version: u32,
  num_terrain_types: u32,
  terrain_types: [u16; num_terrain_types],
}

#[derive(Debug)]
struct Struct {
  magic: [char; 4], // "stru"
  struct_version: u32,
  num_structures: u32,
  structures: [Structure; num_structures],
}
#[derive(Debug)]
struct Structure {
  name: [char; if struct_version <= 19 { 40 } else { 60 }],
  id: u32,
  coordinate: Coordinate,
  direction: u32,
  player: u32,
  _dummy_in_fire: u32,
  _dummy_burn_start: u32,
  _dummy_burn_damage: u32,
  _dummy_status: u8,
  _dummy_structure_padding_1: u8,
  _dummy_structure_padding_2: u8,
  _dummy_structure_padding_3: u8,
  _dummy_current_build_pts: i32,
  _dummy_body: u32,
  _dummy_armour: u32,
  _dummy_resistance: u32,
  _dummy_dummy_1: u32,
  _dummy_subject_inc: u32,
  _dummy_time_started: u32,
  _dummy_output: u32,
  _dummy_capacity: u32,
  _dummy_quantity: u32,

  _dummy_factory_inc: u32, // if (struct_version >= 12)
  _dummy_loops_performed: u8, // if (struct_version >= 12)
  _dummy_structure_padding_4: u8, // if (struct_version >= 12)
  _dummy_structure_padding_5: u8, // if (struct_version >= 12)
  _dummy_structure_padding_6: u8, // if (struct_version >= 12)
  _dummy_power_accrued: u32, // if (struct_version >= 12)
  _dummy_dummy_2: u32, // if (struct_version >= 12)
  _dummy_droid_time_started: u32, // if (struct_version >= 12)
  _dummy_time_to_build: u32, // if (struct_version >= 12)
  _dummy_time_start_hold: u32, // if (struct_version >= 12)

  visibility: u32, // if (struct_version >= 14)
  research_name: [char; if struct_version <= 19 { 40 } else { 60 }], // if (struct_version >= 15)
  _dummy_dummy_3: i16, // if (struct_version >= 17)
  _dummy_structure_padding_7: i16, // if (struct_version >= 15)
  _dummy_dummy_4: u32, // if (struct_version >= 21)
}

#[derive(Debug)]
struct Feat {
  magic: [char; 4], // "feat"
  feat_version: u32,
  num_features: u32,
  features: [Feature; num_features],
}
#[derive(Debug)]
struct Feature {
  name: [char; if feat_version <= 19 { 40 } else { 60 }],
  id: u32,
  coordinate: Coordinate,
  direction: u32,
  player: u32,
  _dummy_in_fire: u32,
  _dummy_burn_start: u32,
  _dummy_burn_damage: u32,

  visibility: u32, // if (feat_version >= 14)
}



#[derive(Debug)]
struct Map {
  magic: [char; 4], // "map "
  map_version: u32,
  width: u32,
  height: u32,
  tiles: [Tile; width * height],
  gw_version: u32,
  num_gateways: u32,
  gateways: [Gateway; num_gateways],
}
#[derive(Debug)]
struct Tile {
  texture: u16,
  height: u8,
}
#[derive(Debug)]
struct Gateway {
  x1: u8,
  y1: u8,
  x2: u8,
  y2: u8,
}

#[derive(Debug)]
struct Game {
  magic: [char; 4], // "game"
  game_version: u32,
  game_time: u32,
  game_type: u32,
  scroll_min_x: i32,
  scroll_min_y: i32,
  scroll_max_x: u32,
  scroll_max_y: u32,
  level_name: [char; 20],
  other: [Other; 8],
}
#[derive(Debug)]
struct Other {
  power: u32,
  _dummy: u32,
}
*/

#[derive(Debug)]
pub struct Dinit {
  magic: String, // "dint"
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

pub fn parse_dinit_file(filepath: &str) -> Dinit {
  let bytes = file_utils::read_file(filepath);
  
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

  dinit
}