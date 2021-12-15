use super::connections::ConnectionsModule;
use super::features::FeaturesModule;
use super::invite::InvitationsModule;
use super::issue_credential::CredentialsModule;
use super::message::MessagesModule;
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
    let endpoint_from_config = config::get_value("/.config/accf/ex.ini", "Default", "endpoint");

    // create an httpAgent when you supply an endpoint
    let agent = match matches.value_of("endpoint") {
        Some(endpoint) => HttpAgent::new(endpoint.to_string()),
        None => match endpoint_from_config {
            Some(e) => HttpAgent::new(e),
            None => match endpoint_from_config {
                Some(e) => HttpAgent::new(e),
                None => throw(Error::NoSuppliedEndpoint),
            },
        },
    };

    agent.check_endpoint().await;

    // Registering the subcommands and their modules
    ConnectionsModule::register(&agent, &matches).await;
    InvitationsModule::register(&agent, &matches).await;
    CredentialsModule::register(&agent, &matches).await;
    MessagesModule::register(&agent, &matches).await;
    FeaturesModule::register(&agent, &matches).await;
}
