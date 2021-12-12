use colored::*;
use serde::Serialize;

pub struct Log;

impl Log {
    pub fn pretty_log<T: Serialize>(obj: &T) {
        let buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"  ");
        let mut ser = serde_json::Serializer::with_formatter(buf, formatter);

        obj.serialize(&mut ser).unwrap();

        Log::log(&String::from_utf8(ser.into_inner()).unwrap());
    }

    // Log messages that contain generic info
    pub fn log(string: &str) {
        println!("{}", String::from(string));
    }

    // Log messages that broke the program
    pub fn error(string: &str) -> ! {
        eprintln!("{}: {}", "Error".red(), String::from(string));
        std::process::exit(1)
    }
}
