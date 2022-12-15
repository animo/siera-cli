use crate::error::{Error, Result};
use crate::help_strings::HelpStrings;
use crate::utils::loader::{Loader, LoaderVariant};
use crate::utils::qr::print_qr_code;
use clap::{Args, Subcommand};
use siera_agent::modules::oob::{
    OobConnectionCreateInvitationOptions, OobConnectionReceiveInvitationOptions, OobModule,
};
use siera_logger::{copy, pretty_stringify_obj};
use std::str;

/// Oob options and flags
#[derive(Args)]
pub struct OobOptions {
    /// All the subcommands of the Oob cli
    #[clap(subcommand)]
    pub commands: OobSubcommands,
}

/// Oob subcommands
#[derive(Subcommand, Debug)]
#[clap(about = HelpStrings::Oob)]
pub enum OobSubcommands {
    /// Create an invitation
    #[clap(about = HelpStrings::OobInvite)]
    Invite {
        /// Whether it should auto accept your side of the Oob flow
        #[clap(long, short, help = HelpStrings::OobInviteAutoAccept)]
        auto_accept: bool,

        /// Whether it should print a qr code
        #[clap(long, short, help = HelpStrings::OobInviteQr)]
        qr: bool,

        /// Whether the invitation should be reusable
        #[clap(long, short, help = HelpStrings::OobInviteMultiUse)]
        multi_use: bool,

        /// A custom alias for that specific Oob
        #[clap(long, short = 'p', help = HelpStrings::OobHandshakeProtocol, default_value="did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/didexchange/1.0")]
        handshake_protocol: String,

        /// A custom alias for that specific Oob
        #[clap(long, short = 'l', help = HelpStrings::OobInviteAlias)]
        alias: Option<String>,
    },

    /// Receive an invitation via url
    #[clap(about = HelpStrings::OobReceive)]
    Receive {
        /// Invitation url
        #[clap(long, short, help = HelpStrings::OobReceiveUrl)]
        url: String,
    },
}

/// Subcommand Oob parser
pub async fn parse_oob_args(
    options: &OobOptions,
    agent: impl OobModule + Send + Sync,
) -> Result<()> {
    let loader = Loader::start(&LoaderVariant::default());

    match &options.commands {
        OobSubcommands::Invite {
            auto_accept,
            qr,
            multi_use,
            handshake_protocol,
            alias,
        } => {
            let options = OobConnectionCreateInvitationOptions {
                alias: alias.as_deref().map(std::borrow::ToOwned::to_owned),
                handshake_protocol: handshake_protocol.clone(),
                auto_accept: *auto_accept,
                multi_use: *multi_use,
                qr: *qr,
            };
            agent.create_invitation(options).await.map(|response| {
                loader.stop();
                log_info!("Created invite with invitation msg id:");
                log!("{}", response.invitation_message_id);
                if *qr {
                    log_info!("Scan this QR code to accept the invitation:\n");
                    print_qr_code(&response.invitation_url).unwrap();
                } else {
                    log_info!("Another agent can use this URL to accept your invitation:\n");
                    log!("{}", &response.invitation_url);
                    log_json!({ "invitation_url": response.invitation_url })
                }
                copy!("{}", response.invitation_url);
            })
        }
        OobSubcommands::Receive { url } => {
            let invitation = invite_url_to_struct(url)?;
            agent
                .receive_invitation(invitation)
                .await
                .map(|connection| {
                    log_debug!("{}", pretty_stringify_obj(&connection));
                    log_info!("Fetched connection id:");
                    log!("{}", &connection.connection_id);
                    log_json!({ "connection_id": connection.connection_id})
                })
        }
    }
}

/// Create an invitation struct from an invitation url
pub fn invite_url_to_struct(url: impl AsRef<str>) -> Result<OobConnectionReceiveInvitationOptions> {
    // Split the url
    let split_url = url
        .as_ref()
        .split("oob=")
        .map(std::borrow::ToOwned::to_owned)
        .collect::<Vec<String>>();

    // Get the query parameters
    let query_parameters = split_url
        .get(1)
        .ok_or(Error::InvalidAgentInvitation)?
        .split('&')
        .map(std::borrow::ToOwned::to_owned)
        .collect::<Vec<String>>();

    let serialized_invitation = query_parameters
        .get(0)
        .ok_or(Error::InvalidAgentInvitation)?;

    // Base64 decode the invitation to a Vec<u8>
    let decoded =
        base64::decode(serialized_invitation).map_err(|_| Error::InvalidAgentInvitation)?;

    // Convert the vec to a valid string
    let decoded_str = str::from_utf8(&decoded)?;

    // Convert the string to an invitation object
    serde_json::from_str(decoded_str).map_err(std::convert::Into::into)
}
