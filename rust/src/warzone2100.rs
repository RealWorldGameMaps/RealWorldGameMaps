use std::convert::TryInto;

#[path="file_reader.rs"] mod file_reader;
use file_reader::FileReader;

#[derive(Debug)]
pub struct TType {
  magic: String, // "ttyp"
  terrain_version: u32,
  num_terrain_types: u32,
  terrain_types: Vec<u8>,
}

#[derive(Debug)]
pub struct Struct {
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

  visibility: [u8; 8], // if (struct_version >= 14)
  research_name: String, // [char; if struct_version <= 19 { 40 } else { 60 }], // if (struct_version >= 15)
  _dummy_dummy_3: i16, // if (struct_version >= 17)
  _dummy_structure_padding_7: i16, // if (struct_version >= 15)
  _dummy_dummy_4: u32, // if (struct_version >= 21)
}

#[derive(Debug)]
pub struct Feat {
  magic: String, // "feat"
  feat_version: u32,
  num_features: u32,
  features: Vec<Feature>,
}
#[derive(Debug)]
struct Feature {
  name: String, // [char; if feat_version <= 19 { 40 } else { 60 }],
  id: u32,
  coordinate: Coordinate,
  direction: u32,
  player: u32,
  _dummy_in_fire: u32,
  _dummy_burn_start: u32,
  _dummy_burn_damage: u32,

  visibility: [u8; 8], // if (feat_version >= 14)
}
#[derive(Debug)]
struct Coordinate {
  x: u32,
  y: u32,
  z: u32,
}

#[derive(Debug)]
pub struct Map {
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
struct Other {
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
  let mut file_reader = FileReader::new(filepath);

  let magic = String::from(file_reader.read_str(0, 4));
  let game_version = file_reader.read_u32(4);

  if game_version > 35 { // big-endian
    file_reader.little_endian = false;
  }

