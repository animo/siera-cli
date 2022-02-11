use std::path::Path;

use super::connections::ConnectionsModule;
use super::credential_definition::CredentialDefinitionModule;
use super::features::FeaturesModule;
use super::invite::InvitationsModule;
use super::issue_credential::CredentialsModule;
use super::message::MessagesModule;
use super::schema::SchemaModule;
use crate::agent::agents::Agent;
use crate::agent::agents::BaseAgent;
use crate::agent::agents::HttpAgentExtended;
use crate::agent::http_agent::HttpAgent;
use crate::error::{throw, Error};
use crate::utils::config;
use crate::utils::logger::Log;
use async_trait::async_trait;
use clap::{App, ArgMatches};

/// trait that every submodule MUST implement
#[async_trait(?Send)]
pub trait Module<T> {
    /// Runner function that executes when the subcommand is called
    async fn run(agent: &dyn Agent, config: T);

    /// Registering a submodule
    async fn register<'a>(agent: &dyn Agent, matches: &ArgMatches<'a>);
}

/// Registers all the components of the cli
pub async fn register_cli() {
    // Load the yaml file containing the cli setup
    let yaml = load_yaml!("../../cli.yaml");

    // Get all the supplied flags and values
    let matches = App::from_yaml(yaml)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();

    // TODO: use `path`
    let mut default_path = std::env::var("HOME").unwrap();
    default_path.push_str("/.config/aries-cli/config.ini");

    let config_path = matches.value_of("config").unwrap_or(&default_path);

    let endpoint_from_config = config::get_value(config_path, "Default", "endpoint");

    let api_key_from_config = config::get_value(config_path, "Default", "api-key");

    // Get the endpoint when you supply an endpoint
    let endpoint = match matches.value_of("endpoint") {
        Some(endpoint) => endpoint.to_string(),
        None => match endpoint_from_config {
            Some(e) => e,
            None => throw(Error::NoSuppliedEndpoint),
        },
    };

    // TODO: Check if this can be refactored
    // Get the endpoint when you supply an endpoint
    let api_key = match matches.value_of("apikey") {
        Some(api_key) => Some(api_key.to_string()),
        None => api_key_from_config,
    };

    // Whether the output of the command should be copied to the users buffer
    let should_copy = matches.is_present("copy");

    // Suppresses output the cli
    let suppress_output = matches.is_present("suppress-output");

    // Instantiate the agent logger
    let logger = Log {
        should_copy,
        suppress_output,
    };

    // Base agent instance
    let base_agent = BaseAgent { logger };

    // TODO: this should be configurable to support more types of agents
    // e.g. aca-py, afj, non-couldagents
    // Http agent instance
    let agent = HttpAgent {
        base_agent,
        url: endpoint,
        api_key,
    };

    agent.check_endpoint().await;

    // Registering the subcommands and their modules
    ConnectionsModule::register(&agent, &matches).await;
    InvitationsModule::register(&agent, &matches).await;
    CredentialsModule::register(&agent, &matches).await;
    MessagesModule::register(&agent, &matches).await;
    FeaturesModule::register(&agent, &matches).await;
    SchemaModule::register(&agent, &matches).await;
    CredentialDefinitionModule::register(&agent, &matches).await;
}
