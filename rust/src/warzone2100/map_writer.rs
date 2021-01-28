use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

extern crate zip;
use zip::{DateTime, write::{FileOptions, ZipWriter}};

#[path = "../file_writer.rs"]
mod file_writer;
use file_writer::FileWriter;

use super::defs::*;

pub struct MapWriter {
	filepath: &'static str,
}

impl MapWriter {
	pub fn new(filepath: &'static str) -> MapWriter {
		MapWriter { filepath }
	}

	pub fn write(&self, warzone2100_map: &Warzone2100Map, map_name: &str) {
		// get the struct data as u8-vectors
		let addon_buf = self.get_addon_file(&warzone2100_map.addon_file);
		let game_buf = self.get_game_file(&warzone2100_map.game);
		let dinit_buf = self.get_dinit_file(&warzone2100_map.dinit);
		let feat_buf = self.get_feat_file(&warzone2100_map.feat);
		let struct_buf = self.get_struct_file(&warzone2100_map.struct_obj);
		let ttypes_buf = self.get_ttypes_file(&warzone2100_map.ttype);
		let map_buf = self.get_map_file(&warzone2100_map.map);

		let file = File::create(self.filepath).unwrap();
		let mut zip = ZipWriter::new(file);

		// Write .addon.lev file
		let mut addon_file_name = map_name.to_owned();
		addon_file_name.push_str(".addon.lev");
		zip.start_file(addon_file_name, FileOptions::default()).unwrap();
		zip.write(&addon_buf[..]).unwrap();

		let mut path = PathBuf::new();
		path.push("multiplay");
		path.push("maps");

		// write .gam file
		let mut game_file_name = map_name.to_owned();
		game_file_name.push_str(".gam");
		let mut game_file_path = path.clone();
		game_file_path.push(game_file_name);
		zip.start_file(game_file_path.to_str().unwrap(), FileOptions::default()).unwrap();
		zip.write(&game_buf[..]).unwrap();

		// go into "8c-Mero_SquaredV11" folder
		path.push(map_name);

		// write dinit.bjo file
		let mut dinit_file_path = path.clone();
		dinit_file_path.push("dinit.bjo");
		zip.start_file(dinit_file_path.to_str().unwrap(), FileOptions::default()).unwrap();
		zip.write(&dinit_buf[..]).unwrap();

		// write feat.bjo file
		let mut feat_file_path = path.clone();
		feat_file_path.push("feat.bjo");
		zip.start_file(feat_file_path.to_str().unwrap(), FileOptions::default()).unwrap();
		zip.write(&feat_buf[..]).unwrap();

		// write game.map file
		let mut map_file_path = path.clone();
		map_file_path.push("game.map");
		zip.start_file(map_file_path.to_str().unwrap(), FileOptions::default()).unwrap();
		zip.write(&map_buf[..]).unwrap();

		// write struct.bjo file
		let mut struct_file_path = path.clone();
		struct_file_path.push("struct.bjo");
		zip.start_file(struct_file_path.to_str().unwrap(), FileOptions::default()).unwrap();
		zip.write(&struct_buf[..]).unwrap();

		// write ttypes.ttp file
		let mut ttypes_file_path = path.clone();
		ttypes_file_path.push("ttypes.ttp");
		zip.start_file(ttypes_file_path.to_str().unwrap(), FileOptions::default()).unwrap();
		zip.write(&ttypes_buf[..]).unwrap();
	}

	// TODO: implement actual behavior
	fn get_addon_file(&self, addon_details: &AddonDetails) -> Vec<u8> {
		let mut out = String::from(&addon_details.comment);

		for level in addon_details.levels.iter() {
			out.push_str("\n\nlevel ");
			out.push_str(&level.level);

			out.push_str("\nplayers ");
			out.push_str(&level.players.to_string());

			out.push_str("\ntype ");
			out.push_str(&level.type_num.to_string());

			out.push_str("\ndataset ");
			out.push_str(&level.dataset);

			out.push_str("\ngame ");
			out.push_str(&level.game);

			for data in level.data.iter() {
				out.push_str("\ndata ");
				out.push_str(&data);
			}
		}

		out.as_bytes().to_vec()
	}

