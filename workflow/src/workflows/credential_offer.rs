use crate::error::Result;
use agent::modules::{
    connections::ConnectionModule,
    credential_definition::CredentialDefinitionModule,
    credentials::{CredentialsModule, CredentialsOfferOptions},
    schema::{SchemaCreateOptions, SchemaModule},
};
use std::collections::HashMap;

/// Credential offer workflow which offers an prebuilt credential to a connection
pub struct CredentialOfferWorkflow {
    connection_id: String,
    credential_deifnition_id: String,
    attributes: HashMap<String, String>,
}

/// All the modules an agent needs to imlement in order to execute the workflow
pub trait Agent:
    ConnectionModule + CredentialsModule + SchemaModule + CredentialDefinitionModule
{
}

impl CredentialOfferWorkflow {
    pub async fn execute(&self, agent: &dyn Agent) -> Result<()> {
        let attribute_keys: Vec<String> = self.attributes.keys().map(|e| e.to_owned()).collect();
        let attribute_values: Vec<String> =
            self.attributes.values().map(|e| e.to_owned()).collect();

        // Create or fetch the schema
        let schema = SchemaModule::create(
            agent,
            SchemaCreateOptions {
                name: String::from("credential-offer-workflow"),
                attributes: attribute_keys.to_owned(),
                version: String::from("1.0"),
            },
        )
        .await?;

        // Create or fetch the credential definition
        let credential_definition =
            CredentialDefinitionModule::create(agent, schema.schema_id).await?;

        let _ = agent
            .send_offer(CredentialsOfferOptions {
                keys: attribute_keys,
                values: attribute_values,
                connection_id: self.connection_id.to_owned(),
                cred_def_id: credential_definition.credential_definition_id,
            })
            .await?;

        Ok(())
    }
}
