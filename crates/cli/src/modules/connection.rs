use agent::modules::connection::{
    ConnectionCreateInvitationOptions, ConnectionGetAllOptions, ConnectionModule,
    ConnectionReceiveInvitationOptions,
};
use clap::{Args, Subcommand};
use colored::*;
use log::{debug, info};
use std::str;

use crate::copy;
use crate::error::{Error, Result};
use crate::help_strings::HelpStrings;
use crate::utils::logger::pretty_stringify_obj;
use crate::utils::{
    loader::{Loader, LoaderVariant},
    qr::print_qr_code,
};

#[derive(Args)]
pub struct ConnectionOptions {
    #[clap(subcommand)]
    pub commands: ConnectionSubcommands,
}

#[derive(Subcommand, Debug)]
#[clap(about = HelpStrings::Connections)]
pub enum ConnectionSubcommands {
    #[clap(about = HelpStrings::ConnectionsInvite)]
    Invite {
        #[clap(long, short, help = HelpStrings::ConnectionsInviteAutoAccept)]
        auto_accept: bool,
        #[clap(long, short, help = HelpStrings::ConnectionsInviteQr)]
        qr: bool,
        #[clap(long, short, help = HelpStrings::ConnectionsInviteToolbox)]
        toolbox: bool,
        #[clap(long, short, help = HelpStrings::ConnectionsInviteMultiUse)]
        multi_use: bool,
        #[clap(long, short = 'l', help = HelpStrings::ConnectionsInviteAlias)]
        alias: Option<String>,
    },
    #[clap(about = HelpStrings::ConnectionsReceive)]
    Receive {
        #[clap(long, short, help = HelpStrings::ConnectionsReceiveUrl)]
        url: String,
    },
    #[clap(about = HelpStrings::ConnectionsList)]
    List {
        #[clap(long, short, help = HelpStrings::ConnectionsListId)]
        id: Option<String>,
        #[clap(long, short, help = HelpStrings::ConnectionsListAlias, conflicts_with = "id")]
        alias: Option<String>,
        #[clap(long, short, help = HelpStrings::ConnectionsListConnectionProtocol, conflicts_with = "id")]
        connection_protocol: Option<String>,
        #[clap(long, short = 'k', help = HelpStrings::ConnectionsListInvitationKey, conflicts_with = "id")]
        invitation_key: Option<String>,
        #[clap(long, short, help = HelpStrings::ConnectionsListMyDid, conflicts_with = "id")]
        my_did: Option<String>,
        #[clap(long, short, help = HelpStrings::ConnectionsListState, conflicts_with = "id")]
        state: Option<String>,
        #[clap(long, short = 'd', help = HelpStrings::ConnectionsListTheirDid, conflicts_with = "id")]
        their_did: Option<String>,
        #[clap(long, short = 'r', help = HelpStrings::ConnectionsListTheirRole, conflicts_with = "id")]
        their_role: Option<String>,
    },
}

pub async fn parse_connection_args(
    options: &ConnectionOptions,
    agent: impl ConnectionModule,
) -> Result<()> {
    let loader = Loader::start(LoaderVariant::default());

    match &options.commands {
        ConnectionSubcommands::Invite {
            auto_accept,
            qr,
            toolbox,
            multi_use,
            alias,
        } => {
            let options = ConnectionCreateInvitationOptions {
                alias: alias.as_deref().map(|a| a.to_string()),
                auto_accept: *auto_accept,
                multi_use: *multi_use,
                qr: *qr,
                toolbox: *toolbox,
            };
            agent.create_invitation(options).await.map(|response| {
                loader.stop();
                info!("{} invite with connection id: ", "Created".green());
                println!("{}", response.connection_id);
                if *qr {
                    info!("Scan this QR code to accept the invitation:\n");
                    print_qr_code(&response.invitation_url).unwrap();
                } else {
                    info!("Another agent can use this URL to accept your invitation:\n");
                    println!("{}", &response.invitation_url);
                }
                copy!("{}", response.invitation_url);
            })
        }
        ConnectionSubcommands::Receive { url } => {
            let invitation = invite_url_to_object(url.to_owned())?;
            agent
                .receive_invitation(invitation)
                .await
                .map(|connection| {
                    debug!("{}", pretty_stringify_obj(&connection));
                    info!("{} connection id:", "Fetched".green());
                    println!("{}", connection.connection_id);
                })
        }
        ConnectionSubcommands::List {
            id,
            alias,
            their_role,
            connection_protocol,
            invitation_key,
            my_did,
            state,
            their_did,
        } => match id {
            Some(i) => agent.get_by_id(i.to_owned()).await.map(|connection| {
                loader.stop();
                copy!("{}", pretty_stringify_obj(&connection));
                println!("{}", pretty_stringify_obj(connection))
            }),
            None => {
                let options = ConnectionGetAllOptions {
                    alias: alias.as_deref().map(|a| a.to_string()),
                    their_did: their_did.as_deref().map(|t| t.to_string()),
                    state: state.as_deref().map(|s| s.to_string()),
                    my_did: my_did.as_deref().map(|m| m.to_string()),
                    invitation_key: invitation_key.as_deref().map(|i| i.to_string()),
                    connection_protocol: connection_protocol.as_deref().map(|c| c.to_string()),
                    their_role: their_role.as_deref().map(|t| t.to_string()),
                };
                agent.get_all(options).await.map(|connections| {
                    loader.stop();
                    copy!("{}", pretty_stringify_obj(&connections.results));
                    println!("{}", pretty_stringify_obj(connections.results))
                })
            }
        },
    }
}

pub fn invite_url_to_object(url: String) -> Result<ConnectionReceiveInvitationOptions> {
    // Split the url
    let split_url = url
        .split("c_i=")
        .map(|u| u.to_owned())
        .collect::<Vec<String>>();

    // Get the query parameters
    let query_parameters = split_url
        .get(1)
        .ok_or(Error::InvalidAgentInvitation)?
        .split('&')
        .map(|u| u.to_owned())
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
    serde_json::from_str(decoded_str).map_err(|e| e.into())
}
