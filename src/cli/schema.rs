use super::register::Module;
use crate::agent::agents::Agent;
use crate::typing::SchemaConfig;
use crate::utils::logger::Log;
use async_trait::async_trait;
use clap::ArgMatches;

/// Schema module for the agent
pub struct SchemaModule;

/// Implementation of a module for schemas
#[async_trait(?Send)]
impl Module<SchemaConfig> for SchemaModule {
    async fn run(agent: &dyn Agent, config: SchemaConfig) {
        let schema = agent.schema(&config).await;
        Log::log(&schema.sent.schema_id);
    }

    async fn register<'a>(agent: &dyn Agent, matches: &ArgMatches<'a>) {
        if let Some(matches_schema) = matches.subcommand_matches("schema") {
            let name = matches_schema.value_of("name").unwrap().to_string();
            let attributes: Vec<String> = matches_schema
                .values_of("attribute")
                .unwrap()
                .map(|x| x.to_string())
                .collect();

            let config = SchemaConfig { attributes, name };

            SchemaModule::run(agent, config).await;
        }
    }
}
