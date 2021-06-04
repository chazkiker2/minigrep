use std::{env, process};

use minigrep::Config;


/// run `minigrep` CLI application
///
/// this is the main entry-point for the `minigrep` CLI
fn main() {
	// command line arguments input by user
	let args: Vec<String> = env::args().collect();

	// try to parse arguments into minigrep config; convey error and exit if fail
	let config = Config::new(&args).unwrap_or_else(|err| {
		eprintln!("Problem parsing arguments: {}", err);
		process::exit(1);
	});

	// try to run minigrep; convey error and exit if fail
	if let Err(e) = minigrep::run(config) {
		eprintln!("Application error: {}", e);
		process::exit(1);
	}
}
