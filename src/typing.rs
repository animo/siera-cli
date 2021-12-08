use super::error::Error;
use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

pub struct InviteConfiguration<'a> {
    pub auto_accept: bool,
    pub multi_use: bool,
    pub alias: Option<&'a str>,
    pub qr: bool,
    pub toolbox: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub agent_endpoint: String,
    pub framework: String,
    pub connection_id: Option<String>,
    pub invitation_options: Option<InvitiationOptions>,
    pub actions: Option<ConfigActions>,
}

#[derive(Serialize, Deserialize)]
pub struct InvitiationOptions {
    pub alias: Option<String>,
    pub auto_accept: Option<String>,
    pub multi_use: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigActions {
    pub offer_credential: Option<ConfigOfferCredential>,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigOfferCredential {
    pub credential_definition_id: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
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
#[serde(rename_all = "camelCase")]
pub struct Invitation {
    #[serde(rename = "connection_id")]
    pub connection_id: String,
    pub invitation: MetaInvitation,
    #[serde(rename = "invitation_url")]
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
