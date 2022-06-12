use crate::agent::CloudAgentPython;
use agent::error::Result;
use agent::modules::schema::{Schema, SchemaCreateOptions, SchemaModule, SchemasGetAllResponse};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Reponse from the cloudagent that contains the wrapped schema
#[derive(Serialize, Deserialize, Debug)]
struct Response {
    /// Schema wrapper
    schema: Schema,
}

#[async_trait]
impl SchemaModule for CloudAgentPython {
    async fn create(&self, options: SchemaCreateOptions) -> Result<Schema> {
        let url = self.create_url(vec!["schemas"])?;

        let body = json!({
          "attributes": options.attributes,
          "schema_name": options.name,
          "schema_version": options.version
        });

        Ok(self.post::<Response>(url, None, Some(body)).await?.schema)
    }

    async fn get_by_id(&self, id: String) -> Result<Schema> {
        let url = self.create_url(vec!["schemas", &id])?;
        Ok(self.get::<Response>(url, None).await?.schema)
    }

    async fn get_all(&self) -> Result<SchemasGetAllResponse> {
        let url = self.create_url(vec!["schemas", "created"])?;
        self.get(url, None).await
    }
}
