use async_trait::async_trait;

use crate::modules::features::FeaturesModule;

use super::agent::CloudAgentPython;

#[async_trait]
impl FeaturesModule for CloudAgentPython {
    async fn discover_features(&self) -> () {
        todo!()
    }
}
