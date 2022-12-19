use crate::agent::CloudAgentPython;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use siera_agent::{
    error::Result,
    modules::json_ld::{JsonLdModule, JsonLdSignOptions, JsonLdVerifyOptions},
};

/// The result of the verification
#[derive(Debug, Deserialize, Serialize)]
pub struct VerifyResult {
    /// The boolean whether the doc is valid
    pub valid: bool,

    /// The error if there si one
    pub error: Option<String>,
}

/// The result of the signing
#[derive(Debug, Deserialize, Serialize)]
pub struct SignResult {
    /// The boolean whether the doc is valid
    pub doc: Value,

    /// The error if there si one
    pub error: Option<String>,
}

#[async_trait]
impl JsonLdModule for CloudAgentPython {
    async fn sign(&self, options: JsonLdSignOptions) -> Result<Value> {
        let url = self.create_url(&["jsonld", "sign"])?;

        let _doc: Value = serde_json::from_str(&options.doc)?;
        let body = json!({
            "doc": _doc,
            "verkey": options.verkey
        });

        let signed_doc_result: SignResult = self.post(url, None, Some(body)).await?;

        // TODO: Aca-py still finishes with success but passes the error along
        //  Maybe better to throw if the dic fails to sign
        Ok(signed_doc_result.doc)
    }

    async fn verify(&self, options: JsonLdVerifyOptions) -> Result<bool> {
        let url = self.create_url(&["jsonld", "verify"])?;

        let _doc: Value = serde_json::from_str(&options.doc)?;
        let body = json!({
            "doc": _doc,
            "verkey": options.verkey
        });

        let verify_doc_result: VerifyResult = self.post(url, None, Some(body)).await?;

        // TODO: Aca-py still finishes with success but passes the error along
        //  Maybe better to throw if the dic fails verify
        Ok(verify_doc_result.valid)
    }
}
