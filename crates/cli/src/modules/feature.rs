use crate::error::Result;
use crate::help_strings::HelpStrings;
use crate::utils::loader::{Loader, LoaderVariant};
use clap::Args;
use siera_agent::modules::feature::FeatureModule;

/// Automation options and flags
#[derive(Args)]
#[clap(about = HelpStrings::Features)]
pub struct FeaturesOptions {}

/// Subcommand Feature parser
pub async fn parse_features_args(agent: impl FeatureModule + Send + Sync) -> Result<()> {
    let loader = Loader::start(&LoaderVariant::default());
    agent.discover_features().await.map(|features| {
        loader.stop();
        debug!({ "features": features });
        log!({
            "protocols": features.disclose.protocols
        });
    })
}
