#![allow(clippy::missing_docs_in_private_items)]
use std::convert::From;

/// Help documentation for CLI commands.

pub enum HelpStrings {
    // Top level
    Cli,
    AgentURL,
    Agent,
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

    // Webhook
    Webhook,

    // OOB
    OobConnection,
    OobHandshakeProtocol,
    OobCreateInvitation,
    OobReceiveInvitation,
    OobReceiveUrl,
    OobInvite,
    OobReceive,
    Oob,
    OobInviteAlias,
    OobInviteMultiUse,
    OobInviteQr,
    OobInviteAutoAccept,

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
    ConnectionsListAlias,
    ConnectionsListConnectionProtocol,
    ConnectionsListInvitationKey,
    ConnectionsListMyDid,
    ConnectionsListState,
    ConnectionsListTheirDid,
    ConnectionsListTheirRole,
    ConnectionsReceive,
    ConnectionsReceiveUrl,

    // Credential Definitions
    CredentialDefinition,
    CredentialDefinitionId,
    CredentialDefinitionCreate,
    CredentialDefinitionCreateSchemaId,
    CredentialDefinitionCreateTag,
    CredentialDefinitionCreateSupportRevocation,
    CredentialDefinitionCreateRevocationRegistrySize,
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

    // Multitenancy
    Multitenancy,
    MultitenancyCreate,
    MultitenancyRemove,
    MultitenancyRemoveWalletId,

    // Automate
    Automation,
    // Offer a credential
    AutomationCredentialOffer,
    AutomationCredentialOfferConnectionId,
    AutomationCredentialOfferNoQr,
    AutomationCredentialOfferSelf,
    AutomationCredentialOfferTimeout,

    // Create credential definition
    AutomationCreateCredentialDefinitionName,
    AutomationCreateCredentialDefinitionAttributes,
    AutomationCreateCredentialDefinitionVersion,

    // Wallet
    Wallet,
    WalletCreate,
    WalletCreateMethod,
    WalletCreateOptions,
    WalletEndpoint,
    WalletEndpointType,
    WalletFetchDidEndpoint,
    WalletGetPublic,
    WalletList,
    WalletListDid,
    WalletListKeyType,
    WalletListMethod,
    WalletListPosture,
    WalletListVerkey,
    WalletRotateKeypair,
    WalletSetEndpoint,
    WalletSetPublic,
}

impl From<HelpStrings> for Option<&str> {
    fn from(help_string: HelpStrings) -> Option<&'static str> {
        Some(help_string.as_str())
    }
}

