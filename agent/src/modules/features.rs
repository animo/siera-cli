use async_trait::async_trait;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::error::AgentResult;

/// Type of the received features from `discover-features`
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Features {
    /// List of all the features the cloudagent supports
    pub results: Map<String, Value>,
}

#[async_trait]
pub trait FeaturesModule {
    /// Requests all the features from the cloudagent
    async fn discover_features(&self) -> AgentResult<Features>;
}

pub trait FeatureEndpoints {
    fn endpoint_discover_features(&self) -> AgentResult<Url>;
}
