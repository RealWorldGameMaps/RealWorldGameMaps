mod warzone2100;

extern crate chrono;
use chrono::offset::Utc;

fn main() {
	// load map from file into Warzone2100Map struct
	let map_reader = warzone2100::MapReader::new("../data/warzone2100/8c-Mero_SquaredV11.wz");
	let mut warzone2100_map = map_reader.read();

	warzone2100_map.addon_file.comment.push_str("// Generated using RealWorldGameMaps (https://github.com/RealWorldGameMaps)\n");
	warzone2100_map.addon_file.comment.push_str("// Date: ");
	let datetime = format!("{}", Utc::now());
	warzone2100_map.addon_file.comment.push_str(&datetime);

	// save Warzone2100Map struct back to file
	let map_writer = warzone2100::MapWriter::new("../data/warzone2100/generated.wz");
	map_writer.write(&warzone2100_map, "8c-Mero_SquaredV11");
}
