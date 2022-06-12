use crate::agent::CloudAgentAfjRest;
use agent::error::Result;
use agent::modules::schema::{
    SchemaCreateOptions, SchemaCreateResponse, SchemaGetResponse, SchemaModule,
    SchemasGetAllResponse,
};
use async_trait::async_trait;
use serde_json::json;

#[async_trait]
impl SchemaModule for CloudAgentAfjRest {
    async fn create(&self, options: SchemaCreateOptions) -> Result<SchemaCreateResponse> {
        let url = self.create_url(vec!["schemas"])?;

        let body = json!(options);

        self.post(url, None, Some(body)).await
    }

    async fn get_by_id(&self, id: String) -> Result<SchemaGetResponse> {
        let url = self.create_url(vec!["schemas", &id])?;
        self.get(url, None).await
    }

    async fn get_all(&self) -> Result<SchemasGetAllResponse> {
        let url = self.create_url(vec!["schemas", "created"])?;
        self.get(url, None).await
    }
}
