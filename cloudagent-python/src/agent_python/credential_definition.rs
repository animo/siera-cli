use super::agent::CloudAgentPython;
use agent::error::Result;
use agent::modules::credential_definition::{
    CredentialDefinitionCreateResponse, CredentialDefinitionGetAllResponse,
    CredentialDefinitionGetByIdResponse, CredentialDefinitionModule,
};
use async_trait::async_trait;
use serde_json::json;

#[async_trait]
impl CredentialDefinitionModule for CloudAgentPython {
    async fn create(&self, schema_id: String) -> Result<CredentialDefinitionCreateResponse> {
        let url = self
            .cloud_agent
            .create_url(vec!["credential-definitions"])?;

        let body = json!({
          "schema_id": schema_id,
        });

        self.cloud_agent.post(url, None, Some(body)).await
    }

    async fn get_by_id(&self, id: String) -> Result<CredentialDefinitionGetByIdResponse> {
        let url = self
            .cloud_agent
            .create_url(vec!["credential-definitions", &id])?;
        self.cloud_agent.get(url, None).await
    }

    async fn get_all(&self) -> Result<CredentialDefinitionGetAllResponse> {
        let url = self
            .cloud_agent
            .create_url(vec!["credential-definitions", "created"])?;
        self.cloud_agent.get(url, None).await
    }
}
