mod warzone2100;

fn main() {
	// load map from file into Warzone2100Map struct
	let map_reader = warzone2100::MapReader::new("../data/warzone2100/8c-Mero_SquaredV11.wz");
	let warzone2100_map = map_reader.read();

	// save Warzone2100Map struct back to file
	let map_writer = warzone2100::MapWriter::new("../data/warzone2100/generated.wz");
	map_writer.write(&warzone2100_map, "8c-Mero_SquaredV11");

	// load in the saved file into a Warzone2100Map struct
	let map_checker = warzone2100::MapReader::new("../data/warzone2100/generated.wz");
	let warzone2100_map_check = map_checker.read();

	// check if the two loaded structs match
	let formatted = format!("{:?}", warzone2100_map);
	let formatted_check = format!("{:?}", warzone2100_map_check);
	assert_eq!(formatted, formatted_check);
}
