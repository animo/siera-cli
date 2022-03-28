use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::help_strings::HelpStrings;

use crate::modules::workflow::WorkflowOptions;
use crate::modules::{
    configuration::ConfigurationOptions, connections::ConnectionOptions,
    credential_definition::CredentialDefinitionOptions, credentials::CredentialOptions,
    features::FeaturesOptions, message::MessageOptions, schema::SchemaOptions,
};

#[derive(Parser)]
#[clap(name = "aries-cli", author, version, about = HelpStrings::Cli)]
#[clap(arg_required_else_help = true)]
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

    #[clap(long, short = 'e', default_value = "default", help = HelpStrings::Environment)]
    pub environment: String,

    #[clap(long, short='v', help = HelpStrings::Verbose, parse(from_occurrences), conflicts_with = "quiet")]
    pub verbose: usize,

    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Connections(ConnectionOptions),
    Features(FeaturesOptions),
    Schemas(SchemaOptions),
    CredentialDefinitions(CredentialDefinitionOptions),
    Message(MessageOptions),
    Credentials(CredentialOptions),
    Configuration(ConfigurationOptions),
    Workflow(WorkflowOptions),
}
