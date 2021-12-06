use colored::*;

pub struct Log {}

impl Log {
    // used for when you want to print to STDOUT without a newline
    pub fn output(string: &str) {
        print!("{}", String::from(string))
    }

    // Log messages that contain generic info
    //pub fn log(string: &str) {
    //println!("{}", String::from(string));
    //}

    // Log messages that warn the user about minor things
    //pub fn warn(string: &str) {
    //println!("{}: {}", "Warning".yellow(), String::from(string));
    //}

    // Log messages that broke the program
    pub fn error(string: &str) -> ! {
        eprintln!("{}: {}", "Error".red(), String::from(string));
        std::process::exit(1)
    }
}
