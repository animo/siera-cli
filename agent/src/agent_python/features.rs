use crate::error::Result;
use async_trait::async_trait;

use crate::{
    modules::features::{Features, FeaturesModule},
    utils::web::create_url,
};

use super::agent::CloudAgentPython;

#[async_trait]
impl FeaturesModule for CloudAgentPython {
    async fn discover_features(&self) -> Result<Features> {
        let url = create_url(vec![&self.cloud_agent.endpoint, "features"])?;

        self.cloud_agent.get::<Features>(url, None).await
    }
}
