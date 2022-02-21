use agent_controller::modules::features::FeaturesModule;
use clap::Args;

use crate::error::Result;
use crate::utils::logger::Log;

#[derive(Args)]
pub struct FeaturesOptions {}

pub async fn parse_features_args(agent: impl FeaturesModule, logger: Log) -> Result<()> {
    agent.discover_features().await.map(|features| {
        if logger.debug {
            logger.log_pretty(features)
        } else {
            logger.log_list(features.results.keys().collect());
        }
    })
}
