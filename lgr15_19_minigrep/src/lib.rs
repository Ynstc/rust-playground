/* 15.0 minigrep */


use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };


    for line in results {
        println!(" {}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl Config {

    pub fn new(args: &[String]) -> Result<Config, &str> {

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {query, filename, case_sensitive})
    }

}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    println!("15.0 search case sensitive");

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str)-> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    println!("15.0 search case insensitive");


    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }


    #[test]
    fn case_insensitive(){
        let query = "RuSt";
        let contents =  "\
Rust:
safe, fast, productive.
Pick three.
Trust me";

assert_eq!(
    vec!["Rust:", "Trust me"],
    search_case_insensitive(query, contents)
);
    }
}


/* 19.0 refator iterators */

pub fn run_iterators(config: ConfigIterators) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search_iterators(&config.query, &contents)
    } else {
        search_case_insensitive_iterators(&config.query, &contents)
    };


    for line in results {
        println!(" {}", line);
    }

    Ok(())
}

pub struct ConfigIterators {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl ConfigIterators {

    pub fn new(mut args: env::Args) -> Result<ConfigIterators, &'static str> {

        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename")
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(ConfigIterators {query, filename, case_sensitive})
    }

}

pub fn search_iterators<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    println!("19.0 search case sensitive");

    contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}

pub fn search_case_insensitive_iterators<'a>(query: &str, contents: &'a str)-> Vec<&'a str> {
    let query = query.to_lowercase();
    println!("19.0 search case insensitive");

    contents
    .lines()
    .filter(|line| line.to_lowercase().contains(&query))
    .collect()
}
