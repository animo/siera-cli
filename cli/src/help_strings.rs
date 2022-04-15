use std::convert::From;

/// Help documentation for CLI commands.

pub enum HelpStrings {
    // Top level
    Cli,
    AgentURL,
    ApiKey,
    Copy,
    Quiet,
    Verbose,
    Config,
    Environment,

    // Configuration
    Configuration,
    ConfigurationAdd,
    ConfigurationRemove,
    ConfigurationRemoveEnvironment,
    ConfigurationAddDefault,
    ConfigurationView,
    ConfigurationInitializeToken,

    // Connections
    Connections,
    ConnectionsId,
    ConnectionsInvite,
    ConnectionsInviteAutoAccept,
    ConnectionsInviteAlias,
    ConnectionsInviteMultiUse,
    ConnectionsInviteQr,
    ConnectionsInviteToolbox,
    ConnectionsList,
    ConnectionsListId,
    ConnectionsReceive,
    ConnectionsReceiveUrl,

    // Credential Definitions
    CredentialDefinition,
    CredentialDefinitionId,
    CredentialDefinitionCreate,
    CredentialDefinitionCreateSchemaId,
    CredentialDefinitionList,

    // Credentials
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

    // Message
    Message,
    MessageId,
    MessageMessage,

    // Schema
    Schema,
    SchemaId,
    SchemaList,
    SchemaCreate,
    SchemaCreateName,
    SchemaCreateVersion,
    SchemaCreateAttributes,

    // Proof
    Proof,
    ProofRequest,
    ProofRequestName,
    ProofRequestPredicate,
    ProofRequestAttribute,
    ProofRequestConnectionId,

    // Workflow
    Workflow,
    WorkflowCredentialOffer,
    WorkflowCredentialOfferConnectionId,
    WorkflowCredentialOfferNoQr,
    WorkflowCredentialOfferSelf,
    WorkflowCredentialOfferTimeout,
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
            HelpStrings::Config => "Supply a path to your configuration file to use that instead of the default",
            HelpStrings::Environment => "Specify your current environment",

            HelpStrings::Configuration => "Add agents to your configuration or view your current configuration. To quickly get started run the following command: aries-cli configuration add --default",
            HelpStrings::ConfigurationAdd => "Add a new, or overwrite an existing, agent your configuration file",
            HelpStrings::ConfigurationRemove => "PERMANENTLY remove an agent from your configuration",
            HelpStrings::ConfigurationRemoveEnvironment => "Environment to delete",
            HelpStrings::ConfigurationAddDefault => {
                "Add the default agent to the configuration (can be combined with --token)"
            }
            HelpStrings::ConfigurationView => "Print your current configuration file",
            HelpStrings::ConfigurationInitializeToken => "Authentication token for a multi tenancy agent",
            HelpStrings::Connections => "Retrieve connections or create invitations",
            HelpStrings::ConnectionsId => "ID of connection to retrieve",
            HelpStrings::ConnectionsInvite => "Create a new connection invitation",
            HelpStrings::ConnectionsInviteAlias => {
                "The name a new connection will use to identify itself"
            }
            HelpStrings::ConnectionsInviteAutoAccept => {
                "Automatically accept the new connection once they accept this invitation"
            }
            HelpStrings::ConnectionsInviteMultiUse => "This invitation can be used more than once",
            HelpStrings::ConnectionsInviteQr => {
                "Print a QR code, convenient for use with mobile apps"
            }
            HelpStrings::ConnectionsInviteToolbox => HELP_STRING_CONNECTIONS_INVITE_TOOLBOX,
            HelpStrings::ConnectionsList => "List all your current connections",
            HelpStrings::ConnectionsListId => "Get a connection by id",
            HelpStrings::ConnectionsReceive => "Receive an invitation via url",
            HelpStrings::ConnectionsReceiveUrl => "The url that contains the invitation, surrounded by quotes",

