use crate::error::{Error, Result};
use async_trait::async_trait;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::str::FromStr;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProofRequestResponse {
    pub state: String,
    #[serde(rename = "presentation_request")]
    pub presentation_request: Value,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "connection_id")]
    pub connection_id: String,
    #[serde(rename = "thread_id")]
    pub thread_id: String,
    #[serde(rename = "presentation_request_dict")]
    pub presentation_request_dict: Value,
    pub role: String,
    #[serde(rename = "auto_present")]
    pub auto_present: bool,
    #[serde(rename = "presentation_exchange_id")]
    pub presentation_exchange_id: String,
    pub trace: bool,
    #[serde(rename = "created_at")]
    pub created_at: String,
    pub initiator: String,
}

#[derive(Debug)]
pub struct Predicate(pub String, pub String, pub String);

impl FromStr for Predicate {
    type Err = Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut res = s.split(',');
        let name = res
            .next()
            .ok_or_else(|| Error::UnableToParseOutValue(s.to_owned()))?;
        let operator = res
            .next()
            .ok_or_else(|| Error::UnableToParseOutValue(s.to_owned()))?;
        let value = res
            .next()
            .ok_or_else(|| Error::UnableToParseOutValue(s.to_owned()))?;

        validate_operator(operator).map_err(|_| Error::InvalidOperator(operator.to_string()))?;
        Ok(Predicate(
            name.to_string(),
            operator.to_string(),
            value.to_string(),
        ))
    }
}

fn validate_operator(op: &str) -> Result<()> {
    if vec![">=", "<=", "=", ">", "<"]
        .iter()
        .map(|o| String::from(*o))
        .any(|o| o == *op)
    {
        return Ok(());
    };
    Err(Error::InvalidOperator(op.to_owned()).into())
}

pub struct ProofRequestOptions {
    pub connection_id: String,
    pub name: String,
    pub attributes: Vec<String>,
    pub predicates: Vec<(String, String, i32)>,
}

#[async_trait]
pub trait ProofModule {
    async fn send_request(&self, options: ProofRequestOptions) -> Result<ProofRequestResponse>;
}
