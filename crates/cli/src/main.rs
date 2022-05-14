//! Agent interface via the cli. The command is `agent-cli` and can be followed
//! by some options, flags and subcommands. This is agent agnostic as long as
//! it implements the `agent module`.

#![deny(clippy::missing_docs_in_private_items)]

use colored::*;
use log::error;
use register::register;

/// Temporary logger crate
extern crate log;

/// Module for the whole cli
mod cli;

/// Generic error module
mod error;

/// Module for the help strings printed by the cli
mod help_strings;

/// Macro module
mod macros;

/// All of the subcommands split into modules
mod modules;

/// Module for registering the cli
mod register;

/// Generic utilities
mod utils;

/// Main entrypoint for the cli
#[tokio::main]
async fn main() {
    match register().await {
        Ok(_) => (),
        Err(e) => {
            error!("{} {}", "error:".bold().red(), e);
            std::process::exit(1);
        }
    }
}
