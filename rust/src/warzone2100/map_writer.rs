use std::convert::TryInto;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

extern crate compress_tools;
use compress_tools::*;

extern crate tempfile;
use tempfile::TempDir;

use super::defs::{Dinit, Droid, Feat, Feature, Game, Gateway, Map, Struct, Structure, TType, Tile, Warzone2100Map};

pub struct MapWriter {
	filepath: &'static str,
}

impl MapWriter {
	pub fn new(filepath: &'static str) -> MapWriter {
		MapWriter { filepath }
	}

	pub fn write(&self, warzone2100_map: &Warzone2100Map, map_name: &str) {
		let tempdir = TempDir::new().unwrap();

		let mut path = PathBuf::new();
		path.push(tempdir.path().to_str().unwrap());

		let mut addon_file_name = String::from(map_name);
		addon_file_name.push_str(".addon.lev");
		let mut addon_file_path = path.clone();
		addon_file_path.push(addon_file_name);
		self.write_addon_file(addon_file_path.to_str().unwrap());

		path.push("multiplay");
		path.push("maps");

		let mut game_file_name = String::from(map_name);
		game_file_name.push_str(".gam");
		let mut game_file_path = path.clone();
		game_file_path.push(game_file_name);
		self.write_game_file(game_file_path.to_str().unwrap(), &warzone2100_map.game);

		path.push(map_name);

		let mut dinit_file_path = path.clone();
		dinit_file_path.push("dinit.bjo");
		self.write_dinit_file(dinit_file_path.to_str().unwrap(), &warzone2100_map.dinit);

		let mut feat_file_path = path.clone();
		feat_file_path.push("feat.bjo");
		self.write_feat_file(feat_file_path.to_str().unwrap(), &warzone2100_map.feat);

		let mut struct_file_path = path.clone();
		struct_file_path.push("struct.bjo");
		self.write_struct_file(struct_file_path.to_str().unwrap(), &warzone2100_map.struct_obj);

		let mut ttypes_file_path = path.clone();
		ttypes_file_path.push("ttypes.ttp");
		self.write_ttypes_file(ttypes_file_path.to_str().unwrap(), &warzone2100_map.ttype);
	}

	pub fn write_addon_file(&self, filepath: &str) {}

	pub fn write_game_file(&self, filepath: &str, game: &Game) {}

	pub fn write_dinit_file(&self, filepath: &str, dinit: &Dinit) {}

	pub fn write_map_file(&self, filepath: &str, map: &Map) {}

	pub fn write_struct_file(&self, filepath: &str, struct_obj: &Struct) {}

	pub fn write_feat_file(&self, filepath: &str, feat: &Feat) {}

	pub fn write_ttypes_file(&self, filepath: &str, ttype: &TType) {}
}
