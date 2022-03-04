extern crate log;

mod cli;
mod error;
mod modules;
mod register;
mod utils;

use register::register;
use colored::*;

#[tokio::main]
async fn main() {
    match register().await {
        Ok(_) => (),
        Err(e) => {
            err!("{} {}", "Error".red(), e);
            std::process::exit(1);
        }
    }
}
