use std::{collections::HashMap, env, process};
mod lib;

pub fn main() {
    let config = lib::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = lib::run(&config) {
        // macro to print in stderr
        eprintln!("Application error {}", e);
        process::exit(1);
    }
}
