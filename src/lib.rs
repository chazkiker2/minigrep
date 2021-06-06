//! # minigrep
//!
//! `minigrep` is a small CLI implementation of the `grep` command, written in Rust.
//!
//! This crate follows the tutorial laid out in [Chapter 12 of The Book]
//!
//! [Chapter 12 of The Book]: https://doc.rust-lang.org/book/ch12-00-an-io-project.html

use std::{env, error::Error, fs, result::Result, string::String};

/// A fully valid `minigrep` command. Consists of a `query` to search for, a `filename` to search within,
/// and a `case_sensitive` boolean to specify whether the search should ignore case or not
pub struct Config {
    /// the query to search for
    pub query: String,
    /// the file to search within
    pub filename: String,
    /// whether query should be case sensitive or not
    pub case_sensitive: bool,
}

impl Config {
    /// Attempt to create a new `Config` from given arguments.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::{env, process};
    /// use minigrep::Config;
    ///
    /// // try to parse CLI arguments into minigrep config;
    /// let config = Config::new(env::args()).unwrap_or_else(|err| {
    ///     // if fail: convey error and exit
    ///     eprintln!("Problem parsing arguments: {}", err);
    ///     process::exit(1);
    /// });
    /// ```
    ///
    /// # Errors
    ///
    /// - return `Err(string)` if a query string is missing from the args
    /// - return `Err(string)` if a file name is missing from the args
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

/// search through each line in given `contents` and return any lines containing a match to the `query`
///
/// # Arguments
///
/// - `query`: a query string to search for
/// - `contents`: the contents within which we should search for the given query
///
/// # Examples
///
/// ```
/// use minigrep::search;
///
/// let query = "duct"; // expect 'duct' in 'productive'
///
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.";
///
/// assert_eq!(vec!["safe, fast, productive."], search(query, contents));
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// search through each line in given `contents` and return any lines containing
/// a case-insensitive match to the `query`.
///
/// (i.e., query `"RuSt"` would match line `"rust"`)
///
/// # Arguments
///
/// - `query`: a query string to search for
/// - `contents`: the contents within which we should search for the given query
///
/// # Examples
/// ```
/// use minigrep::search_case_insensitive;
///
/// let query = "DUCT"; // 'DUCT' will match 'productive' b/c case-insensitive
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.";
///
/// assert_eq!(vec!["safe, fast, productive."], search_case_insensitive(query, contents));
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

/// Given a valid `config` of arguments, run the minigrep application.
/// This is the functional entry-point into the application.
///
/// # Examples
///
/// ```
/// use std::{env, process};
/// use minigrep::Config;
///
/// /// run `minigrep` CLI application
/// fn main() {
///     // try to parse CLI arguments into minigrep config;
///     let config = Config::new(env::args()).unwrap_or_else(|err| {
///         // if fail: convey error and exit
///         eprintln!("Problem parsing arguments: {}", err);
///         process::exit(1);
///     });
///
///     // try to run minigrep; convey error and exit if fail
///     if let Err(e) = minigrep::run(config) {
///         eprintln!("Application error: {}", e);
///         process::exit(1);
///     }
/// }
/// ```
///
/// # Errors
///
/// This function will return an IO error if `config.filename` does not exist.
/// Other errors may also be returned according to [`std::fs::read_to_string`]
///
/// [`std::fs::read_to_string`]: https://doc.rust-lang.org/std/fs/fn.read_to_string.html
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    println!(
        "Searching for '{}' in file '{}'\n",
        config.query, config.filename
    );

    if results.len() == 0 {
        println!("No lines matched your query.");
    } else {
        for line in results {
            println!("{}", line);
        }
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
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    /// test case-insensitive query with matching result
    #[test]
    fn case_insensitive_one_result() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
