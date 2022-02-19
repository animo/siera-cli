use agent::modules::features::FeaturesModule;
use clap::Args;

use crate::utils::logger::Log;

#[derive(Args)]
pub struct FeaturesOptions {}

pub async fn parse_features_args(agent: impl FeaturesModule, logger: Log) {
    match agent.discover_features().await {
        Ok(features) => logger._log_pretty(features),
        Err(e) => logger.error(e.to_string()),
    }
}
