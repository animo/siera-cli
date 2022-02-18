use async_trait::async_trait;

/// Base agent functionality
#[async_trait]
pub trait Agent {
    /// Gets all the connections
    async fn get_connections(&self, filter: String) -> String;

    /// Get a connection by id
    async fn get_connection_by_id(&self, id: String) -> String;

    /// Prints an invitation, as url or qr, in stdout
    async fn create_invitation(&self, config: String) -> String;

    /// Requests all the features from the cloudagent
    async fn discover_features(&self) -> String;

    /// Send a basic message to another agent
    async fn send_message(&self, config: String) -> String;

    /// Offer a credential to another agent
    async fn credential(&self, config: String) -> String;

    /// Create schema at a ledger
    async fn schema(&self, config: String) -> String;

    /// Register a credential definition on the ledger
    async fn credential_definition(&self, config: String) -> String;

    /// Get all registered credential definitions
    async fn credential_definitions(&self, config: String) -> String;
}

/// Cloudagent properties
#[derive(Debug, Clone)]
pub struct CloudAgent {
    /// base url of the cloudagent
    pub url: String,

    /// admin Api key for the cloudagent
    pub api_key: Option<String>,
}

/// Cloudagent specific functionality
#[async_trait]
pub trait CloudAgentExtended {
    /// Check if the endpoint is valid
    async fn check_endpoint(&self) -> ();
}
