use agent::modules::credential_definition::CredentialDefinitionCreateOptions;
use agent::modules::credential_definition::CredentialDefinitionModule;
use agent::modules::schema::{SchemaCreateOptions, SchemaModule};

use colored::Colorize;

use crate::error::Result;

/// Automation for creating a credential definition
pub struct CreateCredentialDefinition {
    /// Schema name
    pub name: String,

    /// Attributes for the schema
    pub attributes: Vec<String>,

    /// Schema version
    pub version: String,
}

impl CreateCredentialDefinition {
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
        agent: impl SchemaModule + CredentialDefinitionModule + Send + Sync,
    ) -> Result<()> {
        let schema = SchemaModule::create(
            &agent,
            SchemaCreateOptions {
                attributes: self.attributes.clone(),
                name: self.name.clone(),
                version: self.version.clone(),
            },
        )
        .await?;

        let options = CredentialDefinitionCreateOptions {
            schema_id: schema.id,
            ..CredentialDefinitionCreateOptions::default()
        };

        log!("{} the credential definition...", "Registering".cyan());
        // Create or fetch the credential definition
        let credential_definition = CredentialDefinitionModule::create(&agent, options).await?;

        log!(
            "{} credential definition with id {}",
            "Created".green(),
            String::from(&credential_definition.credential_definition_id).green()
        );
        copy!("{}", credential_definition.credential_definition_id);
        Ok(())
    }
}
