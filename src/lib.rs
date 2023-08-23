use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

// Impl is like a library in Solidity.
impl Config {
    // This enables Config::build to be called.
    // Return type: Result: Ok or Err, Config or &str
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // String error
            return Err("Not enough arguments");
        }
        // must clone these because args is a borrowed reference
        let query = args[1].clone();
        let file_path = args[2].clone();

        // Return a Config struct
        Ok(Config { query, file_path })
    }
}

// Box<dyn Error> means the function will return a type that implements the Error trait, dyn means dynamic
// We don't actually know what the Error is (inside read_to_string), but we know it implements the Error trait
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

// Search through contents for query
// lifetime of the return value is the same as the lifetime of the contents argument because we return a slice of contents
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
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