use agent_controller::modules::schema::{SchemaCreateOptions, SchemaModule};
use clap::{Args, Subcommand};

use crate::{error::Result, utils::logger::Log};

#[derive(Args)]
pub struct SchemaOptions {
    #[clap(subcommand)]
    pub commands: SchemaSubcommands,
}

#[derive(Subcommand, Debug)]
pub enum SchemaSubcommands {
    Create {
        #[clap(short, long)]
        name: String,
        #[clap(short, long, default_value = "1.0")]
        version: String,
        #[clap(short, long)]
        attributes: Vec<String>,
    },
    Get {
        #[clap(short, long)]
        id: String,
    },
}

pub async fn parse_schema_args(
    commands: &SchemaSubcommands,
    agent: impl SchemaModule,
    logger: Log,
) -> Result<()> {
    match commands {
        SchemaSubcommands::Create {
            name,
            attributes,
            version,
        } => {
            let options = SchemaCreateOptions {
                name: name.to_string(),
                version: version.to_string(),
                attributes: attributes.to_vec(),
            };
            agent
                .create(options)
                .await
                .map(|schema_id| logger.log(schema_id))
        }
        SchemaSubcommands::Get { id } => agent
            .get_by_id(id.to_string())
            .await
            .map(|schema| logger.log_pretty(schema.schema)),
    }
}
