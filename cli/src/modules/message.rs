use agent_controller::modules::message::{MessageModule, SendMessageOptions};
use clap::Args;

use crate::error::Result;
use crate::utils::logger::Log;

#[derive(Args)]
pub struct MessageOptions {
    #[clap(short, long)]
    id: String,
    #[clap(short, long)]
    message: String,
}

pub async fn parse_message_args(
    options: &MessageOptions,
    agent: impl MessageModule,
    logger: Log,
) -> Result<()> {
    let send_options = SendMessageOptions {
        id: options.id.to_owned(),
        message: options.message.to_owned(),
    };
    agent
        .send_message(send_options)
        .await
        .map(|msg| logger.log(format!("Sent message: {}", msg)))
}
