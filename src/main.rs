use std::{env, process};

use minigrep_test_package_001::Config;

/// run `minigrep` CLI application
///
/// this is the main entry-point for the `minigrep` CLI
fn main() {
    // try to parse CLI arguments into minigrep config;
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // if fail: convey error and exit
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // try to run minigrep; convey error and exit if fail
    if let Err(e) = minigrep_test_package_001::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
