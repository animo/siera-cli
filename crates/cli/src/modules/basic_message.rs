use crate::error::Result;
use crate::help_strings::HelpStrings;
use crate::utils::loader::{Loader, LoaderVariant};
use clap::Args;
use siera_agent::modules::basic_message::{BasicMessageModule, SendBasicMessageOptions};
use siera_logger::{pretty_stringify_obj, LogLevel};

/// Basic Message options and flags
#[derive(Args)]
#[clap(about = HelpStrings::Message)]
pub struct BasicMessageOptions {
    /// The connection id to which to send the connectoon to
    #[clap(short = 'i', long, help=HelpStrings::MessageId)]
    connection_id: String,

    /// The message that should be send to the connection id
    #[clap(short, long, help=HelpStrings::MessageMessage)]
    message: String,
}

/// Subcommand Basic Message parser
pub async fn parse_basic_message_args(
    options: &BasicMessageOptions,
    agent: impl BasicMessageModule + Send + Sync,
) -> Result<()> {
    let log_level = &::siera_logger::STATE.read().unwrap().level;
    let loader: Option<Loader> = match log_level {
        LogLevel::Json => None,
        _ => Loader::start(&LoaderVariant::default()).into(),
    };
    let send_options = SendBasicMessageOptions {
        connection_id: options.connection_id.clone(),
        message: options.message.clone(),
    };
    agent.send_message(send_options).await.map(|response| {
        if let Some(l) = loader {
            l.stop()
        }
        log!("Successfully sent message");
        log_json!("{}", pretty_stringify_obj(response));
    })
}
