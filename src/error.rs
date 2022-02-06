use crate::utils::logger::Log;

/// Error types
pub enum Error {
    /// Did not supply any endpoint in either the config, default path or
    /// supplied path, or the ARG
    NoSuppliedEndpoint,

    /// Response from the server could not be parsed
    ServerResponseParseError,

    /// Someting went wrong while sending the request
    //UnknownServerError,

    /// Something went wrong on the server-side
    //InternalServerError,

    /// Could not authenticate
    AuthenticationFailed,

    /// Supplied config path is invalid
    InvalidConfigPath,
}

/// Error handler for enum type
pub fn throw(error: Error) -> ! {
    let logger = Log {
        should_copy: false,
        suppress_output: false,
    };

    match error {
        Error::NoSuppliedEndpoint => logger.error("No Endpoint Supplied"),
        Error::AuthenticationFailed => logger.error("Authentication Failed"),
        Error::ServerResponseParseError => logger.error("Unable to parse response from server"),
        Error::InvalidConfigPath => logger.error("Supplied config path does not exist"),
        //Error::UnknownServerError => {
        //Log::error("Something went wrong while trying to reach the agent")
        //}
        //Error::InternalServerError => Log::error("Internal Server Error"),
    }
}

/// Error handler for Error type
pub fn throw_from_http(error: reqwest::Error) -> ! {
    let logger = Log {
        should_copy: false,
        suppress_output: false,
    };
    logger.error(&error.to_string());
}
