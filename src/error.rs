use crate::utils::logger::Log;

/// Error types
pub enum Error {
    /// Did not supply any endpoint in either the config of the OPTION
    NoSuppliedEndpoint,

    /// Response from the server could not be parsed
    ServerResponseParseError,

    /// Someting went wrong while sending the request
    UnknownServerError,

    /// Something went wrong on the server-side
    InternalServerError,

    /// Could not authenticate
    AuthenticationFailed,
}

/// Error handler for enum type
pub fn throw(error: Error) -> ! {
    match error {
        Error::NoSuppliedEndpoint => Log::error("No Endpoint Supplied"),
        Error::AuthenticationFailed => Log::error("Authentication Failed"),
        Error::ServerResponseParseError => Log::error("Unable to parse response from server"),
        Error::UnknownServerError => {
            Log::error("Something went wrong while trying to reach the agent")
        }
        Error::InternalServerError => Log::error("Internal Server Error"),
    }
}

/// Error handler for Error type
pub fn throw_from_http(error: reqwest::Error) -> ! {
    Log::error(&error.to_string());
}
