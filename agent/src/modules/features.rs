use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::Result;

/// Type of the received features from `discover-features`
#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscoverFeaturesResponse {
    pub trace: bool,
    #[serde(rename = "query_msg")]
    pub query_msg: Value,
    pub disclose: Disclose,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Disclose {
    #[serde(rename = "@type")]
    pub type_field: String,
    #[serde(rename = "@id")]
    pub id: String,
    pub protocols: Vec<Protocol>,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Protocol {
    pub pid: String,
}

#[async_trait]
pub trait FeaturesModule {
    /// Requests all the features from the cloudagent
    async fn discover_features(&self) -> Result<DiscoverFeaturesResponse>;
}
