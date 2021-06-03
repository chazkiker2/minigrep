use std::{env, error::Error, fs, process, result::Result, string::String};

fn main() {
	let args: Vec<String> = env::args().collect();
	let config = Config::new(&args).unwrap_or_else(|err| {
		println!("Problem parsing arguments: {}", err);
		process::exit(1);
	});
	if let Err(e) = run(config) {
		println!("Application error: {}", e);
		process::exit(1);
	}
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(&config.filename)?;
	println!("Searching for '{}' in file '{}'", config.query, config.filename);
	println!("\nFile contains the following text:\n\n{}", contents);
	Ok(())
}

struct Config {
	query: String,
	filename: String,
}

impl Config {
	fn new(args: &[String]) -> Result<Config, &str> {
		if args.len() < 3 {
			Err("not enough arguments")
		} else {
			let query = args[1].clone();
			let filename = args[2].clone();
			Ok(Config { query, filename })
		}
	}

	// fn read(&self) -> String {
	// 	fs::read_to_string(&self.filename)
	// 		.expect("Something went wrong when reading the file")
	// }
}
