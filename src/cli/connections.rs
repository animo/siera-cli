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
#[derive(Debug)]
pub struct ConnectionsConfig {
    /// Filter based on the alias
    pub alias: Option<String>,

    /// Get a connection by this id
    pub connection_id: Option<String>,

    /// Filter based on the invitation key
    pub invitation_key: Option<String>,

    /// Filter based on your did
    pub my_did: Option<String>,

    /// Filter based on their did
    pub their_did: Option<String>,

    // TODO: enum
    /// Filter based on the state
    pub state: Option<String>,

    /// Filter based on their role
    pub their_role: Option<String>,
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
        let logger = agent.logger();
        match config.connection_id {
            Some(id) => {
                let connection = agent.get_connection_by_id(id).await;
                logger.log_pretty(connection);
            }
            None => {
                let connections = agent.get_connections(config).await.results;
                logger.log_list_pretty(connections);
            }
        };
    }

    async fn register<'a>(agent: &dyn Agent, matches: &ArgMatches<'a>) {
        if let Some(matches_connections) = matches.subcommand_matches("connections") {
            let connection_id = matches_connections
                .value_of("connection-id")
                .map(|x| x.to_string());

            let alias = matches_connections.value_of("alias").map(|x| x.to_string());

            let invitation_key = matches_connections
                .value_of("invitation-key")
                .map(|x| x.to_string());

            let my_did = matches_connections
                .value_of("my-did")
                .map(|x| x.to_string());

            let their_did = matches_connections
                .value_of("their-did")
                .map(|x| x.to_string());

            let their_role = matches_connections
                .value_of("their_role")
                .map(|x| x.to_string());

            let state = matches_connections.value_of("state").map(|x| x.to_string());

            let config = ConnectionsConfig {
                connection_id,
                alias,
                invitation_key,
                my_did,
                their_did,
                state,
                their_role,
            };

            ConnectionsModule::run(agent, config).await;
        }
    }
}
