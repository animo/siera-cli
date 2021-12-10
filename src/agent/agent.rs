use crate::typing::{Connection, Connections, Feature, Invitation, InviteConfiguration};
use async_trait::async_trait;

#[async_trait]
pub trait Agent {
    async fn get_connections(&self) -> Connections;
    async fn get_connection_by_id(&self, id: String) -> Connection;
    async fn create_invitation(&self, config: &InviteConfiguration<'_>) -> Invitation;
    async fn discover_features(&self) -> Feature;
}

#[async_trait]
pub trait HttpAgentExtended: Agent {
    fn new(endpoint: &str) -> Self;
    async fn check_endpoint(&self) -> ();
}
