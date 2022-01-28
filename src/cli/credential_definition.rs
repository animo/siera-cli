use super::register::Module;
use crate::agent::agents::Agent;
use crate::utils::logger::Log;
use async_trait::async_trait;
use clap::ArgMatches;
use serde::{Deserialize, Serialize};

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
    pub schema_id: String,

    /// Credential definition tag
    pub tag: String,
}

/// Credential definition module for the agent
pub struct CredentialDefinitionModule;

/// Implementation of a module for credentials
#[async_trait(?Send)]
impl Module<CredentialDefinitionConfig> for CredentialDefinitionModule {
    async fn run(agent: &dyn Agent, config: CredentialDefinitionConfig) {
        let credential_definition = agent.credential_definition(&config).await;
        Log::log(&credential_definition.sent.credential_definition_id);
    }

    async fn register<'a>(agent: &dyn Agent, matches: &ArgMatches<'a>) {
        if let Some(matches_credential_definition) =
            matches.subcommand_matches("credential-definition")
        {
            let schema_id = matches_credential_definition
                .value_of("schema-id")
                .unwrap()
                .to_string();
            let tag = matches_credential_definition
                .value_of("tag")
                .unwrap_or("default")
                .to_string();

            let config = CredentialDefinitionConfig { schema_id, tag };

            CredentialDefinitionModule::run(agent, config).await;
        }
    }
}
