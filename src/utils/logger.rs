use colored::*;
use serde::Serialize;

/// Logger struct that allows us access to stdout and stderr
pub struct Log;

impl Log {
    /// json formatted stdout logger
    pub fn log_pretty<T: Serialize>(obj: T) {
        let buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"  ");
        let mut ser = serde_json::Serializer::with_formatter(buf, formatter);

        obj.serialize(&mut ser).unwrap();

        Log::log(&String::from_utf8(ser.into_inner()).unwrap());
    }

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
