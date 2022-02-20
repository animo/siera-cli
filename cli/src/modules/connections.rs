use agent::modules::connections::{ConnectionCreateInvitationOptions, ConnectionModule};
use clap::{Args, Subcommand};

use crate::error::Result;
use crate::utils::{logger::Log, qr::print_qr_code};

#[derive(Args)]
pub struct ConnectionOptions {
    #[clap(subcommand)]
    pub commands: ConnectionSubcommands,
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

// TODO: Can we just send a dereferenced struct directly?
pub async fn parse_connection_args(
    commands: &ConnectionSubcommands,
    agent: impl ConnectionModule,
    logger: Log,
) -> Result<()> {
    match commands {
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
            agent.create_invitation(options).await.map(|invite_url| {
                if *qr {
                    print_qr_code(invite_url).unwrap();
                } else {
                    logger.log(invite_url);
                }
            })
        }
    }
}
