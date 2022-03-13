use agent::modules::connections::{ConnectionCreateInvitationOptions, ConnectionModule};
use clap::{Args, Subcommand};
use colored::*;
use log::info;

use crate::copy;
use crate::error::Result;
use crate::utils::logger::pretty_stringify_obj;
use crate::utils::{
    loader::{Loader, LoaderVariant},
    logger::pretty_print_obj,
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
) -> Result<()> {
    let loader = Loader::start(LoaderVariant::default());
    if let Some(id) = &options.id {
        return agent.get_by_id(id.to_string()).await.map(|connections| {
            loader.stop();
            copy!("{}", pretty_stringify_obj(&connections));
            pretty_print_obj(connections)
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
                    if *qr {
                        info!(
                            "{}",
                            format!("{}: {}", "Connection id".green(), response.connection_id)
                        );
                        print_qr_code(response.invitation_url).unwrap();
                    } else {
                        info!("{}", response.invitation_url)
                    }
                })
            }
        },
        None => agent.get_all().await.map(|connections| {
            loader.stop();
            copy!("{}", pretty_stringify_obj(&connections.results));
            pretty_print_obj(connections.results)
        }),
    }
}