  let mut game = Game {
    magic,
    game_version,
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


pub fn parse_feat_file(filepath: &str) -> Feat {
  let file_reader = FileReader::new(filepath);
  
  let num_features = file_reader.read_u32(8);

  let mut feat = Feat {
    magic: String::from(file_reader.read_str(0, 4)),
    feat_version: file_reader.read_u32(4),
    num_features,
    features: Vec::with_capacity(num_features as usize),
  };
  assert_eq!(feat.magic, "feat");

  let name_length = if feat.feat_version <= 19 { 40 } else { 60 };
  let feature_size = if feat.feat_version >= 14 { name_length + 44 } else { name_length + 36 };
  for i in 0..num_features {
    
    let offset = 12 + feature_size * (i as usize);

    let name = String::from(file_reader.read_str(offset, name_length));

    let visibility = if feat.feat_version >= 14 {
      file_reader.read_bytes(offset + name_length + 36, 8).try_into().unwrap()
    } else {
      [0, 0, 0, 0, 0, 0, 0, 0]
    };

    let feature = Feature {
      name,
      id: file_reader.read_u32(offset + name_length),
      coordinate: Coordinate {
        x: file_reader.read_u32(offset + name_length + 4),
        y: file_reader.read_u32(offset + name_length + 8),
        z: file_reader.read_u32(offset + name_length + 12),
      },
      direction: file_reader.read_u32(offset + name_length + 16),
      player: file_reader.read_u32(offset + name_length + 20),
      _dummy_in_fire: file_reader.read_u32(offset + name_length + 24),
      _dummy_burn_start: file_reader.read_u32(offset + name_length + 28),
      _dummy_burn_damage: file_reader.read_u32(offset + name_length + 32),
      visibility,
    };

    feat.features.push(feature);
  }

  feat
}

pub fn parse_ttypes_file(filepath: &str) -> TType {
  let file_reader = FileReader::new(filepath);

  let mut num_terrain_types = file_reader.read_u32(8);

  const MAX_TILE_TEXTURES: u32 = 255;
  if num_terrain_types >= MAX_TILE_TEXTURES {
    num_terrain_types = MAX_TILE_TEXTURES - 1;
  }

  let mut ttype = TType {
    magic: String::from(file_reader.read_str(0, 4)),
    terrain_version: file_reader.read_u32(4),
    num_terrain_types,
    terrain_types: Vec::with_capacity(num_terrain_types as usize),
  };
  assert_eq!(ttype.magic, "ttyp");

  for i in 0..num_terrain_types {
    let offset = 12 + 2 * (i as usize);
    ttype.terrain_types.push(file_reader.read_u16(offset) as u8); 
  }

  ttype
}

pub fn parse_struct_file(filepath: &str) -> Struct {
  let file_reader = FileReader::new(filepath);

  let num_structures = file_reader.read_u32(8);

  let mut struct_obj = Struct {
    magic: String::from(file_reader.read_str(0, 4)),
    struct_version: file_reader.read_u32(4),
    num_structures,
    structures: Vec::with_capacity(num_structures as usize),
  };
  assert_eq!(struct_obj.magic, "stru");

  let mut offset = 12;
  for _i in 0..num_structures {
    let name_length = if struct_obj.struct_version <= 19 { 40 } else { 60 };

    let name = String::from(file_reader.read_str(offset, name_length));
    let id = file_reader.read_u32(offset + name_length);
    let coordinate = Coordinate {
      x: file_reader.read_u32(offset + name_length + 4),
      y: file_reader.read_u32(offset + name_length + 8),
      z: file_reader.read_u32(offset + name_length + 12),
    };
    let direction = file_reader.read_u32(offset + name_length + 16);
    let player = file_reader.read_u32(offset + name_length + 20);
    let dummy_in_fire = file_reader.read_u32(offset + name_length + 24);
    let dummy_burn_start = file_reader.read_u32(offset + name_length + 28);
    let dummy_burn_damage = file_reader.read_u32(offset + name_length + 32);
    let dummy_status = file_reader.read_u8(offset + name_length + 36);
    let dummy_structure_padding_1 = file_reader.read_u8(offset + name_length + 37);
    let dummy_structure_padding_2 = file_reader.read_u8(offset + name_length + 38);
    let dummy_structure_padding_3 = file_reader.read_u8(offset + name_length + 39);
    let dummy_current_build_pts = file_reader.read_i32(offset + name_length + 40);
    let dummy_body = file_reader.read_u32(offset + name_length + 44);
    let dummy_armour = file_reader.read_u32(offset + name_length + 48);
    let dummy_resistance = file_reader.read_u32(offset + name_length + 52);
    let dummy_dummy_1 = file_reader.read_u32(offset + name_length + 56);
    let dummy_subject_inc = file_reader.read_u32(offset + name_length + 60);
    let dummy_time_started = file_reader.read_u32(offset + name_length + 64);
    let dummy_output = file_reader.read_u32(offset + name_length + 68);
    let dummy_capacity = file_reader.read_u32(offset + name_length + 72);
    let dummy_quantity = file_reader.read_u32(offset + name_length + 76);
    offset += name_length + 80;

    let greater_eq_12 = struct_obj.struct_version >= 12;  
    let dummy_factory_inc = if greater_eq_12 { file_reader.read_u32(offset) } else { 0 };
    let dummy_loops_performed = if greater_eq_12 { file_reader.read_u8(offset + 4) } else { 0 };
    let dummy_structure_padding_4 = if greater_eq_12 { file_reader.read_u8(offset + 5) } else { 0 };
    let dummy_structure_padding_5 = if greater_eq_12 { file_reader.read_u8(offset + 6) } else { 0 };
    let dummy_structure_padding_6 = if greater_eq_12 { file_reader.read_u8(offset + 7) } else { 0 };
    let dummy_power_accrued = if greater_eq_12 { file_reader.read_u32(offset + 8) } else { 0 };
    let dummy_dummy_2 = if greater_eq_12 { file_reader.read_u32(offset + 12) } else { 0 };
    let dummy_droid_time_started = if greater_eq_12 { file_reader.read_u32(offset + 16) } else { 0 };
    let dummy_time_to_build = if greater_eq_12 { file_reader.read_u32(offset + 20) } else { 0 };
    let dummy_time_start_hold = if greater_eq_12 { file_reader.read_u32(offset + 24) } else { 0 };
    if greater_eq_12 {
      offset += 28;
    }

    let visibility = if struct_obj.struct_version >= 14 {
      let value = file_reader.read_bytes(offset, 8).try_into().unwrap();
      offset += 8;
      value
    } else {
      [0, 0, 0, 0, 0, 0, 0, 0]
    };

    let research_name = if struct_obj.struct_version >= 15 {
      let value = String::from(file_reader.read_str(offset, name_length));
      offset += name_length;
      value
    } else {
      String::new()
    };

    let dummy_dummy_3 = if struct_obj.struct_version >= 17 {
      let value = file_reader.read_i16(offset);
      offset += 2;
      value
    } else {
      0
    };

    let dummy_structure_padding_7 = if struct_obj.struct_version >= 15 {
      let value = file_reader.read_i16(offset);
      offset += 2;
      value
    } else {
      0
    };

    let dummy_dummy_4 = if struct_obj.struct_version >= 21 {
      let value = file_reader.read_u32(offset);
      offset += 4;
      value
    } else {
      0
    };

    let structure = Structure {
      name, id, coordinate, direction, player,
      _dummy_in_fire: dummy_in_fire,
      _dummy_burn_start: dummy_burn_start,
      _dummy_burn_damage: dummy_burn_damage,
      _dummy_status: dummy_status,
      _dummy_structure_padding_1: dummy_structure_padding_1,
      _dummy_structure_padding_2: dummy_structure_padding_2,
      _dummy_structure_padding_3: dummy_structure_padding_3,
      _dummy_current_build_pts: dummy_current_build_pts,
      _dummy_body: dummy_body,
      _dummy_armour: dummy_armour,
      _dummy_resistance: dummy_resistance,
      _dummy_dummy_1: dummy_dummy_1,
      _dummy_subject_inc: dummy_subject_inc,
      _dummy_time_started: dummy_time_started,
      _dummy_output: dummy_output,
      _dummy_capacity: dummy_capacity,
      _dummy_quantity: dummy_quantity,
      _dummy_factory_inc: dummy_factory_inc,
      _dummy_loops_performed: dummy_loops_performed,
      _dummy_structure_padding_4: dummy_structure_padding_4,
      _dummy_structure_padding_5: dummy_structure_padding_5,
      _dummy_structure_padding_6: dummy_structure_padding_6,
      _dummy_power_accrued: dummy_power_accrued,
      _dummy_dummy_2: dummy_dummy_2,
      _dummy_droid_time_started: dummy_droid_time_started,
      _dummy_time_to_build: dummy_time_to_build,
      _dummy_time_start_hold: dummy_time_start_hold,
      visibility, research_name,
      _dummy_dummy_3: dummy_dummy_3,
      _dummy_structure_padding_7: dummy_structure_padding_7,
      _dummy_dummy_4: dummy_dummy_4,
    };

    struct_obj.structures.push(structure);
  }

  struct_obj
}

pub fn parse_map_file(filepath: &str) -> Map {
  let file_reader = FileReader::new(filepath);

  let mut offset = 0;

  let magic = String::from(file_reader.read_str(offset, 4));
  let map_version = file_reader.read_u32(offset + 4);
  let width = file_reader.read_u32(offset + 8);
  let height = file_reader.read_u32(offset + 12);
  offset += 16;

  assert_eq!(magic, "map ");
  let area = width * height;

  let mut tiles = Vec::with_capacity(area as usize);
  for _i in 0..area {
    tiles.push(Tile {
      texture: file_reader.read_u16(offset),
      height: file_reader.read_u8(offset + 2),
    });

    /*
    for (j = 0; j < MAX_PLAYERS; j++) {
			map->mMapTiles[i].tileVisBits = (uint8_t)(map->mMapTiles[i].tileVisBits &~ (uint8_t)(1 << j));
		}
    */
    offset += 3;
  }

  let gw_version = file_reader.read_u32(offset);
  let num_gateways = file_reader.read_u32(offset + 4);
  offset += 8;

  let mut gateways = Vec::with_capacity(num_gateways as usize);

  for _i in 0..num_gateways {
    let gateway = Gateway {
      x1: file_reader.read_u8(offset),
      y1: file_reader.read_u8(offset + 1),
      x2: file_reader.read_u8(offset + 2),
      y2: file_reader.read_u8(offset + 3),
    };
    gateways.push(gateway);
    offset +=  4;
  }

  let map = Map {
    magic, map_version, width, height, tiles, gw_version, num_gateways, gateways
  };

  map
}