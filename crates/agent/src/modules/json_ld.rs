use crate::error::Result;
use async_trait::async_trait;
use serde_json::Value;

/// Options that are supplied when signing a JSON-LD document
pub struct JsonLdSignOptions {
    /// TODO: enum/type content
    /// The doc to be signed
    pub doc: Value,

    /// The verkey
    pub verkey: String,
}

/// Options that are supplied when verifying a JSON-LD document
pub struct JsonLdVerifyOptions {
    /// TODO: enum/type content
    /// The doc to be signed
    pub doc: Value,

    /// The verkey
    pub verkey: String,
}

/// The verify doc and error
pub struct JsonLdVerifyDocResult {
    /// Boolean indicating whether the passed doc is valid or not
    valid: bool,

    /// The error if there is one
    error: Option<String>,
}

/// The signed doc and error
pub struct JsonLdSignDocResult {
    /// The signed doc in JSON
    doc: Value,

    /// The error if there is one
    error: Option<String>,
}

/// Generic cloudagent basic message module
#[async_trait]
pub trait JsonLd {
    /// sign a JSON-LD document
    /// TODO: return type
    async fn sign(&self, options: JsonLdSignOptions) -> Result<(JsonLdVerifyDocResult)>;

    /// verify a JSON-LD document
    /// TODO: return type
    async fn verify(&self, options: JsonLdVerifyOptions) -> Result<(JsonLdVerifyDocResult)>;
}
