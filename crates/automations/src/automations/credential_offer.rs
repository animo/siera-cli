use crate::{
    automations::create_credential_definition::CreateCredentialDefinition,
    error::{Error, Result},
};
use rand::RngCore;
use siera_agent::modules::{
    connection::ConnectionModule,
    credential::{CredentialModule, CredentialOfferOptions},
    credential_definition::CredentialDefinitionModule,
    schema::SchemaModule,
};
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
        info!({ "message": "Starting automation" });
        trace!({
           "connection_id": self.connection_id,
            "attributes": self.attributes
        });
        let attribute_keys: Vec<&str> = self
            .attributes
            .keys()
            .map(std::string::String::as_str)
            .collect();
        let attribute_values: Vec<String> = self.attributes.values().cloned().collect();

        // Check if it as a valid connection
        info! ({
            "message": "Fetching the connection..."
        });

        let connection = ConnectionModule::get_by_id(&agent, self.connection_id.clone()).await?;
        if connection.state != "active" && connection.state != "response" {
            return Err(Error::ConnectionNotReady.into());
        }

        let mut rng = rand::thread_rng();
        let version_major = rng.next_u32();
        let version_minor = rng.next_u32();
        let version = format!("{version_major}.{version_minor}");
        let create_credential_definition = CreateCredentialDefinition {
            version: &version,
            attributes: attribute_keys.clone(),
            name: "full-credential-offer-automation",
        };

        let credential_definition = create_credential_definition.execute(&agent).await?;

        info!({ "message": "Offering the credential..." });

        let credential_offer_response = agent
            .send_offer(CredentialOfferOptions {
                keys: attribute_keys.iter().map(|x| String::from(*x)).collect(),
                values: attribute_values,
                connection_id: self.connection_id.clone(),
                cred_def_id: credential_definition.credential_definition_id,
            })
            .await?;

        trace!({
            "message": "Automation completed and offered a credential"
        });

        trace!({ "credential_offer_response": credential_offer_response });

        Ok(())
    }
}
