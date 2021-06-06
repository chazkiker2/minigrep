use std::{env, error::Error, fs, result::Result, string::String};

/// a fully valid `minigrep` command
///
/// consists of a `query` to search for and a `filename` to search within
pub struct Config {
    /// the query to search for
    pub query: String,
    /// the file to search within
    pub filename: String,
    /// whether query should be case sensitive or not
    pub case_sensitive: bool,
}

impl Config {
    /// attempt to create a new `Config` from given arguments
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
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

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
