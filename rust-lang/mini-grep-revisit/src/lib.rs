use std::{env, fs}; // The fs module provides functions for working with the file system, such as reading and writing files, creating directories, and managing file metadata. It allows you to perform various operations on files and directories, such as opening, reading, writing, and deleting them.
use std::error::Error;

// the std::env::args().collect(); can also be used as an alternative to the env::args().collect without the need to so the `use std::env;`


pub fn run(config: Config) -> Result<(), Box<dyn Error>> { 
    let contents = fs::read_to_string(config.file_path)?;
    //.expect("Should have been able to read the file"); // The fs::read_to_string function reads the entire contents of a file into a String. It takes the file path as an argument and returns a Result<String, std::io::Error>. If the file is successfully read, it returns Ok(String) containing the file's contents. If there is an error (e.g., the file does not exist or cannot be read), it returns Err(std::io::Error) with details about the error. The expect method is used to handle the Result; if the Result is Err, it will panic and print the provided message.
    //println!("With the content:\n{}", contents);
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in search(&config.query, &contents) {
        println!("{line}");
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

/*impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok());

        Ok(Config {query, file_path, ignore_case})
    }
}*/
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

pub fn search <'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}



