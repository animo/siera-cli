use agent_controller::modules::credentials::{CredentialsModule, CredentialsOfferOptions};
use agent_controller::modules::schema::{SchemaCreateOptions, SchemaModule};
use agent_controller::modules::{
    connections::ConnectionModule, credential_definition::CredentialDefinitionModule,
};
use crate::error::Result;
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WorkflowOfferCredential {
    connection_id: Option<String>,
    attributes: BTreeMap<String, String>,
}

pub async fn offer_credential(
    agent: impl ConnectionModule + CredentialsModule + SchemaModule + CredentialDefinitionModule,
    fields: WorkflowOfferCredential,
) -> Result<()> {
    let connection_id = fields.connection_id.unwrap();

    let create_schema_options = SchemaCreateOptions {
        name: String::from("workflow-cred"),
        attributes: fields.attributes.keys().map(|k| k.to_string()).collect(),
        version: String::from("1.0")
    };
    let schema_id = SchemaModule::create(&agent,create_schema_options).await?;

    let cred_def_id = CredentialDefinitionModule::create(&agent,schema_id).await?.sent.credential_definition_id;

    let credential_offer_options = CredentialsOfferOptions { 
        connection_id, 
        cred_def_id , 
        keys: fields.attributes.keys().map(|k| k.to_string()).collect(), 
        values: fields.attributes.values().map(|v| v.to_string()).collect() 
    };

    CredentialsModule::send_offer(&agent, credential_offer_options).await?;
    Ok(())
}
