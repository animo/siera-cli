/// Error types
#[derive(Debug)]
pub enum Error {
    /// Did not supply any endpoint in either the config, default path or
    /// supplied path, or the ARG
    NoSuppliedEndpoint,

    /// Could not authenticate
    AuthenticationFailed,

    /// Environment is invalid
    InvalidEndpoint,
}

pub type AgentResult<T> = Result<T, Error>;
