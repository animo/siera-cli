use super::agent::CloudAgentPython;
use agent::error::Result;
use agent::modules::schema::{
    SchemaCreateOptions, SchemaCreateResponse, SchemaGetResponse, SchemaModule,
    SchemasGetAllResponse,
};
use async_trait::async_trait;
use serde_json::json;

#[async_trait]
impl SchemaModule for CloudAgentPython {
    async fn create(&self, options: SchemaCreateOptions) -> Result<SchemaCreateResponse> {
        let url = self.cloud_agent.create_url(vec!["schemas"])?;

        let body = json!({
          "attributes": options.attributes,
          "schema_name": options.name,
          "schema_version": options.version
        });

        self.cloud_agent.post(url, None, Some(body)).await
    }

    async fn get_by_id(&self, id: String) -> Result<SchemaGetResponse> {
        let url = self.cloud_agent.create_url(vec!["schemas", &id])?;
        self.cloud_agent.get(url, None).await
    }

    async fn get_all(&self) -> Result<SchemasGetAllResponse> {
        let url = self.cloud_agent.create_url(vec!["schemas", "created"])?;
        self.cloud_agent.get(url, None).await
    }
}
