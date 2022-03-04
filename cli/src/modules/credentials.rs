use agent_controller::modules::credentials::{CredentialsModule, CredentialsOfferOptions};
use clap::{Args, Subcommand};
use log::{debug, info};

use crate::error::{Error, Result};
use crate::utils::logger::pretty_stringify_obj;
use colored::*;

#[derive(Args)]
pub struct CredentialOptions {
    #[clap(subcommand)]
    pub commands: CredentialSubcommands,
}

#[derive(Subcommand, Debug)]
pub enum CredentialSubcommands {
    Offer {
        #[clap(long, short = 'i')]
        connection_id: String,
        #[clap(long, short)]
        cred_def_id: String,
        #[clap(long, short)]
        key: Vec<String>,
        #[clap(long, short)]
        value: Vec<String>,
    },
    Propose {
        #[clap(long, short)]
        id: String,
    },
}

pub async fn parse_credentials_args(
    commands: &CredentialSubcommands,
    agent: impl CredentialsModule,
) -> Result<()> {
    match commands {
        CredentialSubcommands::Offer {
            connection_id,
            cred_def_id,
            key,
            value,
        } => {
            if key.len() != value.len() {
                return Err(Error::UnqualAmountKeyValue.into());
            }

            let options = CredentialsOfferOptions {
                connection_id: connection_id.to_string(),
                cred_def_id: cred_def_id.to_string(),
                keys: key.iter().map(|k| k.to_string()).collect(),
                values: value.iter().map(|v| v.to_string()).collect(),
            };
            agent.send_offer(options).await.map(|res| {
                debug!("{}", pretty_stringify_obj(res));
                info!("{} offered a credential", "Sucessefully".green());
            })
        }
        CredentialSubcommands::Propose { id: _id } => todo!(),
    }
}
