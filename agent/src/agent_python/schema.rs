use crate::{
    error::Result,
    modules::schema::{GetSchemaResponse, SchemaCreateOptions},
    utils::web::create_url,
};
use async_trait::async_trait;
use serde_json::json;

use crate::modules::schema::{Schema, SchemaModule};

use super::agent::CloudAgentPython;

#[async_trait]
impl SchemaModule for CloudAgentPython {
    async fn create(&self, options: SchemaCreateOptions) -> Result<String> {
        let url = create_url(vec![&self.cloud_agent.endpoint, "schemas"])?;

        let body = json!({
          "attributes": options.attributes,
          "schema_name": options.name,
          "schema_version": options.version
        });

        self.cloud_agent
            .post::<Schema>(url, None, Some(body))
            .await
            .map(|s| s.sent.schema_id)
    }

    async fn get_by_id(&self, id: String) -> Result<GetSchemaResponse> {
        let url = create_url(vec![&self.cloud_agent.endpoint, "schemas", &id])?;
        self.cloud_agent.get::<GetSchemaResponse>(url, None).await
    }

    async fn get_all(&self) -> Result<Schema> {
        let _url = create_url(vec![&self.cloud_agent.endpoint, "schemas", "created"])?;
        todo!()
    }
}
