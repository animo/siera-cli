use crate::utils::logger::Log;

// Error types
pub enum Error {
    InvalidEndpoint,
    InvalidUrl,
    ServerResponseParseError,
    CannotCreateInvitation,
    ConnectionsUnretrieveable,
    ConnectionDoesNotExist,
}

// Error handler (Should not panic but print a custom error and exit)
pub fn throw(error: Error) -> ! {
    match error {
        // The endpoint in the configuration file is invalid
        Error::InvalidEndpoint => Log::error("Invalid Endpoint"),
        // The url created from the base + endpoint is invalid
        Error::InvalidUrl => Log::error("Invalid Url"),
        // Could not parse the response from the server
        Error::ServerResponseParseError => Log::error("Unable to parse response from server"),
        // The connection does not exist on the agent
        Error::ConnectionDoesNotExist => Log::error("Connection does not exist"),
        // The connection list is unretrieveable (Could this even happen?)
        Error::ConnectionsUnretrieveable => Log::error("Connection is unretrieveable"),
        // Could not create an invitation
        Error::CannotCreateInvitation => Log::error("Could not create an invitation"),
    }
}
