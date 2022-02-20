use crate::error::Result;
use async_trait::async_trait;

use crate::modules::schema::{Schema, SchemaEndpoints, SchemaModule};

use super::agent::CloudAgentPython;

impl SchemaEndpoints for CloudAgentPython {
    fn endpoint_schema(&self) -> Result<reqwest::Url> {
        todo!()
    }

    fn endpoint_schema_created(&self) -> Result<reqwest::Url> {
        todo!()
    }

    fn endpoint_schema_get_by_id(&self) -> Result<reqwest::Url> {
        todo!()
    }
}

#[async_trait]
impl SchemaModule for CloudAgentPython {
    async fn schema(&self) -> Result<Schema> {
        todo!()
    }
}
