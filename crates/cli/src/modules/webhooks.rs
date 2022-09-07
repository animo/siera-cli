use crate::help_strings::HelpStrings;
use clap::{Args, Subcommand};
use std::str;
use tungstenite::{connect, Message};
use url::Url;
use serde_json::{from_str, to_string_pretty, Value};
use colored::Colorize;


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
pub fn listen(agent_url: String) -> ! {
    // TODO: filter by/listen to by topic
    let stripped_agent_url = match &agent_url {
        s if s.starts_with("http://") => &s[7..], 
        s if s.starts_with("https://") => &s[8..],
        s => s,
    };

    let listen_url = format!("wss://{}/ws", stripped_agent_url);
    log!("Listening on {}\n", listen_url);
    
    let (mut socket, _response) = match connect(Url::parse(&listen_url).unwrap()) {
        Ok((socket, _response)) => (socket, _response),
        Err(e) => {log!("{}", e); unreachable!()}
    };
    
    // Loop forever, parse message to stdout
    loop {
        // TODO: Replace with error string from enum
        let msg = match socket.read_message() {
            Ok(m) => m,
            Err(e) => {log!("{}", e); unreachable!()}
        };
        let msg = match msg {
            Message::Text(s) => { s }
            _ => { unreachable!() }
        };
        // TODO: Replace with error string from enum
        let parsed: Value = from_str(&msg).expect("Can't parse to JSON");
        log!("{}{}\n", ("Received hook:\n").yellow(), to_string_pretty(&parsed).unwrap());
    }
}