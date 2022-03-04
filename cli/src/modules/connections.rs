use agent_controller::modules::connections::{ConnectionCreateInvitationOptions, ConnectionModule};
use clap::{Args, Subcommand};
use colored::*;

use crate::error::{Error, Result};
use crate::utils::{logger::Log, qr::print_qr_code};

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

// TODO: we should implement `from` so we can use todo and have a cleaner api
pub async fn parse_connection_args(
    options: &ConnectionOptions,
    agent: impl ConnectionModule,
    logger: Log,
) -> Result<()> {
    if let Some(id) = &options.id {
        return agent
            .get_connection_by_id(id.to_string())
            .await
            .map(|connection| logger.log_pretty(connection));
    }
    if options.all {
        return agent
            .get_connections()
            .await
            .map(|connections| logger.log_pretty(connections.results));
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
                if *qr {
                    logger.log(format!("{}: {}", "Connection id".green(), response.0));
                    print_qr_code(response.1).unwrap();
                } else {
                    logger.log(format!("{}: {}", "Connection id".green(), response.0));
                    logger.log(response.1);
                }
            })
        }
    }
}
