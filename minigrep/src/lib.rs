use std::error::Error;
use std::{env, fs};
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case : bool
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.file_path)?;

    let result = if config.ignore_case {
        search(&config.query, &contents)
    }else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in result{
        println!("{line}");
    }

    Ok(())
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone(); // Clone to take ownership
        let file_path = args[2].clone(); // Clone to take ownership

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn search<'a>(query: &'a str, contains: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contains.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

pub fn search_case_insensitive<'a>(query : &'a str, contains : &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in contains.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "The";

        let contains = "The road to hell
            is there to be seen.
        ";

        assert_eq!(vec!["The road to hell"], search(query, contains));
    }

    #[test]
    fn case_insesitive() {
        let query = "rust";

        let contains = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contains)
        )
    }
}
