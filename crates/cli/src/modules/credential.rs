use crate::error::{Error, Result};
use crate::help_strings::HelpStrings;
use crate::utils::loader::{Loader, LoaderVariant};
use clap::{Args, Subcommand};
use siera_agent::modules::credential::{CredentialModule, CredentialOfferOptions};
use siera_logger::pretty_stringify_obj;

/// Credential options and flags
#[derive(Args)]
pub struct CredentialOptions {
    /// All the subcommands of the credential cli
    #[clap(subcommand)]
    pub commands: CredentialSubcommands,
}

/// Credential subcommands
#[derive(Subcommand, Debug)]
#[clap(about = HelpStrings::Credentials)]
pub enum CredentialSubcommands {
    /// Send a credential offer to the connection id
    #[clap(about = HelpStrings::CredentialsOffer)]
    Offer {
        /// The connection id to which to send the credential to
        #[clap(long, short = 'i', help  = HelpStrings::CredentialsOfferConnectionId)]
        connection_id: String,

        /// The credential definition used as a blueprint for the credential
        #[clap(long, short, help = HelpStrings::CredentialsOfferCredentialDefinitionId)]
        cred_def_id: String,

        /// list of all the keys used in the credential
        /// these are matched with a value based on the index
        #[clap(long, short, help = HelpStrings::CredentialsOfferKey)]
        key: Vec<String>,

        /// list of all the values used in the credential
        /// these are matched with a key based on the index
        #[clap(long, short, help = HelpStrings::CredentialsOfferValue)]
        value: Vec<String>,
    },
}

/// Subcommand credentials parser
pub async fn parse_credentials_args(
    commands: &CredentialSubcommands,
    agent: impl CredentialModule + Send + Sync,
) -> Result<()> {
    let loader = Loader::start(&LoaderVariant::default());
    match commands {
        CredentialSubcommands::Offer {
            connection_id,
            cred_def_id,
            key,
            value,
        } => {
            if key.len() != value.len() {
                return Err(Error::UnequalAmountKeyValue.into());
            }

            let options = CredentialOfferOptions {
                connection_id: connection_id.to_string(),
                cred_def_id: cred_def_id.to_string(),
                keys: key.iter().map(std::string::ToString::to_string).collect(),
                values: value.iter().map(std::string::ToString::to_string).collect(),
            };
            agent.send_offer(options).await.map(|cred| {
                loader.stop();
                log_debug!("{}", pretty_stringify_obj(&cred));
                log_info!("Successefully offered a credential. Credential exchange id: ",);
                log!("{}", cred.credential_exchange_id);
            })
        }
    }
}
