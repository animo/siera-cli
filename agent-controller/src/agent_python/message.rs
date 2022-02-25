use crate::{
    error::Result,
    modules::message::{MessageModule, SendMessageOptions},
};
use async_trait::async_trait;
use serde_json::{json, Value};

use super::agent::CloudAgentPython;

#[async_trait]
impl MessageModule for CloudAgentPython {
    async fn send_message(&self, options: SendMessageOptions) -> Result<String> {
        let url = self
            .cloud_agent
            .create_url(vec!["connections", &options.id, "send-message"])?;

        let body = json!({
          "content": options.message,
        });

        self.cloud_agent
            .post::<Value>(url, None, Some(body))
            .await?;

        Ok(options.message)
    }
}
