use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Type of the schema configuration as received by the cli
pub struct SchemaConfig {
    /// Name of the schema
    pub name: String,

    /// Attributes that have to go inside the schema
    pub attributes: Vec<String>,
}

/// Type of the credential definition configuration as received by the cli
pub struct CredentialDefinitionConfig {
    /// Id of the schema
    pub schema_id: String,

    /// Credential definition tag
    pub tag: String,
}

/// Type of the invitation configuration as received by the cli
pub struct InvitationConfig {
    /// Whether the invitation should auto accept
    pub auto_accept: bool,

    /// Whether the invitation should be multi use
    pub multi_use: bool,

    /// Alias for the connection that will be created with that invitation
    pub alias: Option<String>,

    /// Whether it will print a qr code instead of a url
    pub qr: bool,

    /// Whether it should use a pre-configured toolbox configuration
    pub toolbox: bool,
}

/// Type of the connections configuration as received by the cli
pub struct ConnectionsConfig {
    /// Filter connections by this alias
    pub alias: Option<String>,

    /// Get a connection by this id
    pub connection_id: Option<String>,
}

/// Type of the message configuration as received by the cli
pub struct MessageConfig {
    /// connection id to send the message to
    pub connection_id: String,

    /// The message to send
    pub message: String,
}

/// Type of the issue credential configuration as received by the cli
#[derive(Debug, Serialize)]
pub struct IssueCredentialConfig {
    /// The connection to send the credential to
    pub connection_id: String,

    /// The credential definition used for the credential
    pub credential_definition_id: String,

    /// The attributes for the credential
    pub attributes: Vec<Value>,
}

/// Type of the received connections list
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Connections {
    /// List of the current connections
    pub results: Vec<Connection>,
}

/// Type of a single received connection
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Connection {
    /// Invitation mode
    pub invitation_mode: String,

    /// Aries rfc23 state
    pub rfc23_state: String,

    /// Key of the invitation
    pub invitation_key: String,

    /// Their label
    pub their_label: Option<String>,

    /// Auto acceptance
    pub accept: String,

    /// Their did
    pub their_did: Option<String>,

    /// Timestamp of when the connection was created
    pub created_at: String,

    /// Their role in the invitation process
    pub their_role: String,

    /// When the connection was updated
    pub updated_at: String,

    /// State of the routing
    pub routing_state: String,

    /// The id of the connection
    pub connection_id: String,

    /// Your own did used for this connection
    pub my_did: Option<String>,

    /// State of the connection
    pub state: String,

    /// Alias given for this connection
    pub alias: Option<String>,
}

/// Type of the received invitation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Invitation {
    /// Connection id
    pub connection_id: String,

    /// Invitation object
    pub invitation: Map<String, Value>,

    /// Invitation url that can be used to accept it by another party
    pub invitation_url: String,

    /// Alias for the given invitation
    pub alias: Option<String>,
}

/// Type of the received features from `discover-features`
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Features {
    /// List of all the features the cloudagent supports
    pub results: Map<String, Value>,
}

/// Type for received schema object
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schema {
    /// received value
    pub sent: SchemaSent,
}

/// Sub value of Schema
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemaSent {
    /// Schema metadata
    pub schema: Value,

    /// Id of the schema
    pub schema_id: String,
}

/// Type for received credential definition object
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CredentialDefinition {
    /// received value
    pub sent: CredentialDefinitionSent,
}

/// Sub value of Credential Definition
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CredentialDefinitionSent {
    /// credential definition id
    pub credential_definition_id: String,
}
