use clap::{Parser, Subcommand};

use crate::modules::{connections::ConnectionOptions, features::FeaturesOptions};

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

    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Connections(ConnectionOptions),
    Features(FeaturesOptions),
}
