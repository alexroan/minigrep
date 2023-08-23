use std::env;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Must unwrap the Result type returned by Config::build
    // This block enables a graceful exit if the build fails
    let config = Config::build(&args).unwrap_or_else(|_err| {
        // Exit with a non-zero exit code
        std::process::exit(1);
    });

    // If run returns an error, print it and exit
    // run(config) by itself returns a warning because it's not being used
    if let Err(_e) = minigrep::run(config) {
        std::process::exit(1);
    }
}