	fn get_game_file(&self, game: &Game) -> Vec<u8> {
		let mut file_writer = FileWriter::new(0);
		file_writer.write_str(&game.magic[..], game.magic.len());
		file_writer.write_u32(game.game_version);
		if game.game_version > 35 {
			file_writer.little_endian = false;
		}

		file_writer.write_u32(game.game_time);
		file_writer.write_u32(game.game_type);
		file_writer.write_i32(game.scroll_min_x);
		file_writer.write_i32(game.scroll_min_y);
		file_writer.write_u32(game.scroll_max_x);
		file_writer.write_u32(game.scroll_max_y);
		file_writer.write_str(&game.level_name[..], 20);

		for other in game.other.iter() {
			file_writer.write_u32(other.power);
			file_writer.write_u32(other._dummy);
		}

		file_writer.bytes
	}

	fn get_dinit_file(&self, dinit: &Dinit) -> Vec<u8> {
		let mut file_writer = FileWriter::new(0);
		file_writer.write_str(&dinit.magic[..], dinit.magic.len());
		file_writer.write_u32(dinit.droid_version);
		file_writer.write_u32(dinit.num_droids);

		let name_length = if dinit.droid_version <= 19 { 40 } else { 60 };

		for droid in dinit.droids.iter() {
			file_writer.write_str(&droid.name[..], name_length);
			file_writer.write_u32(droid.id);
			file_writer.write_u32(droid.coordinate.x);
			file_writer.write_u32(droid.coordinate.y);
			file_writer.write_u32(droid.coordinate.z);
			file_writer.write_u32(droid.direction);
			file_writer.write_u32(droid.player);
			file_writer.write_u32(droid._dummy_in_fire);
			file_writer.write_u32(droid._dummy_burn_start);
			file_writer.write_u32(droid._dummy_burn_damage);
		}

		file_writer.bytes
	}

	fn get_map_file(&self, map: &Map) -> Vec<u8> {
		let mut file_writer = FileWriter::new(0);
		file_writer.write_str(&map.magic[..], map.magic.len());
		file_writer.write_u32(map.map_version);
		file_writer.write_u32(map.width);
		file_writer.write_u32(map.height);

		for tile in map.tiles.iter() {
			file_writer.write_u16(tile.texture);
			file_writer.write_u8(tile.height);
		}

		file_writer.write_u32(map.gw_version);
		file_writer.write_u32(map.num_gateways);

		for gateway in map.gateways.iter() {
			file_writer.write_u8(gateway.x1);
			file_writer.write_u8(gateway.y1);
			file_writer.write_u8(gateway.x2);
			file_writer.write_u8(gateway.y2);
		}

		file_writer.bytes
	}

	fn get_feat_file(&self, feat: &Feat) -> Vec<u8> {
		let mut file_writer = FileWriter::new(0);
		file_writer.write_str(&feat.magic[..], feat.magic.len());
		file_writer.write_u32(feat.feat_version);
		file_writer.write_u32(feat.num_features);

		let name_length = if feat.feat_version <= 19 { 40 } else { 60 };
		for feature in feat.features.iter() {
			file_writer.write_str(&feature.name[..], name_length);
			file_writer.write_u32(feature.id);
			file_writer.write_u32(feature.coordinate.x);
			file_writer.write_u32(feature.coordinate.y);
			file_writer.write_u32(feature.coordinate.z);
			file_writer.write_u32(feature.direction);
			file_writer.write_u32(feature.player);
			file_writer.write_u32(feature._dummy_in_fire);
			file_writer.write_u32(feature._dummy_burn_start);
			file_writer.write_u32(feature._dummy_burn_damage);
			if feat.feat_version >= 14 {
				file_writer.write_bytes(feature.visibility.to_vec());
			}
		}

		file_writer.bytes
	}

