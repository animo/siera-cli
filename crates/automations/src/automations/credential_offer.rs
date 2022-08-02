use crate::{
    automations::create_credential_definition::CreateCredentialDefinition,
    error::{Error, Result},
};
use agent::modules::{
    connection::ConnectionModule,
    credential::{CredentialModule, CredentialOfferOptions},
    credential_definition::CredentialDefinitionModule,
    schema::SchemaModule,
};
use colored::Colorize;
use std::collections::HashMap;

/// Credential offer Automation which offers an prebuilt credential to a connection
pub struct CredentialOfferAutomation {
    /// Connection id to which the credential will be send to
    pub connection_id: String,
    /// Key Value pair of the attributes
    pub attributes: HashMap<String, String>,
}

impl CredentialOfferAutomation {
    /// Main executor function
    /// 1. Create a connection
    /// 2. Register the schema
    /// 3. Register the credential definition
    /// 4. Offer the credentail to the connection id
    ///
    /// # Errors
    ///
    /// - When the connection is not active
    /// - When The schema or credential definition could not be created
    /// - When the credential could not be send
    pub async fn execute(
        &self,
        agent: impl ConnectionModule
            + CredentialModule
            + SchemaModule
            + CredentialDefinitionModule
            + Send
            + Sync,
    ) -> Result<()> {
        log_trace!("Starting automation CredentialOfferAutomation");
        log_trace!("{}", self.connection_id);
        log_trace!("{:#?}", self.attributes);
        let attribute_keys: Vec<&str> = self.attributes.keys().map(|a| a.as_str()).collect();
        let attribute_values: Vec<String> = self.attributes.values().cloned().collect();

        // Check if it as a valid connection
        log!("{} the connection...", "Fetching".cyan());
        let connection = ConnectionModule::get_by_id(&agent, self.connection_id.clone()).await?;
        if connection.state != "active" && connection.state != "response" {
            return Err(Error::ConnectionNotReady.into());
        }

        let create_credential_definition = CreateCredentialDefinition {
            version: "1.0",
            attributes: attribute_keys.to_owned(),
            name: "full-credential-offer-automation",
        };

        let credential_definition = create_credential_definition.execute(&agent).await?;

        log!("{} the credential...", "Offering".cyan());
        let credential_offer_response = agent
            .send_offer(CredentialOfferOptions {
                keys: attribute_keys.iter().map(|x| String::from(*x)).collect(),
                values: attribute_values,
                connection_id: self.connection_id.clone(),
                cred_def_id: credential_definition.credential_definition_id,
            })
            .await?;

        log_trace!("Automation completed and offered a credential");
        log_trace!("{:#?}", credential_offer_response);
        Ok(())
    }
}
