use std::{error::Error, fs, result::Result, string::String};

pub struct Config {
	pub query: String,
	pub filename: String,
}

impl Config {
	pub fn new(args: &[String]) -> Result<Config, &str> {
		if args.len() < 3 {
			Err("not enough arguments")
		} else {
			let query = args[1].clone();
			let filename = args[2].clone();
			Ok(Config { query, filename })
		}
	}
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(&config.filename)?;
	println!("Searching for '{}' in file '{}'", config.query, config.filename);
	println!("\nFile contains the following text:\n\n{}", contents);
	Ok(())
}
