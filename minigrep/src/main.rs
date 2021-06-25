use std::{env, process};

use minigrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(0);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(0);
    }
}
