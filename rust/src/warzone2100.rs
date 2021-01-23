struct TType {
  magic: [char; 4], // "ttyp"
  terrain_version: u32,
  num_terrain_types: u32,
  terrain_types: [u16; num_terrain_types],
}

struct Struct {
  magic: [char; 4], // "stru"
  struct_version: u32,
  num_structures: u32,
  structures: [Structure; num_structures],
}
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

struct Feat {
  magic: [char; 4], // "feat"
  feat_version: u32,
  num_features: u32,
  features: [Feature; num_features],
}
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

struct Dinit {
  magic: [char; 4], // "dint"
  droid_version: u32,
  num_droids: u32,
  droids: [Droid; num_droids],
}
struct Droid {
  name: [char; if droid_version <= 19 { 40 } else { 60 }],
  id: u32,
  coordinate: Coordinate,
  direction: u32,
  player: u32,
  _dummy_in_fire: u32,
  _dummy_burn_start: u32,
  _dummy_burn_damage: u32,
}

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
struct Tile {
  texture: u16,
  height: u8,
}
struct Gateway {
  x1: u8,
  y1: u8,
  x2: u8,
  y2: u8,
}

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
struct Other {
  power: u32,
  _dummy: u32,
}


struct Coordinate {
  x: u32,
  y: u32,
  z: u32,
}
