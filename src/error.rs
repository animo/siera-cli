use crate::utils::logger::Log;

// Error types
pub enum Error {
    InvalidEndpoint,
    ServerResponseParseError,
    InternalServerError,
}

// Error handler (Should not panic but print a custom error and exit)
pub fn throw(error: Error) -> ! {
    match error {
        // The endpoint in the configuration file is invalid
        Error::InvalidEndpoint => Log::error("Invalid Endpoint"),
        // Could not parse the response from the server
        Error::ServerResponseParseError => Log::error("Unable to parse response from server"),
        // The server did not respond with a success code
        Error::InternalServerError => Log::error("Internal Server Error"),
    }
}
