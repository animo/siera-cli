use super::register::Module;
use crate::agent_controller::agents::Agent;
use async_trait::async_trait;
use clap::ArgMatches;
use serde::Serialize;
use serde_json::{json, Value};

/// Type of the issue credential configuration as received by the cli
#[derive(Debug, Serialize)]
pub struct IssueCredentialConfig {
    /// The connection to send the credential to
    pub connection_id: String,

    /// The credential definition used for the credential
    pub credential_definition_id: String,

    /// The attributes for the credential
    pub attributes: Vec<Value>,

    /// Whether it should display the full output
    pub full_output: bool,
}

/// Credentials module for the agent
pub struct CredentialsModule;

/// Implementation of a module for credentials
#[async_trait(?Send)]
impl Module<IssueCredentialConfig> for CredentialsModule {
    async fn run(agent: &dyn Agent, config: IssueCredentialConfig) {
        let logger = agent.logger();
        let credential = agent.credential(&config).await;

        logger.log("Credential has been offered!");

        if config.full_output {
            logger.log_pretty(credential);
        }
    }

    async fn register<'a>(agent: &dyn Agent, matches: &ArgMatches<'a>) {
        if let Some(matches_connections) = matches.subcommand_matches("issue-credential") {
            // We can use unwrap here because these values are required by the cli
            let keys: Vec<&str> = matches_connections.values_of("key").unwrap().collect();
            let values: Vec<&str> = matches_connections.values_of("value").unwrap().collect();
            let full_output = matches_connections.is_present("full-output");
            let connection_id = matches_connections
                .value_of("connection-id")
                .unwrap()
                .to_string();
            let credential_definition_id = matches_connections
                .value_of("credential-definition-id")
                .unwrap()
                .to_string();
            let mut attributes = vec![];

            // Iterate over every key and get the matching value.
            // iter::zip is an unstable feature and therefore we cannot use this.
            for (i, _) in keys.iter().enumerate() {
                let key = keys[i];
                let value = values[i];
                attributes.push(json!({"name": key, "value": value}));
            }

            let config = IssueCredentialConfig {
                full_output,
                connection_id,
                credential_definition_id,
                attributes,
            };

            CredentialsModule::run(agent, config).await;
        }
    }
}
