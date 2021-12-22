use std::iter::zip;

use super::register::Module;
use crate::agent::agents::Agent;
use crate::typing::IssueCredentialConfig;
use crate::utils::logger::Log;
use async_trait::async_trait;
use clap::ArgMatches;
use serde_json::json;

/// Credentials module for the agent
pub struct CredentialsModule;

/// Implementation of a module for credentials
#[async_trait(?Send)]
impl Module<IssueCredentialConfig> for CredentialsModule {
    async fn run(agent: &dyn Agent, config: IssueCredentialConfig) {
        let credential = agent.credential(&config).await;

        Log::log_pretty(credential);
    }

    async fn register<'a>(agent: &dyn Agent, matches: &ArgMatches<'a>) {
        if let Some(matches_connections) = matches.subcommand_matches("issue-credential") {
            // We can use unwrap here because these values are required by the cli
            let keys: Vec<&str> = matches_connections.values_of("key").unwrap().collect();
            let values: Vec<&str> = matches_connections.values_of("value").unwrap().collect();
            let connection_id = matches_connections
                .value_of("connection-id")
                .unwrap()
                .to_string();
            let credential_definition_id = matches_connections
                .value_of("credential-definition-id")
                .unwrap()
                .to_string();
            let mut attributes = vec![];

            for (key, value) in zip(keys, values) {
                attributes.push(json!({"name": key, "value": value}));
            }

            let config = IssueCredentialConfig {
                connection_id,
                credential_definition_id,
                attributes,
            };

            CredentialsModule::run(agent, config).await;
        }
    }
}
