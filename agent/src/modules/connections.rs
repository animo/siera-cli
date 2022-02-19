use anyhow::Result;
use async_trait::async_trait;
use reqwest::Url;

#[async_trait]
pub trait ConnectionModule {
    /// Gets all the connections
    async fn get_connections(&self) -> Result<()>;

    /// Get a connection by id
    async fn get_connection_by_id(&self, id: String) -> Result<()>;

    /// Create an invitation
    async fn create_invitation(&self, config: ConnectionCreateInvitationConfig) -> Result<String>;
}

pub trait ConnectionEndpoints {
    /// base + connections
    fn endpoint_get_connections(&self) -> Result<Url>;
    /// base + connections + :id
    fn endpoint_get_connection_by_id(&self, id: &str) -> Result<Url>;
    /// base + connections + create-invitation
    fn endpoint_create_invitation(&self) -> Result<Url>;
}

#[derive(Debug)]
pub struct ConnectionCreateInvitationConfig {
    pub auto_accept: bool,
    pub qr: bool,
    pub toolbox: bool,
    pub multi_use: bool,
    pub alias: Option<String>,
}
