use std::{env, fs};

fn main() {
	let args: Vec<String> = env::args().collect();
	let query = &args[1];
	let filename = &args[2];

	let contents = fs::read_to_string(filename)
		.expect("Something went wrong reading the file");

	println!("Searching for '{}' in file '{}'", query, filename);

	println!("File contains the following text:\n{}", contents);
}
