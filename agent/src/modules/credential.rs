use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::Result;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialOfferResponse {
    #[serde(rename = "auto_issue")]
    pub auto_issue: bool,
    #[serde(rename = "auto_offer")]
    pub auto_offer: bool,
    #[serde(rename = "connection_id")]
    pub connection_id: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "credential_definition_id")]
    pub credential_definition_id: String,
    #[serde(rename = "credential_exchange_id")]
    pub credential_exchange_id: String,
    #[serde(rename = "credential_offer")]
    pub credential_offer: Value,
    #[serde(rename = "credential_offer_dict")]
    pub credential_offer_dict: Value,
    #[serde(rename = "credential_proposal_dict")]
    pub credential_proposal_dict: Value,
    pub initiator: String,
    pub role: String,
    #[serde(rename = "schema_id")]
    pub schema_id: String,
    pub state: String,
    #[serde(rename = "thread_id")]
    pub thread_id: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

pub struct CredentialOfferOptions {
    pub connection_id: String,
    pub cred_def_id: String,
    pub keys: Vec<String>,
    pub values: Vec<String>,
}

#[async_trait]
pub trait CredentialModule {
    async fn send_offer(&self, options: CredentialOfferOptions) -> Result<CredentialOfferResponse>;
}
