use crate::error::Result;
use async_trait::async_trait;

/// Generic cloudagent schema module
#[async_trait]
pub trait WebhooksModule {
    /// Listen to all incoming webhooks
    async fn listen(&self, on_event: fn(serde_json::Value)) -> Result<()>;
}
