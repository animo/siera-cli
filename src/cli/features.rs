use super::register::Module;
use crate::agent::agents::Agent;
use async_trait::async_trait;
use clap::ArgMatches;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Type of the received features from `discover-features`
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Features {
    /// List of all the features the cloudagent supports
    pub results: Map<String, Value>,
}

/// Features module for the agent
pub struct FeaturesModule;

/// Implementation of a module for features
#[async_trait(?Send)]
impl Module<()> for FeaturesModule {
    async fn run(agent: &dyn Agent, _: ()) {
        let features = agent.discover_features().await;
        agent.logger().log_list(features.results.keys().collect());
    }

    async fn register<'a>(agent: &dyn Agent, matches: &ArgMatches<'a>) {
        if matches.subcommand_matches("features").is_some() {
            FeaturesModule::run(agent, ()).await;
        }
    }
}
