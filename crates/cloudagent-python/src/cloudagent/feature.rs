use crate::agent::CloudAgentPython;
use agent::error::Result;
use agent::modules::feature::{DiscoverFeaturesResponse, FeatureModule};
use async_trait::async_trait;

#[async_trait]
impl FeatureModule for CloudAgentPython {
    async fn discover_features(&self) -> Result<DiscoverFeaturesResponse> {
        let url = self.create_url(vec!["discover-features/query"])?;

        self.get::<DiscoverFeaturesResponse>(url, None).await
    }
}
