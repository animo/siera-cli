use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::help_strings::top_level as HelpStrings;

use crate::modules::{
    configuration::ConfigurationOptions, connections::ConnectionOptions,
    credential_definition::CredentialDefinitionOptions, credentials::CredentialOptions,
    features::FeaturesOptions, message::MessageOptions, schema::SchemaOptions,
};

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(arg_required_else_help = true)]
pub struct Cli {
    #[clap(long, short, help = HelpStrings::ENDPOINT)]
    pub endpoint: Option<String>,

    #[clap(long, short, help = HelpStrings::API_KEY)]
    pub api_key: Option<String>,

    #[clap(long, short, help = HelpStrings::COPY)]
    pub copy: bool,

    #[clap(long, short, help = HelpStrings::QUIET)]
    pub quiet: bool,

    #[clap(long, short = 'o', help = HelpStrings::CONFIG)]
    pub config: Option<PathBuf>,

    #[clap(long, short = 'v', default_value = "Default", help = HelpStrings::ENVIRONMENT)]
    pub environment: String,

    #[clap(long, short, help = HelpStrings::VERBOSE)]
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
