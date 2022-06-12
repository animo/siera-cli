use crate::agent::CloudAgentPython;
use agent::error::Result;
use agent::modules::credential_definition::{
    CredentialDefinitionCreateOptions, CredentialDefinitionCreateResponse,
    CredentialDefinitionGetAllResponse, CredentialDefinitionGetByIdResponse,
    CredentialDefinitionModule,
};
use async_trait::async_trait;
use serde_json::json;

#[async_trait]
impl CredentialDefinitionModule for CloudAgentPython {
    async fn create(
        &self,
        options: CredentialDefinitionCreateOptions,
    ) -> Result<CredentialDefinitionCreateResponse> {
        let url = self.create_url(vec!["credential-definitions"])?;

        let body = json!(options);

        self.post(url, None, Some(body)).await
    }

    async fn get_by_id(&self, id: String) -> Result<CredentialDefinitionGetByIdResponse> {
        let url = self.create_url(vec!["credential-definitions", &id])?;
        self.get(url, None).await
    }

    async fn get_all(&self) -> Result<CredentialDefinitionGetAllResponse> {
        let url = self.create_url(vec!["credential-definitions", "created"])?;
        self.get(url, None).await
    }
}
