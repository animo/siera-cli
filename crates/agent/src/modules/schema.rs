use crate::error::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Schema response from the ledger
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Schema {
    /// Version of the schema
    pub ver: String,

    /// Id of the schema
    pub id: String,

    /// Name of the schema
    pub name: String,

    /// Version of the schema
    pub version: String,

    /// Names of the attributes registered with the schema
    #[serde(rename(deserialize = "attrNames"))]
    pub attr_names: Vec<String>,

    /// seqNo as on the ledger that is unique to each schema
    #[serde(rename(deserialize = "seqNo"))]
    pub seq_no: Option<i32>,
}

/// Options supplied by the frontend to create a schema
#[derive(Debug, Serialize)]
pub struct SchemaCreateOptions {
    /// The name of the schema
    pub name: String,

    /// The version of the schema
    pub version: String,

    /// The attributed that must be registered with the schema
    pub attributes: Vec<String>,
}

/// Response from the cloudagent when requesting every schema
/// the cloudagent has registered
#[derive(Debug, Deserialize, Serialize)]
pub struct SchemasGetAllResponse {
    /// List of all the ids of every schema that the cloudagent has registered
    pub schema_ids: Vec<String>,
}

/// Generic cloudagent schema module
#[async_trait]
pub trait SchemaModule {
    /// Create a schema on the ledger
    async fn create(&self, options: SchemaCreateOptions) -> Result<Schema>;

    /// Request the schema by the id
    async fn get_by_id(&self, id: String) -> Result<Schema>;

    /// Get all the registerd schemas
    async fn get_all(&self) -> Result<SchemasGetAllResponse>;
}
