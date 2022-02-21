use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::modules::{
    connections::ConnectionOptions, credential_definition::CredentialDefinitionOptions,
    credentials::CredentialOptions, features::FeaturesOptions, message::MessageOptions,
    schema::SchemaOptions,
};

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(arg_required_else_help = true)]
pub struct Cli {
    #[clap(long, short)]
    pub endpoint: Option<String>,

    #[clap(long, short)]
    pub api_key: Option<String>,

    #[clap(long, short)]
    pub copy: bool,

    #[clap(long, short)]
    pub quiet: bool,

    #[clap(long, short = 'o')]
    pub config: Option<PathBuf>,

    #[clap(long, short = 'v', default_value = "Default")]
    pub environment: String,

    #[clap(long, short)]
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
}
