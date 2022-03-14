use crate::error::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Create schema response
#[derive(Debug, Deserialize, Serialize)]
pub struct SchemaCreateResponse {
    /// Schema metadata
    pub schema: SchemaContent,

    /// Id of the schema
    pub schema_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SchemaGetResponse {
    pub schema: SchemaContent,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SchemaContent {
    ver: String,
    id: String,
    name: String,
    version: String,
    #[serde(rename(deserialize = "attrNames"))]
    attr_names: Vec<String>,
    #[serde(rename(deserialize = "seqNo"))]
    seq_no: Option<i32>,
}

#[derive(Debug)]
pub struct SchemaCreateOptions {
    pub name: String,
    pub version: String,
    pub attributes: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SchemasGetAllResponse {
    pub schema_ids: Vec<String>,
}

#[async_trait]
pub trait SchemaModule {
    /// Requests all the features from the cloudagent
    async fn create(&self, options: SchemaCreateOptions) -> Result<SchemaCreateResponse>;
    async fn get_by_id(&self, id: String) -> Result<SchemaGetResponse>;
    async fn get_all(&self) -> Result<SchemasGetAllResponse>;
}
