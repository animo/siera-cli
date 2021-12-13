use super::register::Module;
use crate::agent::agents::Agent;
use crate::typing::ConnectionsConfig;
use crate::utils::logger::Log;
use async_trait::async_trait;
use clap::ArgMatches;

/// Connections module for the agent
pub struct ConnectionsModule;

/// Implementation of a module for connections
#[async_trait(?Send)]
impl Module<ConnectionsConfig> for ConnectionsModule {
    async fn run(agent: &dyn Agent, config: ConnectionsConfig) {
        match config.connection_id {
            Some(id) => {
                let connection = agent.get_connection_by_id(id).await;

                Log::log_pretty(connection);
            }
            None => {
                let connections = agent.get_connections(config.alias).await.results;

                Log::log_pretty(connections);
            }
        };
    }

    async fn register<'a>(agent: &dyn Agent, matches: &ArgMatches<'a>) {
        if let Some(matches_connections) = matches.subcommand_matches("connections") {
            let connection_id = matches_connections
                .value_of("connection-id")
                .map(|id| id.to_string());

            let alias = matches_connections
                .value_of("alias")
                .map(|alias| alias.to_string());

            let config = ConnectionsConfig {
                connection_id,
                alias,
            };

            ConnectionsModule::run(agent, config).await;
        }
    }
}
