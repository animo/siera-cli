use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::help_strings::HelpStrings;

use crate::modules::{
    configuration::ConfigurationOptions, connections::ConnectionOptions,
    credential_definition::CredentialDefinitionOptions, credentials::CredentialOptions,
    features::FeaturesOptions, message::MessageOptions, schema::SchemaOptions,
};

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(arg_required_else_help = true)]
pub struct Cli {
    #[clap(long, short, help = HelpStrings::Endpoint)]
    pub endpoint: Option<String>,

    #[clap(long, short, help = HelpStrings::ApiKey)]
    pub api_key: Option<String>,

    #[clap(long, short, help = HelpStrings::Copy)]
    pub copy: bool,

    #[clap(long, short, help = HelpStrings::Quiet, conflicts_with = "raw")]
    pub quiet: bool,

    #[clap(long, short = 'o', help = HelpStrings::Config)]
    pub config: Option<PathBuf>,

    #[clap(long, short = 'v', default_value = "Default", help = HelpStrings::Environment)]
    pub environment: String,

    #[clap(long, short, help = HelpStrings::Verbose, conflicts_with = "quiet")]
    pub raw: bool,

    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Connections(ConnectionOptions),
    Features(FeaturesOptions),
    Schema(SchemaOptions),
    CredentialDefinition(CredentialDefinitionOptions),
    Message(MessageOptions),
    Credentials(CredentialOptions),
    Configuration(ConfigurationOptions),
}
