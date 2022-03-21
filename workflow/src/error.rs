use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    ConnectionNotReady,
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConnectionNotReady => write!(f, "Connection is not in state active"),
        }
    }
}
