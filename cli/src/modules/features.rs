use agent_controller::modules::features::FeaturesModule;
use clap::Args;
use log::{debug, info};

use crate::error::Result;
use crate::utils::loader::{start_loader, Loader};
use crate::utils::logger::pretty_stringify_obj;

#[derive(Args)]
pub struct FeaturesOptions {}

pub async fn parse_features_args(agent: impl FeaturesModule) -> Result<()> {
    start_loader(Loader::Spinner);
    agent.discover_features().await.map(|features| {
        debug!("{}", pretty_stringify_obj(&features));
        features
            .results
            .keys()
            .collect::<Vec<_>>()
            .iter()
            .for_each(|x| {
                info!("{}", x);
            });
    })
}
