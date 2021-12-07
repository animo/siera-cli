use crate::typing::{Connection, Connections, Invitation, InviteConfiguration, Result};
use async_trait::async_trait;

#[async_trait]
pub trait Agent {
    async fn get_connections(&self) -> Connections;
    async fn get_connection_by_id(&self, id: String) -> Connection;
    async fn create_invitation(&self, config: &InviteConfiguration<'_>) -> Invitation;
}

#[async_trait]
pub trait HttpAgentExtended: Agent {
    fn new(endpoint: &str) -> Self;
    async fn check_endpoint(&self) -> Result<bool>;
}
