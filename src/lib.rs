use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

// Box<dyn Error> means the function will return a type that implements the Error trait, dyn means dynamic
// We don't actually know what the Error is (inside read_to_string), but we know it implements the Error trait
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
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