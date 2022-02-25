use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    AuthorizationFailed(String),
    UnableToParseResponse(String),
    UrlDoesNotExist(String),
    InternalServerError(String),
    UnknownResponseStatusCode(String),
    UnreachableUrl(String),
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AuthorizationFailed(metadata) => write!(f, "Failed to authorize. Either the wrong or no api-key is provided. \n {metadata}"),
            Error::UnableToParseResponse(metadata) => write!(f, "Unable to parse the response from the server. Is the cloudagent the correct version? \n {metadata}"),
            Error::UrlDoesNotExist(metadata) => write!(f, "Path does not exist on endpoint. This can happen when querying by id and the id is not valid. \n {metadata}"),
            Error::InternalServerError(metadata) => write!(f, "Internal Server Error! \n {metadata}"),
            Error::UnknownResponseStatusCode(metadata) => write!(f, "Received unknown status code from the server. Endpoint is likely incorrect. If the endpoint is correct, please report this error. \n {metadata}"),
            Error::UnreachableUrl(metadata) => write!(f, "Provided url is unreachable. Is the provided endpoint valid? \n {metadata}"),
        }
    }
}
