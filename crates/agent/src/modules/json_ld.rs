use crate::error::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Options that are supplied when signing a JSON-LD document
#[derive(Debug, Deserialize, Serialize)]
pub struct JsonLdSignOptions {
    /// TODO: enum/type content
    /// The doc to be signed
    pub doc: String,

    /// The verkey
    pub verkey: String,
}

/// Options that are supplied when verifying a JSON-LD document
#[derive(Debug, Deserialize, Serialize)]
pub struct JsonLdVerifyOptions {
    /// TODO: enum/type content
    /// The doc to be signed
    pub doc: String,

    /// The verkey
    pub verkey: String,
}

/// Generic cloudagent basic message module
#[async_trait]
pub trait JsonLdModule {
    /// sign a JSON-LD document
    /// TODO: return type
    async fn sign(&self, options: JsonLdSignOptions) -> Result<Value>;

    /// verify a JSON-LD document
    /// TODO: return type
    async fn verify(&self, options: JsonLdVerifyOptions) -> Result<bool>;
}
