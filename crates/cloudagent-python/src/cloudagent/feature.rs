use crate::agent::CloudAgentPython;
use async_trait::async_trait;
use siera_agent::error::Result;
use siera_agent::modules::feature::{DiscoverFeaturesResponse, FeatureModule};

#[async_trait]
impl FeatureModule for CloudAgentPython {
    async fn discover_features(&self) -> Result<DiscoverFeaturesResponse> {
        let url = self.create_url(&["discover-features/query"])?;

        self.get::<DiscoverFeaturesResponse>(url, None).await
    }
}
