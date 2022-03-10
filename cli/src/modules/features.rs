use agent::modules::features::FeaturesModule;
use clap::Args;
use log::{debug, info};

use crate::error::Result;
use crate::utils::{
    loader::{Loader, LoaderVariant},
    logger::pretty_stringify_obj,
};

#[derive(Args)]
pub struct FeaturesOptions {}

pub async fn parse_features_args(agent: impl FeaturesModule) -> Result<()> {
    let loader = Loader::start(LoaderVariant::default());
    agent.discover_features().await.map(|features| {
        loader.stop();
        debug!("{}", pretty_stringify_obj(&features));
        features.disclose.protocols.iter().for_each(|p| {
            info!("{}", p.pid);
        });
    })
}
