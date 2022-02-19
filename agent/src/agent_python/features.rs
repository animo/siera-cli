use async_trait::async_trait;
use serde_json::Value;

use crate::{
    error::AgentResult,
    modules::features::{FeatureEndpoints, FeaturesModule},
    utils::web::create_url,
};

use super::agent::CloudAgentPython;

impl FeatureEndpoints for CloudAgentPython {
    fn endpoint_discover_features(&self) -> crate::error::AgentResult<reqwest::Url> {
        create_url(vec![&self.cloud_agent.endpoint, "features"])
    }
}

#[async_trait]
impl FeaturesModule for CloudAgentPython {
    async fn discover_features(&self) -> AgentResult<Value> {
        let url = self.endpoint_discover_features()?;

        self.cloud_agent.get::<Value>(url, None).await
    }
}
