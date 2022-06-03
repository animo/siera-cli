use async_trait::async_trait;

use crate::error::Result;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

/// Response of the `create` endpoint on the multitenancy module
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultitenancyCreateResponse {
    /// Timestamp of when the subwallet was created
    pub created_at: String,
    /// The mode of the key management (managed, unmanaged)
    pub key_management_mode: String,

    /// More wallet information
    pub settings: Value,

    /// JWT
    pub token: String,

    /// Timestamp of when the last update happened to the wallet
    pub updated_at: String,

    /// The wallet id
    pub wallet_id: String,
}

/// Multitenancy module for a generic cloudagent
#[async_trait]
pub trait MultitenancyModule {
    /// Create a new subwallet
    async fn create(&self) -> Result<MultitenancyCreateResponse>;

    /// Remove a subwallet
    async fn remove(&self, wallet_id: String) -> Result<()>;
}
