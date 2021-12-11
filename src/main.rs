//! An Aries Cloudagent Controller to interact with Aries instances for data manipulation
//! run `accf -e=XXX invite` to run the example script

#![allow(clippy::enum_variant_names)]
#![warn(missing_docs, clippy::missing_docs_in_private_items)]

#[macro_use]
extern crate clap;

use crate::agent::agents::HttpAgentExtended;
use crate::agent::http_agent::HttpAgent;
use crate::error::{throw, Error};
use clap::App;
use typing::{ConnectionsConfig, InvitationConfig};

/// agent
mod agent;

/// cli
mod cli;

/// error
mod error;

/// typing
mod typing;

/// utils
mod utils;

/// Initializes the application
#[tokio::main]
async fn main() {
    // Load the yaml file containing the cli setup
    let yaml = load_yaml!("../cli.yaml");

    // Get all the supplied flags and values
    let matches = App::from_yaml(yaml)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();

    // create an httpAgent when you supply an endpoint
    let agent = match matches.value_of("endpoint") {
        Some(endpoint) => HttpAgent::new(endpoint),
        None => throw(Error::InvalidEndpoint),
    };

    agent.check_endpoint().await;

    // Matches the `feature` subcommand
    if matches.subcommand_matches("features").is_some() {
        cli::features::run(&agent).await
    }

    // Matches the `connections` subcommand
    if let Some(matches_connections) = matches.subcommand_matches("connections") {
        let id = matches_connections.value_of("id");
        let alias = matches_connections.value_of("alias");

        let config = ConnectionsConfig { id, alias };

        cli::connections::run(&agent, config).await
    }

    // Matches the `invite` subcommand
    if let Some(matches_invite) = matches.subcommand_matches("invite") {
        let auto_accept = matches_invite.is_present("auto-accept");
        let multi_use = matches_invite.is_present("multi-use");
        let alias = matches_invite.value_of("alias");
        let qr = matches_invite.is_present("qr");
        let toolbox = matches_invite.is_present("toolbox");

        let config = InvitationConfig {
            auto_accept,
            multi_use,
            alias,
            qr,
            toolbox,
        };

        // create agent and convert config
        cli::invite::run(&agent, config).await
    }
}
