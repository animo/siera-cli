use super::register::Module;
use crate::agent::agents::Agent;
use crate::utils::logger::Log;
use async_trait::async_trait;
use clap::ArgMatches;
use colored::Colorize;

/// Type of the message configuration as received by the cli
pub struct MessageConfig {
    /// connection id to send the message to
    pub connection_id: String,

    /// The message to send
    pub message: String,
}

/// Messages module for the agent
pub struct MessagesModule;

/// Implementation of a module for messages
#[async_trait(?Send)]
impl Module<MessageConfig> for MessagesModule {
    async fn run(agent: &dyn Agent, config: MessageConfig) {
        agent.send_message(&config).await;

        Log::log(&format!(
            "Sent \"{}\" to {}!",
            config.message.bright_purple(),
            config.connection_id.cyan()
        ));
    }

    async fn register<'a>(agent: &dyn Agent, matches: &ArgMatches<'a>) {
        if let Some(matches_connections) = matches.subcommand_matches("message") {
            // We can use unwrap here because these values are required by the cli
            let connection_id = matches_connections
                .value_of("connection-id")
                .unwrap()
                .to_string();
            let message = matches_connections.value_of("message").unwrap().to_string();

            let config = MessageConfig {
                connection_id,
                message,
            };

            MessagesModule::run(agent, config).await;
        }
    }
}
