use crate::typing::{Connection, Connections, Feature, Invitation, InvitationConfig};
use async_trait::async_trait;

#[async_trait]
pub trait Agent {
    async fn get_connections(&self, filter: Option<&str>) -> Connections;
    async fn get_connection_by_id(&self, id: &str) -> Connection;
    async fn create_invitation(&self, config: &InvitationConfig<'_>) -> Invitation;
    async fn discover_features(&self) -> Feature;
}

#[async_trait]
pub trait HttpAgentExtended {
    fn new(endpoint: &str) -> Self;
    async fn check_endpoint(&self) -> ();
}
