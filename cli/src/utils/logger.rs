use clipboard::{ClipboardContext, ClipboardProvider};
use colored::*;
use serde::Serialize;

/// Logger struct that allows us access to stdout and stderr
#[derive(Clone, Copy, Debug, Default)]
pub struct Log {
    /// Whether it should copy the output to a buffer
    pub should_copy: bool,

    /// Whether it should suppress the output to the ClI
    pub suppress_output: bool,
}

// TODO: Why do we need to remove quotes?
impl Log {
    /// json formatted stdout logger
    pub fn log_pretty(&self, obj: impl Serialize) {
        let buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"  ");
        let mut ser = serde_json::Serializer::with_formatter(buf, formatter);

        obj.serialize(&mut ser).unwrap();

        let output = &String::from_utf8(ser.into_inner()).unwrap();

        self.log(output);
    }

    /// Generic CLI logger
    pub fn log(&self, string: impl AsRef<str>) {
        if !self.suppress_output {
            println!("{}", string.as_ref().replace("\"", ""));
        }
        if self.should_copy {
            self.copy_to_buffer(string);
        }
    }

    /// Logs a list via the generic CLI logger
    pub fn log_list(&self, list: Vec<impl AsRef<str>>) {
        list.iter().for_each(|x| self.log(x));
    }

    /// Log messages that broke the program
    pub fn error(&self, e: Box<dyn std::error::Error>) -> ! {
        eprintln!("{}: {}", "Error".red(), e.to_string().replace("\"", ""));
        std::process::exit(1)
    }

    /// Copies a string to the buffer of your OS
    pub fn copy_to_buffer(&self, string: impl AsRef<str>) {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        ctx.set_contents(string.as_ref().to_string()).unwrap();
    }
}
