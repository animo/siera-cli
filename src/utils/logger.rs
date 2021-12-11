use colored::*;

/// Logger struct that allows us access to stdout and stderr
pub struct Log;

impl Log {
    /// Log messages that contain generic info
    pub fn log(string: &str) {
        println!("{}", String::from(string));
    }

    /// Log messages that broke the program
    pub fn error(string: &str) -> ! {
        eprintln!("{}: {}", "Error".red(), String::from(string));
        std::process::exit(1)
    }
}
