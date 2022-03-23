use agent::modules::connections::{
    ConnectionCreateInvitationOptions, ConnectionModule, ConnectionReceiveInvitationOptions,
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
    #[clap(long, short)]
    pub id: Option<String>,

    #[clap(subcommand)]
    pub commands: Option<ConnectionSubcommands>,
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
    Receive {
        #[clap(long, short)]
        url: String,
    },
}

pub async fn parse_connection_args(
    options: &ConnectionOptions,
    agent: impl ConnectionModule,
) -> Result<()> {
    let loader = Loader::start(LoaderVariant::default());
    if let Some(id) = &options.id {
        return agent.get_by_id(id.to_string()).await.map(|connections| {
            loader.stop();
            copy!("{}", pretty_stringify_obj(&connections));
            println!("{}", pretty_stringify_obj(connections));
        });
    }

    match &options.commands {
        Some(o) => match o {
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
                        println!("{}: {}", "Connection id".green(), response.connection_id);
                        print_qr_code(response.invitation_url).unwrap();
                    } else {
                        info!("Another agent can use this URL to accept your invitation:\n");
                        println!("{}", response.invitation_url)
                    }
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
        },
        None => agent.get_all().await.map(|connections| {
            loader.stop();
            copy!("{}", pretty_stringify_obj(&connections.results));
            println!("{}", pretty_stringify_obj(connections.results))
        }),
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
