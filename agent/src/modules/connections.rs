use async_trait::async_trait;

use crate::error::Result;

#[async_trait]
pub trait ConnectionModule {
    /// Gets all the connections
    async fn get_connections(&self) -> Result<()>;

    /// Get a connection by id
    async fn get_connection_by_id(&self, id: String) -> Result<()>;

    /// Create an invitation
    async fn create_invitation(&self, options: ConnectionCreateInvitationOptions)
        -> Result<String>;
}

#[derive(Debug)]
pub struct ConnectionCreateInvitationOptions {
    pub auto_accept: bool,
    pub qr: bool,
    pub toolbox: bool,
    pub multi_use: bool,
    pub alias: Option<String>,
}
