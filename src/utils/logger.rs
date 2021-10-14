use colored::*;

pub struct Log {}

impl Log {
    pub fn log(string: &str) {
        println!("{}", String::from(string));
    }

    pub fn warn(string: &str) {
        println!("{}: {}", "Warning".yellow(), String::from(string));
    }

    pub fn error(string: &str) -> ! {
        eprintln!("{}: {}", "Error".red(), String::from(string));
        std::process::exit(1)
    }
}
