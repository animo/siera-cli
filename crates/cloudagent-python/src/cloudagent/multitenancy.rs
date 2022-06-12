use crate::agent::CloudAgentPython;
use agent::modules::multitenancy::MultitenancyModule;
use agent::{error::Result, modules::multitenancy::MultitenancyCreateResponse};
use async_trait::async_trait;
use serde_json::{json, Value};

#[async_trait]
impl MultitenancyModule for CloudAgentPython {
    /// TODO: this only returns the wallet id for now
    async fn create(&self) -> Result<MultitenancyCreateResponse> {
        let url = self.create_url(vec!["multitenancy", "wallet"])?;

        self.post::<MultitenancyCreateResponse>(url, None, Some(json!({})))
            .await
    }

    async fn remove(&self, wallet_id: String) -> Result<()> {
        let url = self.create_url(vec!["multitenancy", "wallet", &wallet_id, "remove"])?;

        self.post::<Value>(url, None, None).await?;

        Ok(())
    }
}
