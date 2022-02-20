use crate::error::Result;
use async_trait::async_trait;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schema {
    pub results: Value,
}

#[async_trait]
pub trait SchemaModule {
    /// Requests all the features from the cloudagent
    async fn schema(&self) -> Result<Schema>;
}

pub trait SchemaEndpoints {
    fn endpoint_schema(&self) -> Result<Url>;
    fn endpoint_schema_created(&self) -> Result<Url>;
    fn endpoint_schema_get_by_id(&self) -> Result<Url>;
}
