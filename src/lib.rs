use std::{
	env,
	error::Error,
	fs,
	result::Result,
	string::String,
};

/// a fully valid minigrep command
///
/// consists of a `query` to search for and a `filename` to search within
pub struct Config {
	/// the query to search for
	pub query: String,
	/// the file to search within
	pub filename: String,
	pub case_sensitive: bool,
}


impl Config {
	/// attempt to create a new config from given arguments
	pub fn new(args: &[String]) -> Result<Config, &str> {
		if args.len() < 3 {
			return Err("not enough arguments");
		}

		let query = args[1].clone();
		let filename = args[2].clone();

		let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

		Ok(Config { query, filename, case_sensitive })

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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();
	let mut results = Vec::new();
	for line in contents.lines() {
		if line.to_lowercase().contains(&query) {
			results.push(line);
		}
	}

	results
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(&config.filename)?;

	let results = if config.case_sensitive {
		search(&config.query, &contents)
	} else {
		search_case_insensitive(&config.query, &contents)
	};

	println!("Searching for '{}' in file '{}'\n", config.query, config.filename);

	for line in results {
		println!("{}", line);
	}

	Ok(())
}


#[cfg(test)]
mod tests {
	use super::*;


	/// test query with no matching lines
	#[test]
	fn case_sensitive_no_result() {
		let query = "philanthropist";
		let contents = "no such thing";
		assert_eq!(vec![] as Vec<&str>, search(query, contents));
	}


	/// test query with one matching line
	#[test]
	fn case_sensitive_one_result() {
		let query = "duct"; // expect 'duct' in 'productive'
		let contents = "\
Rust:
safe, fast, productive.
Pick three.";

		assert_eq!(vec!["safe, fast, productive."], search(query, contents));
	}

	/// test case-insensitive query with matching result
	#[test]
	fn case_insensitive_one_result() {
		let query = "rUsT";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

		assert_eq!(
			vec!["Rust:", "Trust me."],
			search_case_insensitive(query, contents)
		);
	}

}
