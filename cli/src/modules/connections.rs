use agent_controller::modules::connections::{ConnectionCreateInvitationOptions, ConnectionModule};
use clap::{Args, Subcommand};
use colored::*;
use log::info;

use crate::error::{Error, Result};
use crate::utils::{
    loader::{Loader, LoaderVariant},
    logger::{copy_to_clipboard, pretty_print_obj},
    qr::print_qr_code,
};

#[derive(Args)]
pub struct ConnectionOptions {
    #[clap(long, short, conflicts_with = "id")]
    pub all: bool,

    #[clap(long, short, conflicts_with = "all")]
    pub id: Option<String>,

    #[clap(subcommand)]
    pub commands: Option<ConnectionSubcommands>,
}

#[derive(Subcommand, Debug)]
pub enum ConnectionSubcommands {
    Invite {
        #[clap(long, short)]
        auto_accept: bool,
        #[clap(long, short)]
        qr: bool,
        #[clap(long, short)]
        toolbox: bool,
        #[clap(long, short)]
        multi_use: bool,
        #[clap(long, short = 'l')]
        alias: Option<String>,
    },
}

pub async fn parse_connection_args(
    options: &ConnectionOptions,
    agent: impl ConnectionModule,
    copy: bool,
) -> Result<()> {
    let loader = Loader::start(LoaderVariant::Dots);
    if let Some(id) = &options.id {
        return agent
            .get_connection_by_id(id.to_string())
            .await
            .map(|connections| {
                loader.stop();
                pretty_print_obj(connections)
            });
    }
    if options.all {
        return agent.get_connections().await.map(|connections| {
            loader.stop();
            pretty_print_obj(connections.results)
        });
    }
    match &options
        .commands
        .as_ref()
        .ok_or_else(|| Error::NoSubcommandSupplied("connections".to_string()))?
    {
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
                if *qr {
                    info!("{}", format!("{}: {}", "Connection id".green(), response.0));
                    print_qr_code(response.1).unwrap();
                } else {
                    info!("{}", format!("{}: {}", "Connection id".green(), response.0));
                    info!("{}", response.1);
                    if copy {
                        copy_to_clipboard(response.1);
                    }
                }
            })
        }
    }
}
