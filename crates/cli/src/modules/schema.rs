use crate::error::{Error, Result};
use crate::help_strings::HelpStrings;
use crate::utils::loader::{Loader, LoaderVariant};
use clap::{Args, Subcommand};
use siera_agent::modules::schema::{SchemaCreateOptions, SchemaModule};
use siera_logger::pretty_stringify_obj;

/// Schema options and flags
#[derive(Args)]
#[clap(about = HelpStrings::Schema)]
pub struct SchemaOptions {
    /// All the subcommands of the schema cli
    #[clap(subcommand)]
    pub commands: SchemaSubcommands,
}

/// Schema subcommands
#[derive(Subcommand, Debug)]
pub enum SchemaSubcommands {
    /// Register a new schema on the ledger
    #[clap(about = HelpStrings::SchemaCreate)]
    Create {
        /// The name of schema
        #[clap(short, long, help=HelpStrings::SchemaCreateName)]
        name: String,

        /// The version of the schema
        #[clap(short, long, help=HelpStrings::SchemaCreateVersion, default_value = "1.0")]
        version: String,

        /// A list of strings of the attributes that must be met when issuing a credential
        /// with this schema
        #[clap(short, long, help=HelpStrings::SchemaCreateAttributes, required = true)]
        attribute: Vec<String>,
    },

    /// List all the registered schemas
    #[clap(about = HelpStrings::SchemaList)]
    List {
        /// Filter by id to fetch a specific schema
        #[clap(long, short, help=HelpStrings::SchemaId)]
        id: Option<String>,
    },
}

/// Subcommand Schema parser
pub async fn parse_schema_args(
    options: &SchemaOptions,
    agent: impl SchemaModule + Send + Sync,
) -> Result<()> {
    let loader = Loader::start(&LoaderVariant::default());
    match &options.commands {
        SchemaSubcommands::Create {
            name,
            version,
            attribute,
        } => {
            let options = SchemaCreateOptions {
                name: name.clone(),
                version: version.clone(),
                attributes: attribute.clone(),
            };
            if options.attributes.is_empty() {
                return Err(Error::RequiredAttributes.into());
            }
            agent.create(options).await.map(|schema| {
                debug!({ "schema": schema });
                info!({"message": "Created schema"});
                schema
                    .attr_names
                    .into_iter()
                    .for_each(|name| info!({ "schema_name": name }));
                log!({ "schema_id": schema.id});
                copy!("{}", schema.id);
            })
        }
        SchemaSubcommands::List { id } => match id {
            Some(i) => agent.get_by_id(i.clone()).await.map(|schema| {
                loader.stop();
                log!({ "schema": schema });
                copy!("{}", pretty_stringify_obj(&schema));
            }),
            None => agent.get_all().await.map(|schemas| {
                loader.stop();
                schemas
                    .schema_ids
                    .iter()
                    .for_each(|schema_id| log!({ "schema_id": schema_id }));
                info!({ "message": "Successfully fetched schema IDs" });
            }),
        },
    }
}
