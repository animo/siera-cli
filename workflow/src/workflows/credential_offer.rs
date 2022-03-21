use crate::error::{Error, Result};
use agent::modules::{
    connections::ConnectionModule,
    credential_definition::CredentialDefinitionModule,
    credentials::{CredentialsModule, CredentialsOfferOptions},
    schema::{SchemaCreateOptions, SchemaModule},
};
use log::trace;
use std::collections::HashMap;

/// Credential offer workflow which offers an prebuilt credential to a connection
pub struct CredentialOfferWorkflow {
    pub connection_id: String,
    pub attributes: HashMap<String, String>,
}

impl CredentialOfferWorkflow {
    pub async fn execute(
        &self,
        agent: impl ConnectionModule + CredentialsModule + SchemaModule + CredentialDefinitionModule,
    ) -> Result<()> {
        trace!("Starting workflow CredentialOfferWorkflow");
        trace!("{}", self.connection_id);
        trace!("{:#?}", self.attributes);

        let attribute_keys: Vec<String> = self.attributes.keys().map(|e| e.to_owned()).collect();
        let attribute_values: Vec<String> =
            self.attributes.values().map(|e| e.to_owned()).collect();

        // Check if it as a valid connection
        println!("Fetching the connection...");
        let connection = ConnectionModule::get_by_id(&agent, self.connection_id.to_owned()).await?;
        if connection.state != "active" && connection.state != "response" {
            return Err(Error::ConnectionNotReady.into());
        }

        // Create or fetch the schema
        println!("Registering the schema...");
        let schema = SchemaModule::create(
            &agent,
            SchemaCreateOptions {
                name: String::from("full-credential-offer-workflow"),
                attributes: attribute_keys.to_owned(),
                version: String::from("1.0"),
            },
        )
        .await?;

        println!("Registering the credential definition...");
        // Create or fetch the credential definition
        let credential_definition =
            CredentialDefinitionModule::create(&agent, schema.schema_id).await?;

        println!("Offering the credential...");
        let credential_offer_response = agent
            .send_offer(CredentialsOfferOptions {
                keys: attribute_keys,
                values: attribute_values,
                connection_id: self.connection_id.to_owned(),
                cred_def_id: credential_definition.credential_definition_id,
            })
            .await?;

        trace!("Workflow completed and offered a credential");
        trace!("{:#?}", credential_offer_response);
        Ok(())
    }
}
