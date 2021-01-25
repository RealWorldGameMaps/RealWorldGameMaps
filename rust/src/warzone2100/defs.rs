

#[derive(Debug)]
pub struct TType {
  pub magic: String, // "ttyp"
  pub terrain_version: u32,
  pub num_terrain_types: u32,
  pub terrain_types: Vec<u8>,
}

#[derive(Debug)]
pub struct Struct {
  pub magic: String, // "stru"
  pub struct_version: u32,
  pub num_structures: u32,
  pub structures: Vec<Structure>,
}

#[derive(Debug)]
pub struct Structure {
  pub name: String, // [char; if struct_version <= 19 { 40 } else { 60 }],
  pub id: u32,
  pub coordinate: Coordinate,
  pub direction: u32,
  pub player: u32,
  pub _dummy_in_fire: u32,
  pub _dummy_burn_start: u32,
  pub _dummy_burn_damage: u32,
  pub _dummy_status: u8,
  pub _dummy_structure_padding_1: u8,
  pub _dummy_structure_padding_2: u8,
  pub _dummy_structure_padding_3: u8,
  pub _dummy_current_build_pts: i32,
  pub _dummy_body: u32,
  pub _dummy_armour: u32,
  pub _dummy_resistance: u32,
  pub _dummy_dummy_1: u32,
  pub _dummy_subject_inc: u32,
  pub _dummy_time_started: u32,
  pub _dummy_output: u32,
  pub _dummy_capacity: u32,
  pub _dummy_quantity: u32,
  
  pub _dummy_factory_inc: u32, // if (struct_version >= 12)
  pub _dummy_loops_performed: u8, // if (struct_version >= 12)
  pub _dummy_structure_padding_4: u8, // if (struct_version >= 12)
  pub _dummy_structure_padding_5: u8, // if (struct_version >= 12)
  pub _dummy_structure_padding_6: u8, // if (struct_version >= 12)
  pub _dummy_power_accrued: u32, // if (struct_version >= 12)
  pub _dummy_dummy_2: u32, // if (struct_version >= 12)
  pub _dummy_droid_time_started: u32, // if (struct_version >= 12)
  pub _dummy_time_to_build: u32, // if (struct_version >= 12)
  pub _dummy_time_start_hold: u32, // if (struct_version >= 12)
  
  pub visibility: [u8; 8], // if (struct_version >= 14)
  pub research_name: String, // [char; if struct_version <= 19 { 40 } else { 60 }], // if (struct_version >= 15)
  pub _dummy_dummy_3: i16, // if (struct_version >= 17)
  pub _dummy_structure_padding_7: i16, // if (struct_version >= 15)
  pub _dummy_dummy_4: u32, // if (struct_version >= 21)
}

#[derive(Debug)]
pub struct Feat {
  pub magic: String, // "feat"
  pub feat_version: u32,
  pub num_features: u32,
  pub features: Vec<Feature>,
}

#[derive(Debug)]
pub struct Feature {
  pub name: String, // [char; if feat_version <= 19 { 40 } else { 60 }],
  pub id: u32,
  pub coordinate: Coordinate,
  pub direction: u32,
  pub player: u32,
  pub _dummy_in_fire: u32,
  pub _dummy_burn_start: u32,
  pub _dummy_burn_damage: u32,
  
  pub visibility: [u8; 8], // if (feat_version >= 14)
}

#[derive(Debug)]
pub struct Coordinate {
  pub x: u32,
  pub y: u32,
  pub z: u32,
}

#[derive(Debug)]
pub struct Map {
  pub magic: String, // "map "
  pub map_version: u32,
  pub width: u32,
  pub height: u32,
  pub tiles: Vec<Tile>, // [Tile; width * height],
  pub gw_version: u32,
  pub num_gateways: u32,
  pub gateways: Vec<Gateway>, // [Gateway; num_gateways],
}

#[derive(Debug)]
pub struct Tile {
  pub texture: u16,
  pub height: u8,
}

#[derive(Debug)]
pub struct Gateway {
  pub x1: u8,
  pub y1: u8,
  pub x2: u8,
  pub y2: u8,
}

#[derive(Debug)]
pub struct Dinit {
  pub magic: String, // "dint"
  pub droid_version: u32,
  pub num_droids: u32,
  pub droids: Vec<Droid>,
}

#[derive(Debug)]
pub struct Droid {
  pub name: String,
  pub id: u32,
  pub coordinate: Coordinate,
  pub direction: u32,
  pub player: u32,
  pub _dummy_in_fire: u32,
  pub _dummy_burn_start: u32,
  pub _dummy_burn_damage: u32,
}

#[derive(Debug)]
pub struct Game {
  pub magic: String, // "game"
  pub game_version: u32,
  pub game_time: u32,
  pub game_type: u32,
  pub scroll_min_x: i32,
  pub scroll_min_y: i32,
  pub scroll_max_x: u32,
  pub scroll_max_y: u32,
  pub level_name: String,
  pub other: Vec<Other>, // constant size of 8, can we make this of type [Other; 8] ??
}

#[derive(Debug)]
pub struct Other {
  pub power: u32,
  pub _dummy: u32,
}

#[derive(Debug)]
pub struct Warzone2100Map {
  pub dinit: Dinit,
  pub feat: Feat,
  pub struct_obj: Struct,
  pub game: Game,
  pub map: Map,
  pub ttype: TType,
}