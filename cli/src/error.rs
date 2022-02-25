use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    InvalidConfigurationPath(String),
    InvalidEnvironment(String),
    NoEndpointSupplied(String),
    NoConfigKey(String),
    UnqualAmountKeyValue(String),
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidConfigurationPath(metadata) => write!(f, "Invalid configuration path. \n {metadata}"),
            Error::InvalidEnvironment(metadata) => write!(f, "Invalid environment. \n {metadata}"),
            Error::NoEndpointSupplied(metadata) => write!(f, "No endpoint supplied. Supply an endpoint either via `--endpoint` or via `--config`. \n {metadata}"),
            Error::NoConfigKey(metadata) => write!(f, "Required key does not exist in the configuration file. \n {metadata}"),
            Error::UnqualAmountKeyValue(metadata) => write!(f, "Supplies keys and values are not equal in size. \n {metadata}"),
        }
    }
}
