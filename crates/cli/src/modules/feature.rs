use crate::error::Result;
use crate::help_strings::HelpStrings;
use crate::utils::loader::{Loader, LoaderVariant};
use agent::modules::feature::FeatureModule;
use clap::Args;
use logger::pretty_stringify_obj;

/// Automation options and flags
#[derive(Args)]
#[clap(about = HelpStrings::Features)]
pub struct FeaturesOptions {}

/// Subcommand Feature parser
pub async fn parse_features_args(agent: impl FeatureModule) -> Result<()> {
    let loader = Loader::start(LoaderVariant::default());
    agent.discover_features().await.map(|features| {
        loader.stop();
        log_debug!("{}", pretty_stringify_obj(&features));
        features.disclose.protocols.iter().for_each(|p| {
            log!("{}", p.pid);
        });
    })
}
