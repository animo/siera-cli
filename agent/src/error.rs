use std::fmt::Display;

/// Error types
#[derive(Debug)]
pub enum Error {
    /// Could not authenticate
    AuthenticationFailed,

    /// Environment is invalid
    InvalidEndpoint,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AuthenticationFailed => {
                write!(f, "Either no api key is provided or it is invalid")
            }
            Error::InvalidEndpoint => write!(f, "The endpoint that is supplied is invalid"),
        }
    }
}
pub type AgentResult<T> = Result<T, String>;
