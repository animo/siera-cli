use super::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

pub type Result<T> = std::result::Result<T, Error>;

pub struct InviteConfiguration<'a> {
    pub auto_accept: bool,
    pub multi_use: bool,
    pub alias: Option<&'a str>,
    pub qr: bool,
    pub toolbox: bool,
}

#[derive(Serialize, Deserialize)]
pub struct InvitationOptions {
    pub alias: Option<String>,
    pub auto_accept: Option<String>,
    pub multi_use: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Connections {
    pub results: Vec<Connection>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Connection {
    pub invitation_mode: String,
    pub rfc23_state: String,
    pub invitation_key: String,
    pub their_label: Option<String>,
    pub accept: String,
    pub their_did: Option<String>,
    pub created_at: String,
    pub their_role: String,
    pub updated_at: String,
    pub routing_state: String,
    pub connection_id: String,
    pub my_did: Option<String>,
    pub state: String,
    pub alias: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Invitation {
    pub connection_id: String,
    pub invitation: MetaInvitation,
    pub invitation_url: String,
    pub alias: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaInvitation {
    #[serde(rename = "@type")]
    pub type_field: String,
    #[serde(rename = "@id")]
    pub id: String,
    pub service_endpoint: String,
    pub recipient_keys: Vec<String>,
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Feature {
    pub results: Map<String, Value>,
}
