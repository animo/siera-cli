use crate::agent::CloudAgentPython;
use agent::error::{Error, Result};
use agent::modules::webhooks::WebhooksModule;
use async_trait::async_trait;
use logger::log;
use tungstenite::connect;

#[async_trait]
impl WebhooksModule for CloudAgentPython {
    /// Listen to all incoming webhooks
    async fn listen(&self, on_event: fn(serde_json::Value)) -> Result<()> {
        let stripped_agent_url = match &self.endpoint {
            s if s.starts_with("http://") => &s[7..],
            s if s.starts_with("https://") => &s[8..],
            s => return Err(Error::InvalidAgentUrl(s.clone()).into()),
        };

        let listen_url = format!("wss://{}/ws", stripped_agent_url);
        log!("Listening on {}", listen_url);

        let (mut socket, _response) = connect(listen_url)?;

        // Loop forever, parse message to stdout
        loop {
            let message = socket.read_message()?;
            let parsed: serde_json::Value = serde_json::from_str(&message.to_string())?;
            on_event(parsed);
        }
    }
}
