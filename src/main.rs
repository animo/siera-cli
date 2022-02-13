//! An Aries Cloudagent Controller CLI to interact with Aries instances for data manipulation
//! run `accf -e=XXX invite` to run the example script

#![allow(clippy::enum_variant_names)]
#![warn(missing_docs, clippy::missing_docs_in_private_items)]

#[macro_use]
extern crate clap;

use cli::register::register_cli;

/// agent
mod agent;

/// cli
mod cli;

/// error
mod error;

/// utils
mod utils;

/// Initializes the application
#[tokio::main]
async fn main() {
          register_cli().await;
}
