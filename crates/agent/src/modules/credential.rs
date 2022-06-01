use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::Result;

/// Response structure gotten from the cloudagent when offering a credential
#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialOfferResponse {
    /// Whether it should auto issue the credential
    pub auto_issue: bool,

    /// Whether it should auto offer the credential
    pub auto_offer: bool,

    /// The conneciton id used to send the credential to
    pub connection_id: String,

    /// When the credential offer was created
    pub created_at: String,

    /// The credential definition id used for the credential
    pub credential_definition_id: String,

    /// Returned credential exchange id used for further credential functionality
    pub credential_exchange_id: String,

    /// The credential offer object
    pub credential_offer: Value,

    /// Dictionary of the credential offer
    pub credential_offer_dict: Value,

    /// Dictionary of the credential proposal
    pub credential_proposal_dict: Value,

    /// Who initiated the credential offer
    pub initiator: String,

    /// Your role in the credential offer flow
    pub role: String,

    /// Which schema was used when offering the credential
    pub schema_id: String,

    /// What the state is in the credential offer flow
    pub state: String,

    /// Thread id to refer to this offer
    pub thread_id: String,

    /// Last time the credential offer was updated
    pub updated_at: String,
}

/// Options when offering a credential
pub struct CredentialOfferOptions {
    /// Connection id to send the credential to
    pub connection_id: String,

    /// Credential definition id used a blueprint for the credential
    pub cred_def_id: String,

    /// Keys that are in the credential definition that must be filled in
    pub keys: Vec<String>,

    /// Values for the keys
    /// these are index-matched
    pub values: Vec<String>,
}

/// Generic cloudagent credential module
#[async_trait]
pub trait CredentialModule {
    /// Send a credential offer to the connection id supplied in the options
    async fn send_offer(&self, options: CredentialOfferOptions) -> Result<CredentialOfferResponse>;
}
