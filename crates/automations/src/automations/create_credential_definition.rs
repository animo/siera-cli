use siera_agent::modules::credential_definition::CredentialDefinitionCreateOptions;
use siera_agent::modules::credential_definition::CredentialDefinitionCreateResponse;
use siera_agent::modules::credential_definition::CredentialDefinitionModule;
use siera_agent::modules::schema::{SchemaCreateOptions, SchemaModule};

use colored::Colorize;

use crate::error::Result;

/// Automation for creating a credential definition
pub struct CreateCredentialDefinition<'a> {
    /// Schema name
    pub name: &'a str,

    /// Attributes for the schema
    pub attributes: Vec<&'a str>,

    /// Schema version
    pub version: &'a str,
}

impl<'a> CreateCredentialDefinition<'a> {
    /// Main executor function
    /// 1. Register the schema
    /// 2. Register the credential definition
    ///
    /// # Errors
    ///
    /// - When the schema could not be created
    /// - When the credential definition could not be registered
    pub async fn execute(
        &self,
        agent: &(impl SchemaModule + CredentialDefinitionModule + Send + Sync),
    ) -> Result<CredentialDefinitionCreateResponse> {
        let schema = SchemaModule::create(
            agent,
            SchemaCreateOptions {
                name: self.name.to_owned(),
                version: self.version.to_owned(),
                attributes: self
                    .attributes
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect(),
            },
        )
        .await?;

        let options = CredentialDefinitionCreateOptions {
            schema_id: schema.id,
            ..CredentialDefinitionCreateOptions::default()
        };

        log!("{} the credential definition...", "Registering".cyan());
        // Create or fetch the credential definition
        let credential_definition = CredentialDefinitionModule::create(agent, options).await?;

        log!(
            "{} credential definition with id {}",
            "Created".green(),
            String::from(&credential_definition.credential_definition_id).green()
        );
        copy!("{}", credential_definition.credential_definition_id);
        Ok(credential_definition)
    }
}
