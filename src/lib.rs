use std::{error::Error, fs, result::Result, string::String};

/// a fully valid minigrep command
///
/// consists of a `query` to search for and a `filename` to search within
pub struct Config {
	/// the query to search for
	pub query: String,
	/// the file to search within
	pub filename: String,
}


impl Config {
	/// attempt to create a new config from given arguments
	///
	/// Example:
	/// ```rust
	/// let query = String::from("search query");
	/// let filename = String::from("filename.txt");
	///
	/// let config = Config::new(&vec![query, filename]).unwrap();
	/// ```
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

/// search through each line in given `contents` and return any lines containing a match to the `query`
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


	/// test query with no matching lines
	#[test]
	fn no_result() {
		let query = "philanthropist";
		let contents = "no such thing";
		assert_eq!(vec![] as Vec<&str>, search(query, contents));
	}


	/// test query with one matching line
	#[test]
	fn one_result() {
		let query = "duct"; // expect 'duct' in 'productive'
		let contents = "\
Rust:
safe, fast, productive.
Pick three.";

		assert_eq!(vec!["safe, fast, productive."], search(query, contents));
	}


}
