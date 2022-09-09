use crate::help_strings::HelpStrings;
use crate::error::{Result, Error};
use clap::{Args, Subcommand};
use cloudagent_python::agent::CloudAgentPython;
use colored::Colorize;
use logger::pretty_stringify_obj;
use std::str;
use tungstenite::connect;
use url::Url;


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
        s => return Err(Error::InvalidAgentUrl(s.clone()).into()),
    };

    let listen_url = format!("wss://{}/ws", stripped_agent_url);
    log!("Listening on {}", listen_url);
    
    let (mut socket, _response) = connect({
        let input: &str = &listen_url;
        Url::options().parse(input)?
    })?;
    
    // Loop forever, parse message to stdout
    loop {
        let message = socket.read_message()?;
        let parsed: serde_json::Value = serde_json::from_str(&message.to_string())?;
        let topic = parsed.get("topic");
        let incoming_webhook_message = match topic {
            Some(t) => format!("{}: (topic: {})", "Received hook".green(), t.to_string().blue()),
            None => format!("{}:", "Received hook".green()),
        };
        log!("{}", incoming_webhook_message);
        log!("{}", pretty_stringify_obj(parsed));
    }
}
