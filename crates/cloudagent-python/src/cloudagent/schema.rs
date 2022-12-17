use crate::agent::CloudAgentPython;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use siera_agent::error::Result;
use siera_agent::modules::schema::{
    Schema, SchemaCreateOptions, SchemaModule, SchemasGetAllResponse,
};

/// Response from the cloudagent that contains the wrapped schema
#[derive(Serialize, Deserialize, Debug)]
struct Response {
    /// Schema wrapper
    schema: Schema,
}

#[async_trait]
impl SchemaModule for CloudAgentPython {
    async fn create(&self, options: SchemaCreateOptions) -> Result<Schema> {
        let url = self.create_url(&["schemas"])?;

        let body = json!({
          "attributes": options.attributes,
          "schema_name": options.name,
          "schema_version": options.version
        });

        Ok(self.post::<Response>(url, None, Some(body)).await?.schema)
    }

    async fn get_by_id(&self, id: String) -> Result<Schema> {
        let url = self.create_url(&["schemas", &id])?;
        Ok(self.get::<Response>(url, None).await?.schema)
    }

    async fn get_all(&self) -> Result<SchemasGetAllResponse> {
        let url = self.create_url(&["schemas", "created"])?;
        self.get(url, None).await
    }
}
