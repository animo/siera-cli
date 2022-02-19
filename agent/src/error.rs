use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    AuthorizationFailed,
    UnableToParseResponse,
    UrlDoesNotExist,
    InternalServerError,
    UnknownResponseStatusCode,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Error::AuthorizationFailed => write!(
                f,
                "Failed to authorize. Either the wrong or no api-key is provided"
            ),
            Error::UnableToParseResponse => write!(f, "Unable to parse the response from the server. Is the cloudagent the correct version?"),
            Error::UrlDoesNotExist => write!(f, "Provided endpoint does not exist. Is the supplied endpoint a cloudagent?"),
            Error::InternalServerError => write!(f, "Internal Server Error!"),
            Error::UnknownResponseStatusCode => write!(f, "Received unknown status code from the server. Endpoint is likely incorrect. Please report this error."),
        }
    }
}
