use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::help_strings::HelpStrings;

use crate::modules::automation::AutomationOptions;
use crate::modules::multitenancy::MultitenancyOptions;
use crate::modules::{
    basic_message::BasicMessageOptions, configuration::ConfigurationOptions,
    connection::ConnectionOptions, credential::CredentialOptions,
    oob::OobOptions,
    webhooks::WebhooksOptions,
    credential_definition::CredentialDefinitionOptions, feature::FeaturesOptions,
    proof::ProofOptions, schema::SchemaOptions,
};

/// Main command with options, flags and subcommands
#[derive(Parser)]
#[clap(name = "agent-cli", author, version, about = HelpStrings::Cli)]
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
    #[clap(long, short, help = HelpStrings::Quiet, conflicts_with = "verbose")]
    pub quiet: bool,

    /// Which config path to use instead of the default one
    #[clap(long, short = 'o', help = HelpStrings::Config)]
    pub config: Option<PathBuf>,

    /// The environment which to use
    #[clap(long, short, default_value = "default", help = HelpStrings::Environment)]
    pub environment: String,

    /// Whether more verbose output should be printed
    #[clap(long, short='v', help = HelpStrings::Verbose, parse(from_occurrences), conflicts_with = "quiet")]
    pub verbose: usize,

    /// The main cli subcommands
    #[clap(subcommand)]
    pub commands: Commands,
}

/// All the subcommands
#[derive(Subcommand)]
pub enum Commands {
    /// Connection subcommands
    Connection(ConnectionOptions),
    
    /// Webhooks subcommands
    Webhooks(WebhooksOptions),
    
    /// Oob subcommands
    Oob(OobOptions),

    /// Feature subcommands
    Feature(FeaturesOptions),

    /// Schema subcommands
    Schema(SchemaOptions),

    /// Credential definition subcommands
    CredentialDefinition(CredentialDefinitionOptions),

    /// BasicMessage subcommands
    Message(BasicMessageOptions),

    /// Credential subcommands
    Credential(CredentialOptions),

    /// Configuration subcommands
    Configuration(ConfigurationOptions),

    /// Automation subcommands
    Automate(AutomationOptions),

    /// Proof subcommands
    Proof(ProofOptions),

    /// Multitenancy subcommands
    Multitenancy(MultitenancyOptions),
}

impl From<Commands> for String {
    fn from(c: Commands) -> Self {
        let s = match c {
            Commands::Automate(_) => "Automate",
            Commands::Connection(_) => "Connection",
            Commands::Webhooks(_) => "Webhooks",
            Commands::Oob(_) => "Oob",
            Commands::Feature(_) => "Feature",
            Commands::Schema(_) => "Schema",
            Commands::CredentialDefinition(_) => "CredentialDefinition",
            Commands::Message(_) => "Message",
            Commands::Credential(_) => "Credential",
            Commands::Configuration(_) => "Configuration",
            Commands::Proof(_) => "Proof",
            Commands::Multitenancy(_) => "Multitenancy",
        };

        Self::from(s)
    }
}
