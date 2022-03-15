use std::convert::From;

/// Help documentation for CLI commands.

pub enum HelpStrings {
    Cli,
    AgentURL,
    ApiKey,
    Copy,
    Quiet,
    Verbose,
    Config,
    Environment,
    ConfigurationInitialize,
    ConfigurationView,

    /// Connections
    Connections,
    ConnectionsInvite,
    ConnectionsInviteAutoAccept,
    ConnectionsInviteAlias,
    ConnectionsInviteMultiUse,
    ConnectionsInviteQr,
    ConnectionsInviteToolbox,

    /// Credential Definitions
    CredentialDefinition,
    CredentialDefinitionId,
    CredentialDefinitionCreate,
    CredentialDefinitionCreateSchemaId,

    /// Credentials
    Credentials,
    CredentialsOffer,
    CredentialsOfferCredentialDefinitionId,
    CredentialsOfferConnectionId,
    CredentialsOfferKey,
    CredentialsOfferValue,
    CredentialsPropose,
    CredentialsProposeId,

    // Features
    Features,
}

impl From<HelpStrings> for Option<&str> {
    fn from(help_string: HelpStrings) -> Option<&'static str> {
        Some(help_string.as_str())
    }
}

impl HelpStrings {
    fn as_str(&self) -> &'static str {
        match self {
            HelpStrings::Cli => HELP_STRING_CLI,
            HelpStrings::AgentURL => "The Aries agent URL that requests will be sent to",
            HelpStrings::ApiKey => "This API key will be passed to the agent",
            HelpStrings::Copy => "Copy output to your clipboard",
            HelpStrings::Quiet => "Suppresses most output",
            HelpStrings::Verbose => "Print debug logs",
            HelpStrings::Config => "Path to your configuration file",
            HelpStrings::Environment => "Specify your current environment",

            HelpStrings::ConfigurationInitialize => {
                "Initialize a new configuration file with a default environment"
            }
            HelpStrings::ConfigurationView => "Print your current configuration file",

            HelpStrings::Connections => "Retrieve connections or create invitations",
            HelpStrings::ConnectionsInvite => "Create a new connection invitation",
            HelpStrings::ConnectionsInviteAlias => {
                "The name a new connection will use to identify itself"
            }
            HelpStrings::ConnectionsInviteAutoAccept => {
                "Automatically accept the new connection once they accept this invitation"
            }
            HelpStrings::ConnectionsInviteMultiUse => {
                "Whether this invitation can be used more than once"
            }
            HelpStrings::ConnectionsInviteQr => {
                "Print a QR code, convenient for use with mobile apps"
            }
            // TODO: Find out what "toolbox" is
            HelpStrings::ConnectionsInviteToolbox => "????",

            HelpStrings::CredentialDefinition => "Retrieve or create credential definitions",
            HelpStrings::CredentialDefinitionId => {
                "Specify the ID of a credential definition to retrieve"
            }
            HelpStrings::CredentialDefinitionCreate => "Create a new credential definition",
            HelpStrings::CredentialDefinitionCreateSchemaId => "Specify the schema ID",

            HelpStrings::Credentials => "Offer or propose credentials",
            HelpStrings::CredentialsOffer => "Offer a new credential to an existing connection",
            HelpStrings::CredentialsOfferConnectionId => {
                "Specify the connection to offer the credential to"
            }
            HelpStrings::CredentialsOfferCredentialDefinitionId => {
                "Specify the credential definition to base the credential on"
            }
            // TODO: What dis?
            HelpStrings::CredentialsOfferKey => "????",
            // TODO: What dis?
            HelpStrings::CredentialsOfferValue => "????",
            // TODO: What dis?
            HelpStrings::CredentialsPropose => "????",
            HelpStrings::CredentialsProposeId => "????",

            HelpStrings::Features => "List all available features",
            // TODO: Add docs for all subcommands, e.g., ConnectionsAll.
        }
    }
}

const HELP_STRING_CLI: &str = "
--- Aries cli ---

To begin working with the aries-cli, run the following command:

    $ aries-cli configuration initialize

This command will initialize the configuration file and makes sure
that you do not have to pass the --agent-url argument with every call.

Some example commands are the following:

    $ aries-cli connections
        - fetches all the connections (jq compatible)
    $ aries-cli connections invite -qr
        - create an invitation (as a qr code)
    $ aries-cli features
        - Fetches all the features of the cloudagent
    $ aries-cli schema create --name FOO -a BAR -a BAZ
        - Create a new schema with the name as 'FOO' and the attributes as 'BAR' and 'BAZ'

-----------------
";
