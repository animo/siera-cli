use async_trait::async_trait;

use crate::error::Result;

pub struct SendMessageOptions {
    pub id: String,
    pub message: String,
}

#[async_trait]
pub trait MessageModule {
    /// Requests all the features from the cloudagent
    async fn send_message(&self, options: SendMessageOptions) -> Result<String>;
}
