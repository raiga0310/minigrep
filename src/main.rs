extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // get config
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // print error
        eprintln!("Problem parsing arguments: {}", err);
        // exit
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
