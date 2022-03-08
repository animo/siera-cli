use core::time;
use std::io::{self, Write};

/// All the types of loaders
pub enum Loader {
    Spinner,
}

/// prints a char to stdout with a delay and whether it should be inplace or
/// not
pub fn print_char(c: impl Into<String>, timeout: u64, inplace: bool) {
    if inplace {
        print!("{}\r", c.into());
    } else {
        print!("{}", c.into());
    }
    io::stdout().flush().unwrap();
    std::thread::sleep(time::Duration::from_millis(timeout));
}

/// Start the loader
pub fn start_loader(loader: Loader) {
    match loader {
        Loader::Spinner => spinner_loader(),
    }
}

/// Spinning loader. Does inplace replacement.
fn spinner_loader() {
    let time_between = 50;

    std::thread::spawn(move || loop {
        print_char('|', time_between, true);
        print_char('/', time_between * 2, true);
        print_char('-', time_between * 3, true);
        print_char('\\', time_between * 4, true);
        print_char('/', time_between * 5, true);
        print_char('-', time_between * 6, true);
        print_char('\\', time_between * 7, true);
    });
}
