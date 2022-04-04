use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::help_strings::HelpStrings;

use crate::modules::workflow::WorkflowOptions;
use crate::modules::{
    configuration::ConfigurationOptions, connection::ConnectionOptions,
    credential::CredentialOptions, credential_definition::CredentialDefinitionOptions,
    feature::FeaturesOptions, message::MessageOptions, schema::SchemaOptions,
};

#[derive(Parser)]
#[clap(name = "aries-cli", author, version, about = HelpStrings::Cli)]
#[clap(arg_required_else_help = true, disable_help_subcommand = true)]
pub struct Cli {
    #[clap(long, short='u', help = HelpStrings::AgentURL)]
    pub agent_url: Option<String>,

    #[clap(long, short, help = HelpStrings::ApiKey)]
    pub api_key: Option<String>,

    #[clap(long, short = 't', help = HelpStrings::ConfigurationInitializeToken)]
    pub token: Option<String>,

    #[clap(long, short, help = HelpStrings::Copy)]
    pub copy: bool,

    #[clap(long, short, help = HelpStrings::Quiet, conflicts_with = "verbose")]
    pub quiet: bool,

    #[clap(long, short = 'o', help = HelpStrings::Config)]
    pub config: Option<PathBuf>,

    #[clap(long, short, default_value = "default", help = HelpStrings::Environment)]
    pub environment: String,

    #[clap(long, short='v', help = HelpStrings::Verbose, parse(from_occurrences), conflicts_with = "quiet")]
    pub verbose: usize,

    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Connection(ConnectionOptions),
    Feature(FeaturesOptions),
    Schema(SchemaOptions),
    CredentialDefinition(CredentialDefinitionOptions),
    Message(MessageOptions),
    Credential(CredentialOptions),
    Configuration(ConfigurationOptions),
    Automate(WorkflowOptions),
}
