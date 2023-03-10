use crate::error::Result;
use crate::help_strings::HelpStrings;
use crate::utils::loader::{Loader, LoaderVariant};
use clap::{Args, Subcommand};
use siera_agent::modules::multitenancy::MultitenancyModule;

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
    let loader = Loader::start(&LoaderVariant::default());

    match &options.commands {
        MultitenancySubcommands::Create {} => agent.create().await.map(|response| {
            loader.stop();
            info!({ "response": response });
            copy!("{}", response.wallet_id);
        }),
        MultitenancySubcommands::Remove { wallet_id } => {
            agent.remove(wallet_id.clone()).await?;
            loader.stop();
            info!({ "message": format!("Successfully removed wallet with id: {wallet_id}") });
            Ok(())
        }
    }
}
