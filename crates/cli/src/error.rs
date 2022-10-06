use std::fmt::{Display, Formatter};

/// User-level errors that can be thrown at runtime
#[derive(Debug)]
pub enum Error {
    /// Unable to read the configuration file. Might not be initialized
    CannotReadConfigurationFile,

    /// The path to the file does not exist
    InvalidConfigurationPath,

    /// The structure inside the configuration file is invalid
    InvalidConfigurationStructure,

    /// The specified environment does not exist inside the configuration file
    InvalidEnvironment(String),

    /// The connection to which to send something is not in state active
    InactiveConnection,

    /// No agent url was supplied to the command
    /// Either via the configuration or as an option
    NoAgentURLSupplied,

    /// No environment was supplied while one was required
    NoEnvironmentSupplied,

    /// The invitation is unparseable
    InvalidAgentInvitation,

    /// The key-value pair supplied cannot be indexed matched
    /// as the lengths differ
    UnequalAmountKeyValue,

    /// The environment variable `$HOME` cannot be found
    HomeNotFound,

    /// Unknown OS detected, we only actively support:
    /// - Linux
    /// - MacOS
    /// - Windows
    OsUnknown,

    /// Atleast one attribute is required when registering a schema
    RequiredAttributes,

    /// The configuration is empty
    EmptyConfiguration,

    /// The agent flag was invalid and should be: aca-py or afj
    InvalidAgent(String),

    /// The subcommand is not registered for the specified agent
    SubcommandNotRegisteredForAgent(String, &'static str),

    /// The compare value supplied cannot be parsed into a number
    PredicateValueNonNumber(String, String),
}

impl std::error::Error for Error {}

/// Generic result type which binds the error to be an instance of the `Error` enum
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CannotReadConfigurationFile => write!(f, "Cannot not read configuration file. Try initializing first using: `siera configuration add --default`."),
            Self::InvalidConfigurationPath => write!(f, "Invalid configuration path."),
            Self::InvalidEnvironment(env) => write!(f, "The environment {} does not exist.", env),
            Self::NoAgentURLSupplied => write!(f, "No agent URL supplied. Supply an agent URL either via `--agent-url` or see `siera configuration --help` to learn about setting up an environment."),
            Self::NoEnvironmentSupplied => write!(f, "No Environment supplied. Supply the environment either via `--environment` or see `siera configuration --help` to learn about setting up an environment."),
            Self::UnequalAmountKeyValue => write!(f, "Supplies keys and values are not equal in size."),
            Self::HomeNotFound => write!(f, "Unable to find home directory."),
            Self::OsUnknown => write!(f, "Unknown operating system. Failed to detect OS as windows or unix."),
            Self::RequiredAttributes => write!(f, "Creating a schema requires at least one attribute. Please supply them via the --attributes flag."),
            Self::InvalidConfigurationStructure => write!(f, "Invalid configuration structure. Please make sure you have a valid configuration file."),
            Self::InvalidAgentInvitation => write!(f, "The supplied agent url is incorrect. Make sure it contains the `c_i` query parameter and that the invitation part is correctly base64 encoded."),
            Self::InactiveConnection => write!(f, "The connection was not activated within the specified time. Please try again with a higher --timeout."),
            Self::EmptyConfiguration => write!(f, "Unable to delete from an empty configuration"),
            Self::PredicateValueNonNumber(name, val) => write!(f, "Predicate value {}, for name {}, is not of type number.", val, name),
            Self::InvalidAgent(agent) => write!(f, "Invalid agent '{}' supplied. Choose one of the following: 'aca-py' or 'afj'. (aca-py is default)", agent),
            Self::SubcommandNotRegisteredForAgent(subcommand, agent) => write!(f, "Subcommand '{}' is not registered for {}.", subcommand, agent)
        }
    }
}
