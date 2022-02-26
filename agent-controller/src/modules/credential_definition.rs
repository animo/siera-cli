use crate::error::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Type for received schema object
#[derive(Debug, Clone, Deserialize)]
pub struct CreateCredentialDefinitionResponse {
    /// received value
    pub sent: CredentialDefinitionId,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CredentialDefinitionId {
    pub credential_definition_id: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCredentialDefinitionResponse {
    #[serde(rename = "credential_definition")]
    pub credential_definition: CredentialDefinition,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialDefinition {
    pub ver: String,
    pub id: String,
    pub schema_id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub tag: String,
    pub value: Value,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    pub primary: serde_json::Value,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct R {
    #[serde(rename = "master_secret")]
    pub master_secret: String,
    pub qqqqq: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct GetAllCredentialDefinitionsResponse {
    pub credential_definition_ids: Vec<String>,
}


#[async_trait]
pub trait CredentialDefinitionModule {
    /// Requests all the features from the cloudagent
    async fn create(&self, schema_id: String) -> Result<CreateCredentialDefinitionResponse>;
    async fn get_by_id(&self, id: String) -> Result<GetCredentialDefinitionResponse>;
    async fn get_all(&self) -> Result<GetAllCredentialDefinitionsResponse>;
}
