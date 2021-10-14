use crate::utils::logger::Log;

// Error types
pub enum Error {
    InvalidEndpoint,
    InvalidUrl,
    InvalidRelativePath,
    InvalidConfigFile,
    ServerResponseParseError,
    ConnectionsUnretrieveable,
    ConnectionDoesNotExist,
    CannotCreateInvitation,
    CannotCreateQrCode,
    InvalidInvitationConfiguration,
    NoMediatorUrl,
}

// Error handler (Should not panic but print a custom error and exit)
pub fn throw(error: Error) -> ! {
    match error {
        // The path used for the configuration file is incorrect
        Error::InvalidRelativePath => {
            Log::error("Invalid configuration file path (it MUST be relative)")
        }
        // The configuration file does not have the correct fields
        Error::InvalidConfigFile => Log::error("Invalid Configuration file structure"),
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
        // Could not create a qr code for the invitation
        Error::CannotCreateQrCode => Log::error("Could not create a qrcode for the invitation"),
        // The json configuration is missing the invitation configuration which is required if the
        // connection id is not specified
        Error::InvalidInvitationConfiguration => {
            Log::error("Connection invitation config is invalid")
        }
        // No mediator url provided while create mediator subcommand is called
        Error::NoMediatorUrl => Log::error("No mediator url provided"),
    }
}
