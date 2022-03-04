use agent_controller::modules::features::FeaturesModule;
use clap::Args;

use crate::error::Result;
use crate::utils::logger::pretty_stringify_obj;
use crate::{debug, info};

#[derive(Args)]
pub struct FeaturesOptions {}

pub async fn parse_features_args(agent: impl FeaturesModule) -> Result<()> {
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
