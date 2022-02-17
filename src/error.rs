use crate::utils::logger::Log;

/// Error types
pub enum Error {
    /// Did not supply any endpoint in either the config, default path or
    /// supplied path, or the ARG
    NoSuppliedEndpoint,

    /// Could not authenticate
    AuthenticationFailed,

    /// Invalid configuration path
    InvalidConfigurationPath,

    /// Environment is invalid
    InvalidEnvironment,

    /// Environment is invalid
    InvalidEndpoint,
}

/// Error handler for enum type
pub fn throw(error: Error) -> ! {
    let logger = Log {
        should_copy: false,
        suppress_output: false,
    };

    match error {
        Error::NoSuppliedEndpoint => logger.error("No endpoint is supplied. Check the `--endpoint` option of your configuration file for the `endpoint` key"),
        Error::AuthenticationFailed => logger.error("Authentication Failed"),
        Error::InvalidConfigurationPath => logger.error("Invalid configuration path. If none is supplied it will fallback to '~/.config/aries-cli/config.ini`"),
        Error::InvalidEnvironment => logger.error("Supplied Environment is invalid. If none is supplied it will fallback to `Default`"),
        Error::InvalidEndpoint => logger.error("Supplied endpoint is not a valid url. Please check if it is correct and includes a http(s):// prefix."),
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
