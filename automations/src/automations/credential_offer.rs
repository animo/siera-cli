use crate::error::{Error, Result};
use agent::modules::{
    connection::ConnectionModule,
    credential::{CredentialModule, CredentialOfferOptions},
    credential_definition::CredentialDefinitionModule,
    schema::{SchemaCreateOptions, SchemaModule},
};
use colored::*;
use log::trace;
use std::collections::HashMap;

/// Credential offer Automation which offers an prebuilt credential to a connection
pub struct CredentialOfferAutomation {
    pub connection_id: String,
    pub attributes: HashMap<String, String>,
}

impl CredentialOfferAutomation {
    pub async fn execute(
        &self,
        agent: impl ConnectionModule + CredentialModule + SchemaModule + CredentialDefinitionModule,
    ) -> Result<()> {
        trace!("Starting automation CredentialOfferAutomation");
        trace!("{}", self.connection_id);
        trace!("{:#?}", self.attributes);

        let attribute_keys: Vec<String> = self.attributes.keys().map(|e| e.to_owned()).collect();
        let attribute_values: Vec<String> =
            self.attributes.values().map(|e| e.to_owned()).collect();

        // Check if it as a valid connection
        println!("{} the connection...", "Fetching".cyan());
        let connection = ConnectionModule::get_by_id(&agent, self.connection_id.to_owned()).await?;
        if connection.state != "active" && connection.state != "response" {
            return Err(Error::ConnectionNotReady.into());
        }

        // Create or fetch the schema
        println!("{} the schema...", "Registering".cyan());
        let schema = SchemaModule::create(
            &agent,
            SchemaCreateOptions {
                name: String::from("full-credential-offer-automation"),
                attributes: attribute_keys.to_owned(),
                version: String::from("1.0"),
            },
        )
        .await?;

        println!("{} the credential definition...", "Registering".cyan());
        // Create or fetch the credential definition
        let credential_definition =
            CredentialDefinitionModule::create(&agent, schema.schema_id).await?;

        println!("{} the credential...", "Offering".cyan());
        let credential_offer_response = agent
            .send_offer(CredentialOfferOptions {
                keys: attribute_keys,
                values: attribute_values,
                connection_id: self.connection_id.to_owned(),
                cred_def_id: credential_definition.credential_definition_id,
            })
            .await?;

        trace!("Automation completed and offered a credential");
        trace!("{:#?}", credential_offer_response);
        Ok(())
    }
}
