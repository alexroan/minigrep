use std::env;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Must unwrap the Result type returned by Config::build
    // This block enables a graceful exit if the build fails
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        // Exit with a non-zero exit code
        std::process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // If run returns an error, print it and exit
    // run(config) by itself returns a warning because it's not being used
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        std::process::exit(1);
    }
}
