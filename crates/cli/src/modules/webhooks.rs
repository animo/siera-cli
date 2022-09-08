use crate::{help_strings::HelpStrings, error::Error};
use clap::{Args, Subcommand};
use cloudagent_python::agent::CloudAgentPython;
use std::str;
use tungstenite::{connect};
use url::Url;
use serde_json::{to_string_pretty};


/// Webhooks options and flags
#[derive(Args)]
#[clap(about = HelpStrings::Schema)]
pub struct WebhooksOptions {
    /// All the subcommands of the Webhooks cli
    #[clap(subcommand)]
    pub commands: WebhooksSubcommands,
}

/// Webhooks subcommands
#[derive(Subcommand, Debug)]
#[clap(about = HelpStrings::Webhooks)]
pub enum WebhooksSubcommands {
    /// Listen for webhooks on provided url
    #[clap(about = HelpStrings::Webhooks)]
    Listen {},
}

/// Listen to webhooks for an agentURL
pub fn listen(agent: CloudAgentPython) -> Result<()> {
    // TODO: filter by/listen to by topic
    let stripped_agent_url = match &agent.endpoint {
        s if s.starts_with("http://") => &s[7..], 
        s if s.starts_with("https://") => &s[8..],
        s => return Err(Error::InvalidAgentUrl(s.into())),
    };

    let listen_url = format!("wss://{}/ws", stripped_agent_url);
    log!("Listening on {}\n", listen_url);
    
    let (mut socket, _response) = connect({
        let input: &str = &listen_url;
        Url::options().parse(input).unwrap()
    })?;
    
    // Loop forever, parse message to stdout
    loop {
        let msg = socket.connect()?;
        log!("Received hook:\n");
        log!("{:?}", to_string_pretty(&msg));
    }
}