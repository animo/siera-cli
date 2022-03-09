use super::agent::CloudAgentPython;
use agent::error::Result;
use agent::modules::features::{Features, FeaturesModule};
use async_trait::async_trait;

#[async_trait]
impl FeaturesModule for CloudAgentPython {
    async fn discover_features(&self) -> Result<Features> {
        let url = self
            .cloud_agent
            .create_url(vec!["discover-features/query"])?;

        self.cloud_agent.get::<Features>(url, None).await
    }
}
