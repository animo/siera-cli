use agent::modules::credential_definition::CredentialDefinitionModule;
use clap::{Args, Subcommand};
use log::{debug, info};
use serde_json::json;

use crate::{
    error::{Error, Result},
    utils::loader::{Loader, LoaderVariant},
    utils::logger::pretty_stringify_obj,
};

#[derive(Args)]
pub struct CredentialDefinitionOptions {
    #[clap(subcommand)]
    pub commands: Option<CredentialDefinitionSubcommands>,

    #[clap(long, short)]
    pub id: Option<String>,

    #[clap(long, short)]
    pub all: bool,
}

#[derive(Subcommand, Debug)]
pub enum CredentialDefinitionSubcommands {
    Create {
        #[clap(short, long)]
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
            info!("{}", pretty_stringify_obj(loggable));
        });
    }
    if options.all {
        return agent.get_all().await.map(|cred_defs| {
            loader.stop();
            cred_defs
                .credential_definition_ids
                .iter()
                .for_each(|x| info!("{}", x))
        });
    }
    match options
        .commands
        .as_ref()
        .ok_or_else(|| Error::NoSubcommandSupplied("credential-definition".to_string()))?
    {
        CredentialDefinitionSubcommands::Create { schema_id } => {
            agent.create(schema_id.to_string()).await.map(|cred_def| {
                loader.stop();
                info!("{}", cred_def.credential_definition_id);
            })
        }
    }
}
