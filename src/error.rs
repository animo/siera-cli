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
}

// Error handler (Should not panic but print a custom error and exit)
pub fn throw(error: Error) -> ! {
    match error {
        // The path used for the configuration file is incorrect
        Error::InvalidRelativePath => panic!("Invalid Endpoint"),
        // The configuration file does not have the correct fields
        Error::InvalidConfigFile => panic!("Invalid Configuration file structure"),
        // The endpoint in the configuration file is invalid
        Error::InvalidEndpoint => panic!("Invalid Endpoint"),
        // The url created from the base + endpoint is invalid
        Error::InvalidUrl => panic!("Invalid Url"),
        // Could not parse the response from the server
        Error::ServerResponseParseError => panic!("Unable to parse response from server"),
        // The connection does not exist on the agent
        Error::ConnectionDoesNotExist => panic!("Connection does not exist"),
        // The connection list is unretrieveable (Could this even happen?)
        Error::ConnectionsUnretrieveable => panic!("Connection is unretrieveable"),
        // Could not create an invitation
        Error::CannotCreateInvitation => panic!("Could not create an invitation"),
        // Could not create a qr code for the invitation
        Error::CannotCreateQrCode => panic!("Could not create a qrcode for the invitation"),
    }
}
