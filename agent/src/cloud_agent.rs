use anyhow::Result;
use async_trait::async_trait;

/// Cloudagent properties
#[derive(Debug, Clone)]
pub struct CloudAgent {
    /// base url of the cloudagent
    pub endpoint: String,

    /// admin Api key for the cloudagent
    pub api_key: Option<String>,
}

/// Cloudagent specific functionality
#[async_trait]
pub trait CloudAgentExtended {
    /// Check if the endpoint is valid
    async fn check_endpoint(&self) -> Result<()>;
}
