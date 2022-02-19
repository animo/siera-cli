/// Error types
#[derive(Debug)]
pub enum Error {
    /// Could not authenticate
    AuthenticationFailed,

    /// Environment is invalid
    InvalidEndpoint,
}

pub type AgentResult<T> = Result<T, Error>;
