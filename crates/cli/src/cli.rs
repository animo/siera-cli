use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::help_strings::HelpStrings;

use crate::modules::automation::AutomationOptions;
use crate::modules::multitenancy::MultitenancyOptions;
use crate::modules::{
    basic_message::BasicMessageOptions, configuration::ConfigurationOptions,
    connection::ConnectionOptions, credential::CredentialOptions,
    credential_definition::CredentialDefinitionOptions, feature::FeaturesOptions,
    json_ld::JsonLdOptions, oob::OobOptions, proof::ProofOptions, schema::SchemaOptions,
    wallet::WalletOptions, webhook::WebhookOptions,
};

/// Main command with options, flags and subcommands
#[derive(Parser)]
#[clap(name = "siera", author, version, about = HelpStrings::Cli)]
#[clap(arg_required_else_help = true, disable_help_subcommand = true)]
pub struct Cli {
    /// The agent url used for commandos
    #[clap(long, short='u', help = HelpStrings::AgentURL)]
    pub agent_url: Option<String>,

    /// The api key used for agent authentication
    #[clap(long, short, help = HelpStrings::ApiKey)]
    pub api_key: Option<String>,

    /// The multi tenancy token
    #[clap(long, short = 't', help = HelpStrings::ConfigurationInitializeToken)]
    pub token: Option<String>,

    /// The agent type
    #[clap(long, short = 'f', help = HelpStrings::Agent)]
    pub agent: Option<String>,

    /// Whether specific output should be copied to the clipboard
    #[clap(long, short, help = HelpStrings::Copy)]
    pub copy: bool,

    /// Whether the output should be quiet
    #[clap(long, short, help = HelpStrings::Quiet, conflicts_with = "verbose", conflicts_with = "json")]
    pub quiet: bool,

    /// Whether the output should be quiet
    #[clap(long, short = 'j', help = HelpStrings::Quiet, conflicts_with = "verbose", conflicts_with = "quiet")]
    pub json: bool,

    /// Which config path to use instead of the default one
    #[clap(long, short = 'o', help = HelpStrings::Config)]
    pub config: Option<PathBuf>,

    /// The environment which to use
    #[clap(long, short, default_value = "default", help = HelpStrings::Environment)]
    pub environment: String,

    /// Whether more verbose output should be printed
    #[clap(long, short='v', help = HelpStrings::Verbose, parse(from_occurrences), conflicts_with = "quiet", conflicts_with = "json")]
    pub verbose: usize,

    /// The main cli subcommands
    #[clap(subcommand)]
    pub commands: Commands,
}

/// All the subcommands
#[derive(Subcommand)]
pub enum Commands {
    /// Automation subcommands
    Automate(AutomationOptions),

    /// Connection subcommands
    Connection(ConnectionOptions),

    /// Configuration subcommands
    Configuration(ConfigurationOptions),

    /// Credential subcommands
    Credential(CredentialOptions),

    /// Credential definition subcommands
    CredentialDefinition(CredentialDefinitionOptions),

    /// Feature subcommands
    Feature(FeaturesOptions),

    /// JSON-LD subcommands
    JsonLd(JsonLdOptions),

    /// BasicMessage subcommands
    Message(BasicMessageOptions),

    /// Multitenancy subcommands
    Multitenancy(MultitenancyOptions),

    /// Oob subcommands
    Oob(OobOptions),

    /// Proof subcommands
    Proof(ProofOptions),

    /// Schema subcommands
    Schema(SchemaOptions),

    /// Wallet subcommands
    Wallet(WalletOptions),

    /// Webhook subcommands
    Webhook(WebhookOptions),
}

impl From<Commands> for String {
    fn from(c: Commands) -> Self {
        let s = match c {
            Commands::Automate(_) => "Automate",
            Commands::Configuration(_) => "Configuration",
            Commands::Connection(_) => "Connection",
            Commands::Credential(_) => "Credential",
            Commands::CredentialDefinition(_) => "CredentialDefinition",
            Commands::Feature(_) => "Feature",
            Commands::JsonLd(_) => "JsonLd",
            Commands::Message(_) => "Message",
            Commands::Multitenancy(_) => "Multitenancy",
            Commands::Oob(_) => "Oob",
            Commands::Proof(_) => "Proof",
            Commands::Schema(_) => "Schema",
            Commands::Wallet(_) => "Wallet",
            Commands::Webhook(_) => "Webhook",
        };

        Self::from(s)
    }
}
