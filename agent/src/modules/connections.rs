use crate::error::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Type of the received invitation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Invitation {
    /// Connection id
    pub connection_id: String,

    /// Invitation object
    pub invitation: Value,

    /// Invitation url that can be used to accept it by another party
    pub invitation_url: String,

    /// Alias for the given invitation
    pub alias: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetConnectionsResponse {
    pub results: Vec<GetConnectionByIdResponse>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetConnectionByIdResponse {
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
    async fn get_connections(&self) -> Result<GetConnectionsResponse>;

    /// Get a connection by id
    async fn get_connection_by_id(&self, id: String) -> Result<GetConnectionByIdResponse>;

    /// Create an invitation
    async fn create_invitation(
        &self,
        options: ConnectionCreateInvitationOptions,
    ) -> Result<(String, String)>;
}

#[derive(Debug)]
pub struct ConnectionCreateInvitationOptions {
    pub auto_accept: bool,
    pub qr: bool,
    pub toolbox: bool,
    pub multi_use: bool,
    pub alias: Option<String>,
}
