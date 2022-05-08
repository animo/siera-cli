use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::Result;

/// Repsonse of the cloudagent for discovering features
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscoverFeaturesResponse {
    /// The query used for the request
    pub query_msg: Value,

    /// Which features are disclosed
    pub disclose: Disclose,
}

/// A feature which is in the `Disclosed` structure
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Disclose {
    /// The type of the request
    #[serde(rename = "@type")]
    pub type_field: String,

    /// The id of the request
    #[serde(rename = "@id")]
    pub id: String,

    /// All the supported protocols of the cloudagent
    pub protocols: Vec<Protocol>,
}

/// A protocol
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Protocol {
    /// Protocol id
    pub pid: String,
}

/// Generic cloudagent feature module
#[async_trait]
pub trait FeatureModule {
    /// Requests all the features from the cloudagent
    async fn discover_features(&self) -> Result<DiscoverFeaturesResponse>;
}
