use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

// Impl is like a library in Solidity.
impl Config {
    // This enables Config::build to be called.
    // Return type: Result: Ok or Err, Config or &str
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

// Box<dyn Error> means the function will return a type that implements the Error trait, dyn means dynamic
// We don't actually know what the Error is (inside read_to_string), but we know it implements the Error trait
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    // for line in search(&config.query, &contents) {
    //     println!("{}", line);
    // }
    
    // iterator version
    search(&config.query, &contents).into_iter().for_each(|line| println!("{}", line));

    Ok(())
}

// Search through contents for query
// lifetime of the return value is the same as the lifetime of the contents argument because we return a slice of contents
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }

    // iterator version
    contents.lines().filter(|line| line.contains(query)).collect()
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

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}