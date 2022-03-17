use agent::modules::credential_definition::CredentialDefinitionModule;
use clap::{Args, Subcommand};
use colored::*;
use log::{debug, info};
use serde_json::json;

use crate::{
    copy,
    error::Result,
    help_strings::HelpStrings,
    utils::loader::{Loader, LoaderVariant},
    utils::logger::pretty_stringify_obj,
};

#[derive(Args)]
#[clap(about = "Retrieve or create credential definitions")]
pub struct CredentialDefinitionOptions {
    #[clap(long, short, help = HelpStrings::CredentialDefinitionId)]
    pub id: Option<String>,

    #[clap(subcommand)]
    pub commands: Option<CredentialDefinitionSubcommands>,
}

#[derive(Subcommand, Debug)]
pub enum CredentialDefinitionSubcommands {
    #[clap(about = HelpStrings::CredentialDefinitionCreate)]
    Create {
        #[clap(short, long, help = HelpStrings::CredentialDefinitionCreateSchemaId)]
        schema_id: String,
    },
}

pub async fn parse_credential_definition_args(
    options: &CredentialDefinitionOptions,
    agent: impl CredentialDefinitionModule,
) -> Result<()> {
    let loader = Loader::start(LoaderVariant::default());
    if let Some(id) = &options.id {
        return agent.get_by_id(id.to_string()).await.map(|cred_def| {
            loader.stop();
            let loggable = json!({
                "id": cred_def.credential_definition.id,
                "schema_id": cred_def.credential_definition.schema_id,
                "type": cred_def.credential_definition.type_field,
                "tag": cred_def.credential_definition.tag,
                "ver": cred_def.credential_definition.ver,
            });
            debug!("{}", pretty_stringify_obj(cred_def));
            copy!("{}", pretty_stringify_obj(&loggable));
            println!("{}", pretty_stringify_obj(loggable));
        });
    }

    match &options.commands {
        Some(o) => match o {
            CredentialDefinitionSubcommands::Create { schema_id } => {
                agent.create(schema_id.to_string()).await.map(|cred_def| {
                    loader.stop();
                    copy!("{}", cred_def.credential_definition_id);
                    info!("{} credential definition with id: ", "Created".green());
                    println!("{}", cred_def.credential_definition_id);
                })
            }
        },
        None => agent.get_all().await.map(|cred_defs| {
            loader.stop();
            cred_defs
                .credential_definition_ids
                .iter()
                .for_each(|x| info!("{}", x))
        }),
    }
}
