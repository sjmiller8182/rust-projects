
use std::fs;
use std::env;
use std::error::Error;

pub struct Config{
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return  Err("Not enough arguments");
        }

        let case_sensitive = match args.get(3) {
            Some(arg) => parse_case_sensitivity_arg(arg),
            None => env::var("CASE_INSENSITIVE").is_err(),
        };
        //let case_sensitive = case_insens_arg;
        let query = args[1].clone();
        let filename = args[2].clone();
        //let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {query, filename, case_sensitive})
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = vec![];

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut results = vec![];

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub fn parse_case_sensitivity_arg(arg: &str) -> bool{
    if arg.to_lowercase() == "sensitive" {
        return true
    }
    false
}

#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct Tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "dUct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust and Duct me.";

        assert_eq!(vec!["safe, fast, productive.", "Trust and Duct me."], search_case_insensitive(query, contents))
    }

    #[test]
    fn parse_case_sens_arg() {
        let case_sens_arg = "sensitive";
        assert!(parse_case_sensitivity_arg(case_sens_arg))
    }

    #[test]
    fn parse_case_insens_arg() {
        let case_sens_arg = "insensitive";
        assert!(!parse_case_sensitivity_arg(case_sens_arg))
    }
}

