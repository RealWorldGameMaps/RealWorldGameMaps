use std::convert::TryInto;

/*#[path="file_utils.rs"] mod file_utils;
use file_utils::get_str_from_bytes;
use file_utils::get_u32_from_bytes;*/

#[path="file_reader.rs"] mod file_reader;

use file_reader::FileReader;

#[derive(Debug)]
struct TType {
  magic: String, // "ttyp"
  terrain_version: u32,
  num_terrain_types: u32,
  terrain_types: Vec<u16>,
}

#[derive(Debug)]
struct Struct {
  magic: String, // "stru"
  struct_version: u32,
  num_structures: u32,
  structures: Vec<Structure>,
}

#[derive(Debug)]
struct Structure {
  name: String, // [char; if struct_version <= 19 { 40 } else { 60 }],
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
  research_name: Vec<char>, // [char; if struct_version <= 19 { 40 } else { 60 }], // if (struct_version >= 15)
  _dummy_dummy_3: i16, // if (struct_version >= 17)
  _dummy_structure_padding_7: i16, // if (struct_version >= 15)
  _dummy_dummy_4: u32, // if (struct_version >= 21)
}

#[derive(Debug)]
struct Feat {
  magic: String, // "feat"
  feat_version: u32,
  num_features: u32,
  features: Vec<Feature>,
}
#[derive(Debug)]
struct Feature {
  name: Vec<char>, // [char; if feat_version <= 19 { 40 } else { 60 }],
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
  magic: String, // "map "
  map_version: u32,
  width: u32,
  height: u32,
  tiles: Vec<Tile>, // [Tile; width * height],
  gw_version: u32,
  num_gateways: u32,
  gateways: Vec<Gateway>, // [Gateway; num_gateways],
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

#[derive(Debug)]
pub struct Game {
  magic: String, // "game"
  game_version: u32,
  game_time: u32,
  game_type: u32,
  scroll_min_x: i32,
  scroll_min_y: i32,
  scroll_max_x: u32,
  scroll_max_y: u32,
  level_name: String,
  other: Vec<Other>, // constant size of 8, can we make this of type [Other; 8] ??
}

#[derive(Debug)]
pub struct Other {
  power: u32,
  _dummy: u32,
}

pub fn parse_dinit_file(filepath: &str) -> Dinit {
  let file_reader = FileReader::new(filepath);
  
  let num_droids = file_reader.read_u32(8);

  let mut dinit = Dinit {
    magic: String::from(file_reader.read_str(0, 4)),
    droid_version: file_reader.read_u32(4),
    num_droids: num_droids,
    droids: Vec::with_capacity(num_droids.try_into().unwrap()),
  };
  assert_eq!(dinit.magic, "dint");

  for i in 0..dinit.num_droids {
    let offset = 12 + 76 * (i as usize);

    let droid = Droid {
      name: String::from(file_reader.read_str(offset, 40)),
      id: file_reader.read_u32(offset + 40),
      coordinate: Coordinate {
        x: file_reader.read_u32(offset + 44),
        y: file_reader.read_u32(offset + 48),
        z: file_reader.read_u32(offset + 52),
      },
      direction: file_reader.read_u32(offset + 56),
      player: file_reader.read_u32(offset + 60),
      _dummy_in_fire: file_reader.read_u32(offset + 64),
      _dummy_burn_start: file_reader.read_u32(offset + 68),
      _dummy_burn_damage: file_reader.read_u32(offset + 72),
    };
    dinit.droids.push(droid);
  }

  dinit
}

pub fn parse_game_file(filepath: &str) -> Game {
  let file_reader = FileReader::new(filepath);

  let mut game = Game {
    magic: String::from(file_reader.read_str(0, 4)),
    game_version: file_reader.read_u32(4),
    game_time: file_reader.read_u32(8),
    game_type: file_reader.read_u32(12),
    scroll_min_x: file_reader.read_i32(16),
    scroll_min_y: file_reader.read_i32(20),
    scroll_max_x: file_reader.read_u32(24),
    scroll_max_y: file_reader.read_u32(28),
    level_name: String::from(file_reader.read_str(32, 20)),
    other: Vec::with_capacity(8),
  };
  assert_eq!(game.magic, "game");

  for i in 0..8 {
    let offset = 32 + game.level_name.len() + 4 * (i as usize);

    if game.game_version >= 10 {
      // todo: handle dummy value
      game.other.push(Other {
        power: file_reader.read_u32(offset),
        _dummy: 0,
      });
    } else {
      game.other.push(Other {
        power: 0,
        _dummy: 0,
      });
    }
  }

  game
}