use crate::error::Result;
use crate::help_strings::HelpStrings;
use crate::utils::loader::{Loader, LoaderVariant};
use clap::{Args, Subcommand};
use siera_agent::modules::multitenancy::MultitenancyModule;
use siera_logger::{pretty_stringify_obj, LogLevel};

/// Credential Definition options and flags
#[derive(Args)]
#[clap(about = "CRUD functionality with multitenancy wallets")]
pub struct MultitenancyOptions {
    /// All the subcommands of the multitenancy cli
    #[clap(subcommand)]
    pub commands: MultitenancySubcommands,
}

/// Credential Definition subcommands
#[derive(Subcommand, Debug)]
pub enum MultitenancySubcommands {
    /// Create a subwallet
    #[clap(about = HelpStrings::MultitenancyCreate)]
    Create {},

    /// Remove a subwallet
    #[clap(about = HelpStrings::MultitenancyRemove)]
    Remove {
        /// List a single credential definition by id
        #[clap(long, short, help = HelpStrings::MultitenancyRemoveWalletId)]
        wallet_id: String,
    },
}

/// Subcommand multitenancy parser
pub async fn parse_multitenancy_args(
    options: &MultitenancyOptions,
    agent: impl MultitenancyModule + Send + Sync,
) -> Result<()> {
    let log_level = &::siera_logger::STATE.read().unwrap().level;
    let loader: Option<Loader> = match log_level {
        LogLevel::Json => None,
        _ => Loader::start(&LoaderVariant::default()).into(),
    };

    match &options.commands {
        MultitenancySubcommands::Create {} => agent.create().await.map(|response| {
            if let Some(l) = loader {
                l.stop()
            }
            copy!("{}", response.wallet_id);
            log!("{}", pretty_stringify_obj(&response));
            log_json!("{}", pretty_stringify_obj(response));
        }),
        MultitenancySubcommands::Remove { wallet_id } => {
            agent.remove(wallet_id.clone()).await?;
            if let Some(l) = loader {
                l.stop()
            }
            log!("Successfully removed wallet with id: {}", wallet_id);
            log_json!("{}", "{}");
            Ok(())
        }
    }
}
