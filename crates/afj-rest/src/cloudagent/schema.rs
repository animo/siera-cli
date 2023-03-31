use crate::agent::CloudAgentAfjRest;
use async_trait::async_trait;
use serde_json::json;
use siera_agent::error::{Error, Result};
use siera_agent::modules::schema::{
    Schema, SchemaCreateOptions, SchemaModule, SchemasGetAllResponse,
};

#[async_trait]
impl SchemaModule for CloudAgentAfjRest {
    async fn create(&self, options: SchemaCreateOptions) -> Result<Schema> {
        let url = self.create_url(&["schemas"])?;

        let body = json!(options);

        self.post(url, None, Some(body)).await
    }

    async fn get_by_id(&self, id: String) -> Result<Schema> {
        let url = self.create_url(&["schemas", &id])?;

        self.get(url, None).await
    }

    async fn get_all(&self) -> Result<SchemasGetAllResponse> {
        Err(Error::CommandNotAvailable(format!("{self}")).into())
    }
}
