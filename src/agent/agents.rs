use crate::typing::{Connection, Connections, Feature, Invitation, InvitationConfig};
use async_trait::async_trait;

/// base cloudagent functionality
#[async_trait]
pub trait Agent {
    /// Gets all the connections
    async fn get_connections(&self, filter: Option<&str>) -> Connections;

    /// Get a connection by id
    async fn get_connection_by_id(&self, id: &str) -> Connection;

    /// Prints an invitation, as url or qr, in stdout
    async fn create_invitation(&self, config: &InvitationConfig<'_>) -> Invitation;

    /// Requests all the features from the cloudagent
    async fn discover_features(&self) -> Feature;
}

/// HTTP specific cloudagent functionality
#[async_trait]
pub trait HttpAgentExtended {
    /// New http agent instance
    fn new(endpoint: &str) -> Self;

    /// Check if the endpoint is valid
    async fn check_endpoint(&self) -> ();
}
