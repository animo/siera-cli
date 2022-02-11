use crate::utils::logger::Log;

/// Error types
pub enum Error {
    /// Did not supply any endpoint in either the config, default path or
    /// supplied path, or the ARG
    NoSuppliedEndpoint,

    /// Could not authenticate
    AuthenticationFailed,
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
