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

/// Response from the cloudagent when requesting a schema
#[derive(Debug, Deserialize, Serialize)]
pub struct SchemaGetResponse {
    /// Schema metadata
    pub schema: SchemaContent,
}

/// The content of a schema object
#[derive(Debug, Deserialize, Serialize)]
pub struct SchemaContent {
    /// Version of the schema
    ver: String,

    /// Id of the schema
    id: String,

    /// Name of the schema
    name: String,

    /// Version of the schema
    version: String,

    /// Names of the attributes registered with the schema
    #[serde(rename(deserialize = "attrNames"))]
    pub attr_names: Vec<String>,

    /// seqNo as on the ledger that is unique to each schema
    #[serde(rename(deserialize = "seqNo"))]
    seq_no: Option<i32>,
}

/// Options supplied by the frontend to create a schema
#[derive(Debug)]
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
    async fn create(&self, options: SchemaCreateOptions) -> Result<SchemaCreateResponse>;

    /// Request the schema by the id
    async fn get_by_id(&self, id: String) -> Result<SchemaGetResponse>;

    /// Get all the registerd schemas
    async fn get_all(&self) -> Result<SchemasGetAllResponse>;
}
