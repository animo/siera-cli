use super::register::Module;
use crate::agent::agents::Agent;
use async_trait::async_trait;
use clap::ArgMatches;
use serde::{Deserialize, Serialize};

/// Type of the received connections list
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CredentialDefinitions {
    /// List of the current connections
    pub credential_definition_ids: Vec<String>,
}

/// Type for received credential definition object
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CredentialDefinition {
    /// received value
    pub sent: CredentialDefinitionSent,
}

/// Sub value of Credential Definition
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CredentialDefinitionSent {
    /// credential definition id
    pub credential_definition_id: String,
}

/// Type of the credential definition configuration as received by the cli
pub struct CredentialDefinitionConfig {
    /// Id of the schema
    pub schema_id: Option<String>,

    /// Credential definition tag
    pub tag: String,
}

/// Credential definition module for the agent
pub struct CredentialDefinitionModule;

/// Implementation of a module for credentials
#[async_trait(?Send)]
impl Module<CredentialDefinitionConfig> for CredentialDefinitionModule {
    async fn run(agent: &dyn Agent, config: CredentialDefinitionConfig) {
        let output = match config.schema_id {
            Some(_) => vec![
                agent
                    .credential_definition(&config)
                    .await
                    .sent
                    .credential_definition_id,
            ],
            None => {
                agent
                    .credential_definitions()
                    .await
                    .credential_definition_ids
            }
        };
        agent.logger().log_pretty(output);
    }

    async fn register<'a>(agent: &dyn Agent, matches: &ArgMatches<'a>) {
        if let Some(matches_credential_definition) =
            matches.subcommand_matches("credential-definition")
        {
            let schema_id = matches_credential_definition
                .value_of("schema-id")
                .map(|x| x.to_string());
            let tag = matches_credential_definition
                .value_of("tag")
                .unwrap()
                .to_string();

            let config = CredentialDefinitionConfig { schema_id, tag };

            CredentialDefinitionModule::run(agent, config).await;
        }
    }
}
