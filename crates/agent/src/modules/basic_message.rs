use crate::error::Result;
use async_trait::async_trait;

/// Options that are supplied when sending a basic message to another agent
pub struct SendBasicMessageOptions {
    /// The connection id to which to send the message
    pub connection_id: String,

    /// A simple text message
    pub message: String,
}

/// Generic cloudagent basic message module
#[async_trait]
pub trait BasicMessageModule {
    /// Send a basic message to another agent via the connection id
    async fn send_message(&self, options: SendBasicMessageOptions) -> Result<()>;
}
