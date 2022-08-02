use std::fmt::{Display, Formatter};

/// User-level errors that can be thrown at runtime
#[derive(Debug)]
pub enum Error {
    /// The cloudagent did not allow the request without proper authorization
    AuthorizationFailed,

    /// The server gave a response that was not expected and therefore not deserializeable
    UnableToParseResponse,

    /// Provided url does not exist
    UrlDoesNotExist,

    /// The server supplied a status code which is not handled accordingly
    UnknownResponseStatusCode(String),

    /// The server responded with a 5xx status code. Not our fault
    InternalServerError(u16),

    /// Supplied url is not reachable
    UnreachableUrl,

    /// Specific handle case for a 5xx status code which means that the cloudagent might be offline
    HttpServiceUnavailable,

    // TODO: why is this here?
    /// Predicate structure is invalid
    UnableToParseOutValue(String),

    // TODO: why is this here?
    /// Predicate used an invalid operator
    InvalidOperator(String),
}

impl std::error::Error for Error {}

/// Generic result type which binds the error to be an instance of the `Error` enum
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AuthorizationFailed => write!(f, "Failed to authorize. Api-key or authorization token is either wrong or missing."),
            Self::UnableToParseResponse => write!(f, "Unable to parse the response from the server. Is the cloudagent the correct version?"),
            Self::UrlDoesNotExist => write!(f, "Path does not exist on agent URL. This can happen when querying by id and the id is not valid or when attempting to use a feature that is not supported on the cloudagent."),
            Self::UnknownResponseStatusCode(msg) => write!(f, "Received unknown status code from the server. Agent URL is likely incorrect. If the agent URL is correct, please report this error at https://github.com/animo/agent-cli/issues/new \nAdditional info: {}", msg),
            Self::InternalServerError(status) => write!(f, "Internal Server Error (status code: {})!", status),
            Self::UnreachableUrl => write!(f, "Provided url is unreachable. Is the provided agent URL valid?"),
            Self::HttpServiceUnavailable => write!(f, "Cloudagent is currently unavailable. Are you sure the agent is online?"),
            Self::UnableToParseOutValue(val) => write!(f, "Unable to parse the predicate values from: {}. The following structure is required: (name,operator,value)", val),
            Self::InvalidOperator(op) => write!(f, "Invalid Operator ({}). \">=\", \"<=\", \"=\", \"<\" and \">\" are allowed.", op)
        }
    }
}
