use agent::modules::credential_definition::CredentialDefinitionCreateOptions;
use agent::modules::credential_definition::CredentialDefinitionModule;
use agent::modules::schema::{SchemaCreateOptions, SchemaModule};

use colored::*;

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
    pub async fn execute(
        &self,
        agent: impl SchemaModule + CredentialDefinitionModule,
    ) -> Result<()> {
        let schema = SchemaModule::create(
            &agent,
            SchemaCreateOptions {
                attributes: self.attributes.to_owned(),
                name: self.name.to_owned(),
                version: self.version.to_owned(),
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
