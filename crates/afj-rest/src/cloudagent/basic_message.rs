use crate::agent::CloudAgentAfjRest;
use async_trait::async_trait;
use serde_json::json;
use siera_agent::error::Result;
use siera_agent::modules::basic_message::{BasicMessageModule, SendBasicMessageOptions};

#[async_trait]
impl BasicMessageModule for CloudAgentAfjRest {
    async fn send_message(&self, options: SendBasicMessageOptions) -> Result<()> {
        let url = self.create_url(&["basic-messages", &options.connection_id])?;

        let body = json!({
          "content": options.message,
        });

        self.post(url, None, Some(body)).await
    }
}
