mod warzone2100;

mod file_writer;
use file_writer::FileWriter;

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

	//let map_reader = warzone2100::MapReader::new("../data/warzone2100/8c-Mero_SquaredV11.wz");
	//let warzone2100_map = map_reader.read();
	//println!("{:?}", warzone2100_map); // this line crashes the terminal inside of VS Code
	//println!("{:?}", warzone2100_map.game); // better use this line for testing

	/*
	let map_reader = warzone2100::MapReader::new("../data/warzone2100/8c-Mero_SquaredV11.wz");
	let warzone2100_map = map_reader.read();

	let map_writer = warzone2100::MapWriter::new("../data/warzone2100/generated/generated.wz");
	map_writer.write_dinit_file("../data/warzone2100/generated/dinit.bjo", &warzone2100_map.dinit);

	let map_reader2 = warzone2100::MapReader::new("not used");
	let dinit = map_reader2.parse_dinit_file("../data/warzone2100/generated/dinit.bjo");

	let dinit_str = format!("{:?}", dinit);
	let dinit2_str = format!("{:?}", warzone2100_map.dinit);
	assert_eq!(dinit_str, dinit2_str);
	*/

	let file_writer = FileWriter::new("../data/generated.txt", 100);
	file_writer.write_string(0, "hello world!", 12);

	// println!("num_droids: {:?} | {:?}", dinit.num_droids, dinit.droids.len());
}
