use crate::error::Result;
use async_trait::async_trait;

use crate::{
    modules::features::{FeatureEndpoints, Features, FeaturesModule},
    utils::web::create_url,
};

use super::agent::CloudAgentPython;

impl FeatureEndpoints for CloudAgentPython {
    fn endpoint_discover_features(&self) -> Result<reqwest::Url> {
        create_url(vec![&self.cloud_agent.endpoint, "features"])
    }
}

#[async_trait]
impl FeaturesModule for CloudAgentPython {
    async fn discover_features(&self) -> Result<Features> {
        let url = self.endpoint_discover_features()?;

        self.cloud_agent.get::<Features>(url, None).await
    }
}