impl HelpStrings {
    const fn as_str(&self) -> &'static str {
        match self {
            Self::Cli => HELP_STRING_CLI,
            Self::AgentURL => "The Aries agent URL that requests will be sent to",
            Self::Agent => "Type of Aries agent (aca-py or afj) [default: aca-py]",
            Self::ApiKey => "This API key will be passed to the agent",
            Self::Copy => "Copy output to your clipboard",
            Self::Quiet => "Suppresses most output",
            Self::Verbose => "Print debug logs",
            Self::Config => "Supply a path to your configuration file to use that instead of the default",
            Self::Environment => "Specify your current environment",

            Self::Webhook => "Listen to webhook",
            Self::Configuration => "Add agents to your configuration or view your current configuration. To quickly get started run the following command: siera configuration add --default",
            Self::ConfigurationAdd => "Add a new, or overwrite an existing, agent your configuration file",
            Self::ConfigurationRemove => "PERMANENTLY remove an agent from your configuration",
            Self::ConfigurationRemoveEnvironment => "Environment to delete",
            Self::ConfigurationAddDefault => {
                "Add the default agent to the configuration (can be combined with --token)"
            }
            Self::ConfigurationView => "Print your current configuration file",
            Self::ConfigurationInitializeToken => "Authentication token for a multi tenancy agent",
            Self::Oob | Self::OobConnection => "Retrieve oob connections or create oob invitations",
            Self::OobHandshakeProtocol => "The handshake protocol to use. Defaults to did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/didexchange/1.0",
            Self::OobReceive | Self::OobReceiveInvitation => "Receive an oob invitation via url",
            Self::OobCreateInvitation => "Create an oob invitation",
            Self::OobInvite => "Create an Oob connection invite",
            Self::OobInviteAlias => {
                "The name a new oob connection will use to identify itself"
            }
            Self::OobInviteAutoAccept => {
                "Automatically accept the new oob connection once they accept this invitation"
            }
            Self::OobInviteMultiUse => "This oob invitation can be used more than once",
            Self::Connections => "Retrieve connections or create invitations",
            Self::ConnectionsId => "ID of connection to retrieve",
            Self::ConnectionsInvite => "Create a new connection invitation",
            Self::ConnectionsInviteAlias => {
                "The name a new connection will use to identify itself"
            }
            Self::ConnectionsInviteAutoAccept => {
                "Automatically accept the new connection once they accept this invitation"
            }
            Self::ConnectionsInviteMultiUse => "This invitation can be used more than once",
            Self::OobInviteQr | Self::ConnectionsInviteQr => {
                "Print a QR code, convenient for use with mobile apps"
            }
            Self::ConnectionsInviteToolbox => HELP_STRING_CONNECTIONS_INVITE_TOOLBOX,
            Self::ConnectionsList => "List all your current connections",
            Self::ConnectionsListId => "Get a connection by id",
            Self::ConnectionsListAlias => "Filter connections on the `alias` property",
            Self::ConnectionsListConnectionProtocol => "Filter connections on the `connection_protocol` property",
            Self::ConnectionsListInvitationKey => "Filter connections on the `invitation_key` property",
            Self::ConnectionsListMyDid => "Filter connections on the `my_did` property",
            Self::ConnectionsListState => "Filter connections on the `state` property",
            Self::ConnectionsListTheirDid => "Filter connections on the `their_did` property",
            Self::ConnectionsListTheirRole => "Filter connections on the `their_role` property",
            Self::ConnectionsReceive => "Receive an invitation via url",
            Self::OobReceiveUrl | Self::ConnectionsReceiveUrl => "The url that contains the invitation, surrounded by quotes",

            Self::CredentialDefinition => "Retrieve or create credential definitions",
            Self::CredentialDefinitionId => "ID of a credential definition to retrieve",
            Self::CredentialDefinitionCreate => "Create a new credential definition",
            Self::CredentialDefinitionCreateSchemaId => "Schema ID to use in the definition",
            Self::CredentialDefinitionCreateTag => "Tag for the credential definition",
            Self::CredentialDefinitionCreateSupportRevocation => "Whether the credential definition should support revocation",
            Self::CredentialDefinitionCreateRevocationRegistrySize => "The size of the revocation registry",
            Self::CredentialDefinitionList => "List all your credential definitions",

            Self::Credentials => "Issue Credential V1",
            Self::CredentialsOffer => "Offer a new credential to an existing connection",
            Self::CredentialsOfferConnectionId => {
                "Existing connection ID to offer the credential to"
            }
            Self::CredentialsOfferCredentialDefinitionId => {
                "A credential definition to base the credential on"
            }
            Self::CredentialsOfferKey => "An attribute key name",
            Self::CredentialsOfferValue => "An attribute value",
            Self::CredentialsPropose => "Not implemented yet: propose a credential that should be offered to you",
            Self::CredentialsProposeId => "Not implemented yet: connection ID to send proposal to",

            Self::Features => "List all available features",

            Self::Message => "Send a secure message to an existing connection",
            Self::MessageId => "Connection ID to send the message to",
            Self::MessageMessage => "Contents of the message",

            Self::Schema => "Retrieve or create schemas",
            Self::SchemaId => "ID of the schema to retrieve",
            Self::SchemaCreate => "Create a new schema",
            Self::SchemaCreateName => "Name of the schema",
            Self::SchemaCreateVersion => "Version of of the schema, useful to be able to specify multiple versions of the same schema",
            Self::SchemaCreateAttributes => "Keys that describe the structure of the schema - for example \"age\". Given in the following format: -a foo -a bar -a baz",
            Self::SchemaList => "List all your current schemas",

            Self::Proof => "Present proof V1",
            Self::ProofRequest => "Request a proof by connection id",
            Self::ProofRequestName => "Name of the proof request",
            Self::ProofRequestAttribute => "Attribute required in the proof request. e.g. -a=name -a=lastname",
            Self::ProofRequestPredicate => "Predicates required in the proof request (format = name,operator,value). e.g. -p=\"age,>=,18\"",
            Self::ProofRequestConnectionId => "Connection id to send the proof request to",

            Self::Automation => "Run a set of actions against the agent",
            Self::AutomationCredentialOffer => "Offer a premade credential to an agent",
            Self::AutomationCredentialOfferConnectionId => "Connection id of the receiving party",
            Self::AutomationCredentialOfferNoQr => "Do not show a QR code",
            Self::AutomationCredentialOfferSelf => "Offer a credential to self",
            Self::AutomationCredentialOfferTimeout=> "Timeout in seconds",
            Self::AutomationCreateCredentialDefinitionName => "Name of the schema the credential definition will be based on",
            Self::AutomationCreateCredentialDefinitionAttributes => "Attributes of the schema the credential definition will be based on",
            Self::AutomationCreateCredentialDefinitionVersion => "Version of the schema the credential definition will be based on",
            Self::Multitenancy => "Manage multiple agents",
            Self::MultitenancyCreate => "Create a new sub agent",
            Self::MultitenancyRemove => "Remove a sub agent",
            Self::MultitenancyRemoveWalletId => "Remove the wallet by id of a sub agent",

            Self::Wallet => "Interacts with a wallet",
            Self::WalletCreate => "Create a local DID",
            Self::WalletCreateMethod => "The did method. One of 'key' or 'sov'",
            Self::WalletCreateOptions => "Key types are e.g. ed25519, bls12381g2",
            Self::WalletEndpoint => "The endpoint url",
            Self::WalletEndpointType => "The endpoint type",
            Self::WalletFetchDidEndpoint => "Get the endpoint information associated with a DID",
            Self::WalletGetPublic => "Get the public DID of the wallet",
            Self::WalletList => "Query for DID associated with a wallet",
            Self::WalletListDid => "A DID to query for",
            Self::WalletListKeyType => "Key types are e.g. ed25519, bls12381g2",
            Self::WalletListMethod => "DID method to query for. e.g. sov to only fetch indy/sov DIDs Available values : key, sov",
            Self::WalletListPosture => "Whether DID is current public DID, posted to ledger but current public DID, or local to the wallet. Available values : public, posted, wallet_only",
            Self::WalletListVerkey => "The verification key of interest",
            Self::WalletRotateKeypair => "Rotate the keypair for a DID",
            Self::WalletSetEndpoint => "E.g. 'Endpoint'",
            Self::WalletSetPublic => "Set the public DID of the wallet",
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
----- Siera -----

To begin working with the siera, run the following command:

    $ siera configuration add --default

This command will initialize the configuration file and makes sure
that you do not have to pass the --agent-url argument with every call.

Some example commands are the following:

    $ siera connection list
        - fetches all the connections (jq compatible)
    $ siera connection invite --qr
        - create an invitation (as a qr code)
    $ siera feature
        - Fetches all the features of the cloudagent
    $ siera schema create --name FOO -a BAR -a BAZ
        - Create a new schema with the name as 'FOO' and the attributes as 'BAR' and 'BAZ'

-----------------
";
