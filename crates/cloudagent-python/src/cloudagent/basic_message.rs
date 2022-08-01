use crate::agent::CloudAgentPython;
use agent::error::Result;
use agent::modules::basic_message::{BasicMessageModule, SendBasicMessageOptions};
use async_trait::async_trait;
use serde_json::json;

#[async_trait]
impl BasicMessageModule for CloudAgentPython {
    async fn send_message(&self, options: SendBasicMessageOptions) -> Result<()> {
        let url = self.create_url(vec!["connections", &options.connection_id, "send-message"])?;

        let body = json!({
          "content": options.message,
        });

        self.post(url, None, Some(body)).await
    }
}
