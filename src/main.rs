use std::{env, fs, string::String};

fn main() {
	let args: Vec<String> = env::args().collect();
	let config = parse_config(&args);
	let contents = read_file(&config.filename);
	println!("Searching for '{}' in file '{}'", config.query, config.filename);
	println!("\nFile contains the following text:\n\n{}", contents);
}

struct Config {
	query: String,
	filename: String,
}

fn parse_config(args: &[String]) -> Config {
	let query = args[1].clone();
	let filename = args[2].clone();
	Config { query, filename }
}

fn read_file(filename: &String) -> String {
	fs::read_to_string(filename)
		.expect("Something went wrong when reading the file")
}
