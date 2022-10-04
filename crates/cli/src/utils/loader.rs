use core::time;
use std::io::{self, Write};
use std::sync::mpsc::{Receiver, Sender};

/// Prints a character to stdout and breaks the loop when a message is received
macro_rules! print_char_with_stop {
    ($char: expr, $timeout: expr, $inplace: expr, $receiver: expr) => {
        Loader::print_char($char, $timeout, $inplace);
        if $receiver.try_recv().is_ok() {
            break;
        }
    };
}

/// All the types of loaders
#[derive(Clone)]
pub enum LoaderVariant {
    /// Simple spinner loader that iterates over:
    /// / - | \
    Spinner,
    /// Dot loader that just outputs dots over and over
    #[allow(dead_code)]
    Dots,
}

/// Loader structure
pub struct Loader {
    /// Sender of a message channel that send a true when the loader should stop spinning
    /// We have to do this to get information to the thread where the spinner is spinning
    sender: Sender<bool>,
}

/// We implement default here so that the public api for calling the logger can just use this one
/// if unsure about which loader variant to call
impl Default for LoaderVariant {
    fn default() -> Self {
        Self::Spinner
    }
}

impl Loader {
    /// Start a specific loader
    pub fn start(loader_variant: &LoaderVariant) -> Self {
        let (sender, receiver) = std::sync::mpsc::channel::<bool>();
        match loader_variant {
            LoaderVariant::Spinner => Self::loader_spinner(receiver),
            LoaderVariant::Dots => Self::loader_dots(receiver),
        };
        Self { sender }
    }

    /// Stop the loader instance
    pub fn stop(&self) {
        elog!("\n");
        self.sender.send(false).unwrap();
    }

    /// prints a char to stdout with a delay and whether it should be inplace or
    /// not
    fn print_char(c: impl Into<String>, timeout: u64, inplace: bool) {
        if inplace {
            elog!("{}\r", c.into());
        } else {
            elog!("{}", c.into());
        }
        io::stdout().flush().unwrap();
        std::thread::sleep(time::Duration::from_millis(timeout));
    }

    /// Spinning loader. Does inplace replacement.
    fn loader_spinner(receiver: Receiver<bool>) {
        let time_between = 50;

        std::thread::spawn(move || loop {
            print_char_with_stop!('|', time_between, true, receiver);
            print_char_with_stop!('/', time_between * 2, true, receiver);
            print_char_with_stop!('-', time_between * 3, true, receiver);
            print_char_with_stop!('\\', time_between * 4, true, receiver);
            print_char_with_stop!('/', time_between * 5, true, receiver);
            print_char_with_stop!('-', time_between * 6, true, receiver);
            print_char_with_stop!('\\', time_between * 7, true, receiver);
        });
    }

    /// Dots loader
    /// Does not do inplace replacement but just keeps printing dots
    fn loader_dots(receiver: Receiver<bool>) {
        let time_between = 100;

        std::thread::spawn(move || loop {
            print_char_with_stop!('.', time_between, false, receiver);
        });
    }
}
