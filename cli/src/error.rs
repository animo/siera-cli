use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    InvalidConfigurationPath,
    InvalidEnvironment,
    NoEndpointSupplied,
    NoConfigKey,
    UnqualAmountKeyValue,
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidConfigurationPath => write!(f, "Invalid configuration path"),
            Error::InvalidEnvironment => write!(f, "Invalid environment"),
            Error::NoEndpointSupplied => write!(f, "No endpoint supplied. Supply an endpoint either via `--endpoint` or via `--config`"),
            Error::NoConfigKey => write!(f, "Required key does not exist in the configuration file."),
            Error::UnqualAmountKeyValue => write!(f, "Supplies keys and values are not equal in size."),
        }
    }
}
