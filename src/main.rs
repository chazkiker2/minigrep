use std::{env, fs, string::String};

fn main() {
	let args: Vec<String> = env::args().collect();
	let config = Config::new(&args);
	let contents = config.read();
	println!("Searching for '{}' in file '{}'", config.query, config.filename);
	println!("\nFile contains the following text:\n\n{}", contents);
}

struct Config {
	query: String,
	filename: String,
}

impl Config {
	fn new(args: &[String]) -> Config {
		let query = args[1].clone();
		let filename = args[2].clone();
		Config { query, filename }
	}

	fn read(&self) -> String {
		fs::read_to_string(&self.filename)
			.expect("Something went wrong when reading the file")
	}
}
