use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Type of the invitation configuration as received by the cli
pub struct InvitationConfig<'a> {
    /// Whether the invitation should auto accept
    pub auto_accept: bool,

    /// Whether the invitation should be multi use
    pub multi_use: bool,

    /// Alias for the connection that will be created with that invitation
    pub alias: Option<&'a str>,

    /// Whether it will print a qr code instead of a url
    pub qr: bool,

    /// Whether it should use a pre-configured toolbox configuration
    pub toolbox: bool,
}

/// Type of the connections configuration as received by the cli
pub struct ConnectionsConfig<'a> {
    /// Filter connections by this alias
    pub alias: Option<&'a str>,

    /// Get a connection by this id
    pub id: Option<&'a str>,
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
pub struct Feature {
    /// List of all the features the cloudagent supports
    pub results: Map<String, Value>,
}
