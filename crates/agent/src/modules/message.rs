use async_trait::async_trait;

use crate::error::Result;

/// Options that are supplied when sending a basic message to another agent
pub struct SendMessageOptions {
    /// The connection id to which to send the message
    pub connection_id: String,

    /// A simple text message
    pub message: String,
}

/// Generic cloudagent basic message module
#[async_trait]
pub trait MessageModule {
    /// Send a basic message to another agent via the connection id
    async fn send_message(&self, options: SendMessageOptions) -> Result<String>;
}