	fn get_ttypes_file(&self, ttype: &TType) -> Vec<u8> {
		let mut file_writer = FileWriter::new(0);
		file_writer.write_str(&ttype.magic[..], ttype.magic.len());
		file_writer.write_u32(ttype.terrain_version);
		file_writer.write_u32(ttype.num_terrain_types);

		for terrain_type in ttype.terrain_types.iter() {
			file_writer.write_u16(terrain_type.clone() as u16);
		}

		file_writer.bytes
	}

	fn get_struct_file(&self, struct_obj: &Struct) -> Vec<u8> {
		let mut file_writer = FileWriter::new(0);
		file_writer.write_str(&struct_obj.magic[..], struct_obj.magic.len());
		file_writer.write_u32(struct_obj.struct_version);
		file_writer.write_u32(struct_obj.num_structures);

		let name_length = if struct_obj.struct_version <= 19 { 40 } else { 60 };
		for structure in struct_obj.structures.iter() {
			file_writer.write_str(&structure.name[..], name_length);
			file_writer.write_u32(structure.id);
			file_writer.write_u32(structure.coordinate.x);
			file_writer.write_u32(structure.coordinate.y);
			file_writer.write_u32(structure.coordinate.z);
			file_writer.write_u32(structure.direction);
			file_writer.write_u32(structure.player);
			file_writer.write_u32(structure._dummy_in_fire);
			file_writer.write_u32(structure._dummy_burn_start);
			file_writer.write_u32(structure._dummy_burn_damage);
			file_writer.write_u8(structure._dummy_status);
			file_writer.write_u8(structure._dummy_structure_padding_1);
			file_writer.write_u8(structure._dummy_structure_padding_2);
			file_writer.write_u8(structure._dummy_structure_padding_3);
			file_writer.write_i32(structure._dummy_current_build_pts);
			file_writer.write_u32(structure._dummy_body);
			file_writer.write_u32(structure._dummy_armour);
			file_writer.write_u32(structure._dummy_resistance);
			file_writer.write_u32(structure._dummy_dummy_1);
			file_writer.write_u32(structure._dummy_subject_inc);
			file_writer.write_u32(structure._dummy_time_started);
			file_writer.write_u32(structure._dummy_output);
			file_writer.write_u32(structure._dummy_capacity);
			file_writer.write_u32(structure._dummy_quantity);

			if struct_obj.struct_version >= 12 {
				file_writer.write_u32(structure._dummy_factory_inc);
				file_writer.write_u8(structure._dummy_loops_performed);
				file_writer.write_u8(structure._dummy_structure_padding_4);
				file_writer.write_u8(structure._dummy_structure_padding_5);
				file_writer.write_u8(structure._dummy_structure_padding_6);
				file_writer.write_u32(structure._dummy_power_accrued);
				file_writer.write_u32(structure._dummy_dummy_2);
				file_writer.write_u32(structure._dummy_droid_time_started);
				file_writer.write_u32(structure._dummy_time_to_build);
				file_writer.write_u32(structure._dummy_time_start_hold);
			}

			if struct_obj.struct_version >= 14 {
				file_writer.write_bytes(structure.visibility.to_vec());
			}

			if struct_obj.struct_version >= 15 {
				file_writer.write_str(&structure.research_name[..], name_length);
			}

			if struct_obj.struct_version >= 17 {
				file_writer.write_i16(structure._dummy_dummy_3);
			}

			if struct_obj.struct_version >= 15 {
				file_writer.write_i16(structure._dummy_structure_padding_7);
			}

			if struct_obj.struct_version >= 21 {
				file_writer.write_u32(structure._dummy_dummy_4);
			}
		}

		file_writer.bytes
	}
}
