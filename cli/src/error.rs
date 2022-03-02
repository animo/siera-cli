use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    InvalidConfigurationPath,
    InvalidEnvironment,
    NoEndpointSupplied,
    NoConfigKey,
    UnqualAmountKeyValue,
    ConfigExists,
    UnreachableCode,
    RequiredAttributes,
    NoSubcommandSupplied(String),
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidConfigurationPath => write!(f, "Invalid configuration path."),
            Error::InvalidEnvironment => write!(f, "Invalid environment."),
            Error::NoEndpointSupplied => write!(f, "No endpoint supplied. Supply an endpoint either via `--endpoint` or via `--config`."),
            Error::NoConfigKey => write!(f, "Required key does not exist in the configuration file."),
            Error::UnqualAmountKeyValue => write!(f, "Supplies keys and values are not equal in size."),
            Error::ConfigExists => write!(f, "Configuration file already exists."),
            Error::UnreachableCode => write!(f, "Unreachable code detected! Please report this issue with the command that caused it."),
            Error::NoSubcommandSupplied(subcommand) => write!(f, "No subcommand supplied for {}. check --help for the available options.", subcommand),
            Error::RequiredAttributes => write!(f, "Creating a schema requires at least one attribute. Please supply them via the --attributes flag."),
            
        }
    }
}
