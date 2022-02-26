use crate::error::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Type for received schema object
#[derive(Debug, Clone, Deserialize)]
pub struct Schema {
    /// received value
    pub sent: CreateSchemaResponse,
}

/// Sub value of Schema
#[derive(Debug, Clone, Deserialize)]
pub struct CreateSchemaResponse {
    /// Schema metadata
    pub schema: Value,

    /// Id of the schema
    pub schema_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetSchemaResponse {
    pub schema: SchemaContent,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SchemaContent {
    ver: String,
    id: String,
    name: String,
    version: String,
    #[serde(rename(deserialize = "attrNames"))]
    attr_names: Vec<String>,
    #[serde(rename(deserialize = "seqNo"))]
    seq_no: i32,
}

#[derive(Debug)]
pub struct SchemaCreateOptions {
    pub name: String,
    pub version: String,
    pub attributes: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetAllSchemasResponse {
    pub schema_ids: Vec<String>,
}

#[async_trait]
pub trait SchemaModule {
    /// Requests all the features from the cloudagent
    async fn create(&self, options: SchemaCreateOptions) -> Result<String>;
    async fn get_by_id(&self, id: String) -> Result<GetSchemaResponse>;
    async fn get_all(&self) -> Result<GetAllSchemasResponse>;
}
