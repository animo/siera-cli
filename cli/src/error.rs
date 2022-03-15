use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    CannotReadConfigurationFile,
    InvalidConfigurationPath,
    InvalidConfigurationStructure,
    InvalidEnvironment,
    NoAgentURLSupplied,
    UnqualAmountKeyValue,
    HomeNotFound,
    ConfigExists,
    OsUnknown,
    RequiredAttributes,
    _NoSubcommandSupplied(String),
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CannotReadConfigurationFile => write!(f, "Cannot not read configuration file. Try initializing first using: `aries-cli configuration initialize`."),
            Error::InvalidConfigurationPath => write!(f, "Invalid configuration path."),
            Error::InvalidEnvironment => write!(f, "Invalid environment."),
            Error::NoAgentURLSupplied => write!(f, "No agent URL supplied. Supply an agent URL either via `--agent-url` or see `aries-cli configuration --help` to learn about setting up an environment."),
            Error::UnqualAmountKeyValue => write!(f, "Supplies keys and values are not equal in size."),
            Error::HomeNotFound => write!(f, "Unable to find home directory."),
            Error::ConfigExists => write!(f, "Configuration file already exists."),
            Error::OsUnknown => write!(f, "Unknown operating system. Failed to detect OS as windows or unix."),
            Error::_NoSubcommandSupplied(subcommand) => write!(f, "No subcommand supplied for {}. Check `aries-cli {} --help for the available options.", subcommand, subcommand),
            Error::RequiredAttributes => write!(f, "Creating a schema requires at least one attribute. Please supply them via the --attributes flag."),
            Error::InvalidConfigurationStructure => write!(f, "Invalid configuration structure. Please make sure you have a valid configuration file."),
        }
    }
}
