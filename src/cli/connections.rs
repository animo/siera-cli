use super::register::Module;
use crate::agent::agents::Agent;
use async_trait::async_trait;
use clap::ArgMatches;
use serde::{Deserialize, Serialize};

/// Type of the received connections list
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Connections {
    /// List of the current connections
    pub results: Vec<Connection>,
}

/// Type of the connections configuration as received by the cli
pub struct ConnectionsConfig {
    /// Filter connections by this alias
    pub alias: Option<String>,

    /// Get a connection by this id
    pub connection_id: Option<String>,
}

/// Type of a single received connection
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Connection {
    /// Invitation mode
    pub invitation_mode: String,

    /// Aries rfc23 state
    pub rfc23_state: String,

    /// Key of the invitation
    pub invitation_key: String,

    /// Their label
    pub their_label: Option<String>,

    /// Auto acceptance
    pub accept: String,

    /// Their did
    pub their_did: Option<String>,

    /// Timestamp of when the connection was created
    pub created_at: String,

    /// Their role in the invitation process
    pub their_role: String,

    /// When the connection was updated
    pub updated_at: String,

    /// State of the routing
    pub routing_state: String,

    /// The id of the connection
    pub connection_id: String,

    /// Your own did used for this connection
    pub my_did: Option<String>,

    /// State of the connection
    pub state: String,

    /// Alias given for this connection
    pub alias: Option<String>,
}

/// Connections module for the agent
pub struct ConnectionsModule;

/// Implementation of a module for connections
#[async_trait(?Send)]
impl Module<ConnectionsConfig> for ConnectionsModule {
    async fn run(agent: &dyn Agent, config: ConnectionsConfig) {
        let output = match config.connection_id {
            Some(id) => vec![agent.get_connection_by_id(id).await],
            None => agent.get_connections(config.alias).await.results,
        };
        agent.logger().log_pretty(output);
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
