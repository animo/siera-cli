use agent::error::Error;

pub fn get_error_string(error: Error) -> &'static str {
    match error {
        Error::AuthenticationFailed => "Incorrect api key.",
        Error::InvalidEndpoint => "Endpoint is invalid",
    }
}
