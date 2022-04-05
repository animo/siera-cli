use super::agent::CloudAgentPython;
use agent::error::Result;
use agent::modules::feature::{DiscoverFeaturesResponse, FeatureModule};
use async_trait::async_trait;

#[async_trait]
impl FeatureModule for CloudAgentPython {
    async fn discover_features(&self) -> Result<DiscoverFeaturesResponse> {
        let url = self
            .cloud_agent
            .create_url(vec!["discover-features/query"])?;

        self.cloud_agent
            .get::<DiscoverFeaturesResponse>(url, None)
            .await
    }
}
