use super::register::Module;
use crate::agent::agents::Agent;
use crate::utils::logger::Log;
use async_trait::async_trait;
use clap::ArgMatches;

/// Features module for the agent
pub struct FeaturesModule;

/// Implementation of a module for features
#[async_trait(?Send)]
impl Module<()> for FeaturesModule {
    async fn run(agent: &dyn Agent, _: ()) {
        let features = agent.discover_features().await;
        for (_, item) in features.results.iter().enumerate() {
            Log::log(item.0);
        }
    }

    async fn register<'a>(agent: &dyn Agent, matches: &ArgMatches<'a>) {
        if matches.subcommand_matches("features").is_some() {
            FeaturesModule::run(agent, ()).await;
        }
    }
}
