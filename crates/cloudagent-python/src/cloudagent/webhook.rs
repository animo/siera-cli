use crate::agent::CloudAgentPython;
use async_trait::async_trait;
use siera_agent::error::{Error, Result};
use siera_agent::modules::webhook::WebhookModule;
use tungstenite::connect;

#[async_trait]
impl WebhookModule for CloudAgentPython {
    /// Listen to all incoming webhook
    async fn listen(&self, on_event: fn(serde_json::Value)) -> Result<()> {
        let (uses_tls, stripped_agent_url) = match &self.endpoint {
            s if s.starts_with("http://") => (false, &s[7..]),
            s if s.starts_with("https://") => (true, &s[8..]),
            s => return Err(Error::InvalidAgentUrl(s.clone()).into()),
        };

        let scheme = if uses_tls { "wss" } else { "ws" };

        let listen_url = format!("{scheme}://{stripped_agent_url}/ws");
        info!({ "message": format!("Listening on {listen_url}") });

        let (mut socket, _response) = connect(listen_url)?;

        // Loop forever, parse message to stdout
        loop {
            let message = socket.read()?;
            let parsed: serde_json::Value = serde_json::from_str(&message.to_string())?;
            on_event(parsed);
        }
    }
}
