use crate::error::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectionCreateInvitationResponse {
    pub connection_id: String,
    pub invitation: Value,
    pub invitation_url: String,
    pub alias: Option<String>,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionGetAllResponse {
    pub results: Vec<ConnectionGetByIdResponse>,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionGetByIdResponse {
    #[serde(rename = "their_role")]
    pub their_role: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "my_did")]
    pub my_did: Option<String>,
    #[serde(rename = "connection_id")]
    pub connection_id: String,
    #[serde(rename = "their_did")]
    pub their_did: Option<String>,
    #[serde(rename = "invitation_key")]
    pub invitation_key: String,
    pub state: String,
    #[serde(rename = "routing_state")]
    pub routing_state: String,
    pub accept: String,
    #[serde(rename = "their_label")]
    pub their_label: Option<String>,
    #[serde(rename = "invitation_mode")]
    pub invitation_mode: String,
    #[serde(rename = "rfc23_state")]
    pub rfc23_state: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    pub alias: Option<String>,
    #[serde(rename = "request_id")]
    pub request_id: Option<String>,
    #[serde(rename = "invitation_msg_id")]
    pub invitation_msg_id: Option<String>,
}

#[async_trait]
pub trait ConnectionModule {
    /// Gets all the connections
    async fn get_all(&self) -> Result<ConnectionGetAllResponse>;

    /// Get a connection by id
    async fn get_by_id(&self, id: String) -> Result<ConnectionGetByIdResponse>;

    /// Create an invitation
    async fn create_invitation(
        &self,
        options: ConnectionCreateInvitationOptions,
    ) -> Result<ConnectionCreateInvitationResponse>;
}

#[derive(Debug)]
pub struct ConnectionCreateInvitationOptions {
    pub auto_accept: bool,
    pub qr: bool,
    pub toolbox: bool,
    pub multi_use: bool,
    pub alias: Option<String>,
}
