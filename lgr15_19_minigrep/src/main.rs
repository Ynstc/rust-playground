use std::env;
use std::process;

use lgr15_19_minigrep::{Config, ConfigIterators};

fn main() {
    // minigrep();
    minigrep_refactor_iterators();
}

/* 15.0 minigrep */
fn minigrep() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });


    println!("Searching for {:?}", config.query);
    println!("In file {:?}", config.filename);

    if let Err(e) = lgr15_19_minigrep::run(config) {
        eprintln!("App Error: {}", e);
        process::exit(1);
    }
}

/* 19.0 refator iterators */
fn minigrep_refactor_iterators() {
    let config = ConfigIterators::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });



    println!("Searching for {:?}", config.query);
    println!("In file {:?}", config.filename);

    if let Err(e) = lgr15_19_minigrep::run_iterators(config) {
        eprintln!("App Error: {}", e);
        process::exit(1);
    }
}

/* how to use */
// 1. cargo run lor poem.txt
// set variable 'export CASE_INSENSITIVE=true`

// 2. cargo run lor poem.txt
// unset variable 'export `unset CASE_INSENSITIVE`

