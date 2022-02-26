use crate::{
    error::Result,
    modules::credential_definition::{
        CreateCredentialDefinitionResponse, GetCredentialDefinitionResponse, GetAllCredentialDefinitionsResponse,
    },
};
use async_trait::async_trait;
use serde_json::json;

use crate::modules::credential_definition::CredentialDefinitionModule;

use super::agent::CloudAgentPython;

#[async_trait]
impl CredentialDefinitionModule for CloudAgentPython {
    async fn create(&self, schema_id: String) -> Result<CreateCredentialDefinitionResponse> {
        let url = self
            .cloud_agent
            .create_url(vec!["credential-definitions"])?;

        let body = json!({
          "schema_id": schema_id,
        });

        self.cloud_agent.post(url, None, Some(body)).await
    }

    async fn get_by_id(&self, id: String) -> Result<GetCredentialDefinitionResponse> {
        let url = self
            .cloud_agent
            .create_url(vec!["credential-definitions", &id])?;
        self.cloud_agent.get(url, None).await
    }

    async fn get_all(&self) -> Result<GetAllCredentialDefinitionsResponse> {
        let url = self
            .cloud_agent
            .create_url(vec!["credential-definitions", "created"])?;
        self.cloud_agent.get(url, None).await
    }
}
