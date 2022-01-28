use super::connections::ConnectionsModule;
use super::credential_definition::CredentialDefinitionModule;
use super::features::FeaturesModule;
use super::invite::InvitationsModule;
use super::issue_credential::CredentialsModule;
use super::message::MessagesModule;
use super::schema::SchemaModule;
use crate::agent::agents::{Agent, HttpAgentExtended};
use crate::agent::http_agent::HttpAgent;
use crate::error::{throw, Error};
use crate::utils::config;
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

    // Takes a path, but prepends the home directory... kinda sketchy
    let endpoint_from_config = config::get_value("/.config/acl/ex.ini", "Default", "endpoint");

    // Takes a path, but prepends the home directory... kinda sketchy
    let api_key_from_config = config::get_value("/.config/acl/ex.ini", "Default", "api_key");

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
    let agent = HttpAgent {
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
