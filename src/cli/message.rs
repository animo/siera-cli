use colored::Colorize;

use crate::agent::agents::Agent;
use crate::typing::MessageConfig;
use crate::utils::logger::Log;

/// CLI runner for the `message` subcommand
pub async fn run(agent: &dyn Agent, config: MessageConfig<'_>) {
    agent.send_message(&config).await;

    Log::log(&format!(
        "Sent \"{}\" to {}!",
        config.message.bright_purple(),
        config.id.cyan()
    ));
}
