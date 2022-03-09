extern crate log;

mod cli;
mod error;
mod help_strings;
mod modules;
mod register;
mod utils;

use colored::*;
use log::error;
use register::register;

#[tokio::main]
async fn main() {
    match register().await {
        Ok(_) => (),
        Err(e) => {
            error!("{} {}", "Error:".red(), e);
            std::process::exit(1);
        }
    }
}
