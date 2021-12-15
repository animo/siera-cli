use crate::utils::logger::Log;

/// Error types
pub enum Error {
    /// Did not supply any endpoint in either the config of the OPTION
    NoSuppliedEndpoint,
    
    /// Endpoint is incorrect
    InvalidEndpoint,

    /// Response from the server could not be parsed
    ServerResponseParseError,

    /// Something went wrong on the server-side
    InternalServerError,
}

/// Error handler (Should not panic but print a custom error and exit)
pub fn throw(error: Error) -> ! {
    match error {
        Error::NoSuppliedEndpoint => Log::error("No Endpoint Supplied"),
        Error::InvalidEndpoint => Log::error("Invalid Endpoint"),
        Error::ServerResponseParseError => Log::error("Unable to parse response from server"),
        Error::InternalServerError => Log::error("Internal Server Error"),
    }
}
