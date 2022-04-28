use std::env;
use std::process;

use lgr15_minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });


    println!("Searching for {:?}", config.query);
    println!("In file {:?}", config.filename);

    if let Err(e) = lgr15_minigrep::run(config) {
        eprintln!("App Error: {}", e);
        process::exit(1);
    }
}
