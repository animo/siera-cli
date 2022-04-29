use agent::modules::credential_definition::{
    CredentialDefinitionCreateOptions, CredentialDefinitionModule,
};
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
    #[clap(subcommand)]
    pub commands: CredentialDefinitionSubcommands,
}

#[derive(Subcommand, Debug)]
pub enum CredentialDefinitionSubcommands {
    #[clap(about = HelpStrings::CredentialDefinitionCreate)]
    Create {
        #[clap(short, long, help = HelpStrings::CredentialDefinitionCreateSchemaId)]
        schema_id: String,

        #[clap(short, long, help = HelpStrings::CredentialDefinitionCreateTag)]
        tag: Option<String>,

        #[clap(short = 'r', long, help = HelpStrings::CredentialDefinitionCreateSupportRevocation)]
        support_revocation: bool,

        #[clap(short = 'v', long, help = HelpStrings::CredentialDefinitionCreateRevocationRegistrySize)]
        revocation_registry_size: Option<i32>,
    },
    #[clap(about = HelpStrings::CredentialDefinitionList)]
    List {
        #[clap(long, short, help = HelpStrings::CredentialDefinitionId)]
        id: Option<String>,
    },
}

pub async fn parse_credential_definition_args(
    options: &CredentialDefinitionOptions,
    agent: impl CredentialDefinitionModule,
) -> Result<()> {
    let loader = Loader::start(LoaderVariant::default());

    match &options.commands {
        CredentialDefinitionSubcommands::Create {
            schema_id,
            support_revocation,
            tag,
            revocation_registry_size,
        } => {
            let options = CredentialDefinitionCreateOptions {
                schema_id: schema_id.to_string(),
                support_revocation: *support_revocation,
                tag: tag.as_deref().map(|t| t.to_string()),
                revocation_registry_size: *revocation_registry_size,
            };
            agent.create(options).await.map(|cred_def| {
                loader.stop();
                copy!("{}", cred_def.credential_definition_id);
                info!("{} credential definition with id: ", "Created".green());
                println!("{}", cred_def.credential_definition_id);
            })
        }
        CredentialDefinitionSubcommands::List { id } => match id {
            Some(i) => agent.get_by_id(i.to_owned()).await.map(|cred_def| {
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
            }),

            None => agent.get_all().await.map(|cred_defs| {
                loader.stop();
                cred_defs
                    .credential_definition_ids
                    .iter()
                    .for_each(|x| println!("{}", x));
                info!(
                    "{} fetched credential definition IDs",
                    "Sucessfully".green()
                );
            }),
        },
    }
}
