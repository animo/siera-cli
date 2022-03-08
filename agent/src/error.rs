use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    AuthorizationFailed,
    UnableToParseResponse,
    UrlDoesNotExist,
    UnknownResponseStatusCode(String),
    InternalServerError(u16),
    UnreachableUrl,
    HttpServiceUnavailable,
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AuthorizationFailed => write!(f, "Failed to authorize. Either the wrong or no api-key is provided."),
            Error::UnableToParseResponse => write!(f, "Unable to parse the response from the server. Is the cloudagent the correct version?"),
            Error::UrlDoesNotExist => write!(f, "Path does not exist on endpoint. This can happen when querying by id and the id is not valid."),
            Error::UnknownResponseStatusCode(msg) => write!(f, "Received unknown status code from the server. Endpoint is likely incorrect. If the endpoint is correct, please report this error at https://github.com/animo/aries-cli/issues/new \nAdditional info: {}", msg),
            Error::InternalServerError(status) => write!(f, "Internal Server Error (status code: {})!", status),
            Error::UnreachableUrl => write!(f, "Provided url is unreachable. Is the provided endpoint valid?"),
            Error::HttpServiceUnavailable => write!(f, "Cloudagent is currently unavailable. Are you sure the agent is online?")

        }
    }
}
