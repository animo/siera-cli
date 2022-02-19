use async_trait::async_trait;
use reqwest::Url;
use serde_json::Value;

use crate::error::AgentResult;

#[async_trait]
pub trait FeaturesModule {
    /// Requests all the features from the cloudagent
    async fn discover_features(&self) -> AgentResult<Value>;
}

pub trait FeatureEndpoints {
    fn endpoint_discover_features(&self) -> AgentResult<Url>;
}
