use agent::modules::schema::{SchemaCreateOptions, SchemaModule};
use clap::{Args, Subcommand};
use colored::*;
use log::{debug, info};

use crate::{
    copy,
    error::{Error, Result},
    help_strings::HelpStrings,
    utils::{
        loader::{Loader, LoaderVariant},
        logger::pretty_stringify_obj,
    },
};

#[derive(Args)]
#[clap(about = HelpStrings::Schema)]
pub struct SchemaOptions {
    #[clap(long, short, help=HelpStrings::SchemaId)]
    pub id: Option<String>,

    #[clap(subcommand)]
    pub commands: Option<SchemaSubcommands>,
}

#[derive(Subcommand, Debug)]
pub enum SchemaSubcommands {
    #[clap(about = HelpStrings::SchemaCreate)]
    Create {
        #[clap(short, long, help=HelpStrings::SchemaCreateName)]
        name: String,
        #[clap(short, long, help=HelpStrings::SchemaCreateVersion, default_value = "1.0")]
        version: String,
        #[clap(short, long, help=HelpStrings::SchemaCreateAttributes)]
        attributes: Vec<String>,
    },
}

pub async fn parse_schema_args(options: &SchemaOptions, agent: impl SchemaModule) -> Result<()> {
    let loader = Loader::start(LoaderVariant::default());
    if let Some(id) = &options.id {
        return agent.get_by_id(id.to_string()).await.map(|schema| {
            loader.stop();
            copy!("{}", pretty_stringify_obj(&schema));
            println!("{}", pretty_stringify_obj(schema));
        });
    }

    match &options.commands {
        Some(o) => match o {
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
                if options.attributes.is_empty() {
                    return Err(Error::RequiredAttributes.into());
                }
                agent.create(options).await.map(|schema| {
                    debug!("{}", pretty_stringify_obj(&schema));
                    copy!("{}", schema.schema_id);
                    println!("{}", schema.schema_id);
                })
            }
        },
        None => agent.get_all().await.map(|schemas| {
            loader.stop();
            schemas.schema_ids.iter().for_each(|x| println!("{}", x));
            info!("{} fetched schema IDs", "Successfully".green());
        }),
    }
}
