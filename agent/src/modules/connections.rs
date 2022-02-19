use async_trait::async_trait;

use crate::error::AgentResult;

#[async_trait]
pub trait ConnectionModule {
    /// Gets all the connections
    async fn get_connections(&self) -> AgentResult<()>;

    /// Get a connection by id
    async fn get_connection_by_id(&self, id: String) -> AgentResult<()>;

    /// Create an invitation
    async fn create_invitation(
        &self,
        config: ConnectionCreateInvitationConfig,
    ) -> AgentResult<String>;
}

#[derive(Debug)]
pub struct ConnectionCreateInvitationConfig {
    pub auto_accept: bool,
    pub qr: bool,
    pub toolbox: bool,
    pub multi_use: bool,
    pub alias: Option<String>,
}
