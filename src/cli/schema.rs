use super::register::Module;
use crate::agent::agents::Agent;
use async_trait::async_trait;
use clap::ArgMatches;
use serde::Deserialize;
use serde_json::Value;

/// Type of the schema configuration as received by the cli
pub struct SchemaConfig {
    /// Name of the schema
    pub name: String,

    /// Schema version
    pub version: String,

    /// Attributes that have to go inside the schema
    pub attributes: Vec<String>,
}

/// Type for received schema object
#[derive(Debug, Clone, Deserialize)]
pub struct Schema {
    /// received value
    pub sent: SchemaSent,
}

/// Sub value of Schema
#[derive(Debug, Clone, Deserialize)]
pub struct SchemaSent {
    /// Schema metadata
    pub schema: Value,

    /// Id of the schema
    pub schema_id: String,
}

/// Schema module for the agent
pub struct SchemaModule;

/// Implementation of a module for schemas
#[async_trait(?Send)]
impl Module<SchemaConfig> for SchemaModule {
    async fn run(agent: &dyn Agent, config: SchemaConfig) {
        let schema = agent.schema(&config).await;
        agent.logger().log(&schema.sent.schema_id);
    }

    async fn register<'a>(agent: &dyn Agent, matches: &ArgMatches<'a>) {
        if let Some(matches_schema) = matches.subcommand_matches("schema") {
            let name = matches_schema.value_of("name").unwrap().to_string();
            let version = matches_schema
                .value_of("schema-version")
                .unwrap_or("1.0")
                .to_string();
            let attributes: Vec<String> = matches_schema
                .values_of("attribute")
                .unwrap()
                .map(|x| x.to_string())
                .collect();

            let config = SchemaConfig {
                name,
                version,
                attributes,
            };

            SchemaModule::run(agent, config).await;
        }
    }
}