            HelpStrings::CredentialDefinition => "Retrieve or create credential definitions",
            HelpStrings::CredentialDefinitionId => "ID of a credential definition to retrieve",
            HelpStrings::CredentialDefinitionCreate => "Create a new credential definition",
            HelpStrings::CredentialDefinitionCreateSchemaId => "Schema ID to use in the definition",
            HelpStrings::CredentialDefinitionList => "List all your credential definitions",

            HelpStrings::Credentials => "Issue Credential V1",
            HelpStrings::CredentialsOffer => "Offer a new credential to an existing connection",
            HelpStrings::CredentialsOfferConnectionId => {
                "Existing connection ID to offer the credential to"
            }
            HelpStrings::CredentialsOfferCredentialDefinitionId => {
                "A credential definition to base the credential on"
            }
            HelpStrings::CredentialsOfferKey => "An attribute key name",
            HelpStrings::CredentialsOfferValue => "An attribute value",
            HelpStrings::CredentialsPropose => "Not implemented yet: propose a credential that should be offered to you",
            HelpStrings::CredentialsProposeId => "Not implemented yet: connection ID to send proposal to",

            HelpStrings::Features => "List all available features",

            HelpStrings::Message => "Send a secure message to an existing connection",
            HelpStrings::MessageId => "Connection ID to send the message to",
            HelpStrings::MessageMessage => "Contents of the message",

            HelpStrings::Schema => "Retrieve or create schemas",
            HelpStrings::SchemaId => "ID of the schema to retrieve",
            HelpStrings::SchemaCreate => "Create a new schema",
            HelpStrings::SchemaCreateName => "Name of the schema",
            HelpStrings::SchemaCreateVersion => "Version of of the schema, useful to be able to specify multiple versions of the same schema",
            HelpStrings::SchemaCreateAttributes => "Keys that describe the structure of the schema - for example \"age\". Given in the following format: -a foo -a bar -a baz",
            HelpStrings::SchemaList => "List all your current schemas",

            HelpStrings::Proof => "Present proof V1",
            HelpStrings::ProofRequest => "Request a proof by connection id",
            HelpStrings::ProofRequestName => "Name of the proof request",
            HelpStrings::ProofRequestAttribute => "Attribute required in the proof request. e.g. -a=name -a=lastname",
            HelpStrings::ProofRequestPredicate => "Predicates required in the proof request (format = name,operator,value). e.g. -p=\"age,>=,18\"",
            HelpStrings::ProofRequestConnectionId => "Connection id to send the proof request to",

            HelpStrings::Workflow => "Automated actions that combine multiple functions",
            HelpStrings::WorkflowCredentialOffer => "Simple credential offer automation to offer a premade credential to any agent",
            HelpStrings::WorkflowCredentialOfferConnectionId => "Connection id of the receiving party",
            HelpStrings::WorkflowCredentialOfferNoQr => "Disables printing the qr code",
            HelpStrings::WorkflowCredentialOfferSelf => "Completes the entire flow with itself",
            HelpStrings::WorkflowCredentialOfferTimeout=> "Timeout for the entire flow in seconds",
        }
    }
}

const HELP_STRING_CONNECTIONS_INVITE_TOOLBOX: &str =
    "Short-hand to create an invitation for the Aries Toolbox that sets:
    alias=\"toolbox\"
    multi-use=\"false\"
    auto-accept=\"true\"
    and gives admin rights over the invitation to the toolbox";

const HELP_STRING_CLI: &str = "
--- Aries cli ---

To begin working with the aries-cli, run the following command:

    $ aries-cli configuration add --default

This command will initialize the configuration file and makes sure
that you do not have to pass the --agent-url argument with every call.

Some example commands are the following:

    $ aries-cli connection list
        - fetches all the connections (jq compatible)
    $ aries-cli connection invite --qr
        - create an invitation (as a qr code)
    $ aries-cli features
        - Fetches all the features of the cloudagent
    $ aries-cli schema create --name FOO -a BAR -a BAZ
        - Create a new schema with the name as 'FOO' and the attributes as 'BAR' and 'BAZ'

-----------------
";
