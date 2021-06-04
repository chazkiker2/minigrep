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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let mut results = Vec::new();

	for line in contents.lines() {
		if line.contains(query) {
			results.push(line);
		}
	}

	results
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(&config.filename)?;

	println!("Searching for '{}' in file '{}'\n", config.query, config.filename);

	for line in search(&config.query, &contents) {
		println!("{}", line);
	}

	Ok(())
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn no_result() {
		let query = "philanthropist";
		let contents = "no such thing";
		assert_eq!(vec![] as Vec<&str>, search(query, contents));
	}

	#[test]
	fn one_result() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.";

		assert_eq!(vec!["safe, fast, productive."], search(query, contents));
	}


}
