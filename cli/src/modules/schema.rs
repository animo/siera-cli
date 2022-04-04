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
    #[clap(subcommand)]
    pub commands: SchemaSubcommands,
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
    List {
        #[clap(long, short, help=HelpStrings::SchemaId)]
        id: Option<String>,
    },
}

pub async fn parse_schema_args(options: &SchemaOptions, agent: impl SchemaModule) -> Result<()> {
    let loader = Loader::start(LoaderVariant::default());
    match &options.commands {
        SchemaSubcommands::Create {
            name,
            version,
            attributes,
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
                info!(
                    "{} schema with the following attributes: ",
                    "Created".green(),
                );
                schema
                    .schema
                    .attr_names
                    .into_iter()
                    .for_each(|name| info!("- {}", name));
                info!("{}", "Schema id: ".cyan());
                println!("{}", schema.schema_id);
                copy!("{}", schema.schema_id);
            })
        }
        SchemaSubcommands::List { id } => match id {
            Some(i) => agent.get_by_id(i.to_owned()).await.map(|schema| {
                loader.stop();
                copy!("{}", pretty_stringify_obj(&schema));
                println!("{}", pretty_stringify_obj(schema));
            }),
            None => agent.get_all().await.map(|schemas| {
                loader.stop();
                schemas.schema_ids.iter().for_each(|x| println!("{}", x));
                info!("{} fetched schema IDs", "Successfully".green());
            }),
        },
    }
}
