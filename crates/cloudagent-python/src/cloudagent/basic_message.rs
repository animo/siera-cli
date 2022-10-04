use crate::agent::CloudAgentPython;
use agent::error::Result;
use agent::modules::basic_message::{BasicMessageModule, SendBasicMessageOptions};
use async_trait::async_trait;
use serde_json::{json, Value};

#[async_trait]
impl BasicMessageModule for CloudAgentPython {
    async fn send_message(&self, options: SendBasicMessageOptions) -> Result<()> {
        let url = self.create_url(&["connections", &options.connection_id, "send-message"])?;

        let body = json!({
          "content": options.message,
        });

        self.post::<Value>(url, None, Some(body)).await?;

        Ok(())
    }
}
