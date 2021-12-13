use crate::typing::{
    Connection, Connections, Features, Invitation, InvitationConfig, IssueCredentialConfig,
    MessageConfig,
};
use async_trait::async_trait;

/// base cloudagent functionality
#[async_trait]
pub trait Agent {
    /// Gets all the connections
    async fn get_connections(&self, filter: Option<String>) -> Connections;

    /// Get a connection by id
    async fn get_connection_by_id(&self, id: String) -> Connection;

    /// Prints an invitation, as url or qr, in stdout
    async fn create_invitation(&self, config: &InvitationConfig) -> Invitation;

    /// Requests all the features from the cloudagent
    async fn discover_features(&self) -> Features;

    /// Send a basic message to another agent
    async fn send_message(&self, config: &MessageConfig) -> ();

    /// Offer a credential to another agent
    async fn offer_credential(&self, config: &IssueCredentialConfig) -> ();
}

/// HTTP specific cloudagent functionality
#[async_trait]
pub trait HttpAgentExtended {
    /// New http agent instance
    fn new(endpoint: String) -> Self;

    /// Check if the endpoint is valid
    async fn check_endpoint(&self) -> ();
}
