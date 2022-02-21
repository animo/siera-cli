// TODO: this should be under `server`
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::error::Result;

/// Type of the received features from `discover-features`
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Features {
    /// List of all the features the cloudagent supports
    pub results: Map<String, Value>,
}

#[async_trait]
pub trait FeaturesModule {
    /// Requests all the features from the cloudagent
    async fn discover_features(&self) -> Result<Features>;
}
