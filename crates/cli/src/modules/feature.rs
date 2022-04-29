use agent::modules::feature::FeatureModule;
use clap::Args;
use log::debug;

use crate::error::Result;
use crate::help_strings::HelpStrings;
use crate::utils::{
    loader::{Loader, LoaderVariant},
    logger::pretty_stringify_obj,
};

#[derive(Args)]
#[clap(about = HelpStrings::Features)]
pub struct FeaturesOptions {}

pub async fn parse_features_args(agent: impl FeatureModule) -> Result<()> {
    let loader = Loader::start(LoaderVariant::default());
    agent.discover_features().await.map(|features| {
        loader.stop();
        debug!("{}", pretty_stringify_obj(&features));
        features.disclose.protocols.iter().for_each(|p| {
            println!("{}", p.pid);
        });
    })
}
