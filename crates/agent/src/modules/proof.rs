use crate::error::{Error, Result};
use async_trait::async_trait;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::str::FromStr;

/// Response from the cloudagent when a proof request is created
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProofRequestResponse {
    /// The state of the proof request
    pub state: String,

    /// The presentation request as an object
    pub presentation_request: Value,

    /// The last time the proof request was updated
    pub updated_at: String,

    /// The connection id to which the proof request is send to
    pub connection_id: String,

    /// The thread id that can be used to reference this proof request
    pub thread_id: String,

    /// A dictionary of the presentation request
    pub presentation_request_dict: Value,

    /// Your role in the proof request flow
    pub role: String,

    /// Whether it should automatically respond in the proof request flow
    pub auto_present: bool,

    /// The presentation exhange id that can be used in the other presentation exchange steps
    pub presentation_exchange_id: String,

    /// When the proof request was created
    pub created_at: String,

    /// Who the initiator was of the proof request
    pub initiator: String,
}

/// A simple predicate enum
/// The first string is the name of the key/value pair
/// The second string is the operator that is used `>=`, `<=`, `=`, `>` or `>`
/// The third string is the the value that should be evaluated with
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

        validate_operator(operator).map_err(|_| Error::InvalidOperator(operator.to_owned()))?;
        Ok(Self(name.to_owned(), operator.to_owned(), value.to_owned()))
    }
}

/// Simple validation to check if the supplied operator is valid
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

/// Options supplied when a proof request is created
pub struct ProofRequestOptions {
    /// The connection id to which the proof request is send to
    pub connection_id: String,

    /// The name of the proof request
    pub name: String,

    /// All the attributes that are requested from the other agent
    pub attributes: Vec<String>,

    /// All the predicates that are requested from the other agent
    pub predicates: Vec<(String, String, i32)>,
}

/// Generic cloudagent proof module
#[async_trait]
pub trait ProofModule {
    /// Send a proof request via the connection id to another agent
    async fn send_request(&self, options: ProofRequestOptions) -> Result<ProofRequestResponse>;
}
