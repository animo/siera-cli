use crate::cli::connections::Connection;
use crate::cli::connections::Connections;
use crate::cli::connections::ConnectionsConfig;
use crate::cli::credential_definition::CredentialDefinition;
use crate::cli::credential_definition::CredentialDefinitionConfig;
use crate::cli::credential_definition::CredentialDefinitions;
use crate::cli::features::Features;
use crate::cli::invite::Invitation;
use crate::cli::invite::InvitationConfig;
use crate::cli::issue_credential::IssueCredentialConfig;
use crate::cli::message::MessageConfig;
use crate::cli::schema::Schema;
use crate::cli::schema::SchemaConfig;
use crate::utils::logger::Log;
use async_trait::async_trait;
use serde_json::Value;

/// Base agent structure
#[derive(Clone, Debug)]
pub struct BaseAgent {
    /// Agent logger
    pub logger: Log,
}

/// Base agent functionality
#[async_trait]
pub trait Agent {
    /// returns the agent logger
    fn logger(&self) -> Log;

    /// Gets all the connections
    async fn get_connections(&self, filter: ConnectionsConfig) -> Connections;

    /// Get a connection by id
    async fn get_connection_by_id(&self, id: String) -> Connection;

    /// Prints an invitation, as url or qr, in stdout
    async fn create_invitation(&self, config: &InvitationConfig) -> Invitation;

    /// Requests all the features from the cloudagent
    async fn discover_features(&self) -> Features;

    /// Send a basic message to another agent
    async fn send_message(&self, config: &MessageConfig);

    /// Offer a credential to another agent
    async fn credential(&self, config: &IssueCredentialConfig) -> Value;

    /// Create schema at a ledger
    async fn schema(&self, config: &SchemaConfig) -> Schema;

    /// Register a credential definition on the ledger
    async fn credential_definition(
        &self,
        config: &CredentialDefinitionConfig,
    ) -> CredentialDefinition;

    /// Get all registered credential definitions
    async fn credential_definitions(&self) -> CredentialDefinitions;
}

/// HTTP specific cloudagent functionality
#[async_trait]
pub trait HttpAgentExtended {
    /// New http agent instance
    fn new(base_agent: BaseAgent, endpoint: String, api_key: Option<String>) -> Self;

    /// Check if the endpoint is valid
    async fn check_endpoint(&self) -> ();
}
