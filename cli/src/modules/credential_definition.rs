use agent_controller::modules::credential_definition::CredentialDefinitionModule;
use clap::{Args, Subcommand};
use serde_json::json;

use crate::{
    error::{Error, Result},
    utils::logger::Log,
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
    logger: Log,
) -> Result<()> {
    if let Some(id) = &options.id {
        return agent.get_by_id(id.to_string()).await.map(|cred_def| {
            if logger.debug {
                logger.log_pretty(cred_def)
            } else {
                let loggable = json!({
                    "id": cred_def.credential_definition.id,
                    "schema_id": cred_def.credential_definition.schema_id,
                    "type": cred_def.credential_definition.type_field,
                    "tag": cred_def.credential_definition.tag,
                    "ver": cred_def.credential_definition.ver,
                });
                logger.log_pretty(loggable);
            }
        });
    }
    if options.all {
        return agent
            .get_all()
            .await
            .map(|cred_defs| logger.log_list(cred_defs.credential_definition_ids));
    }
    match options
        .commands
        .as_ref()
        .ok_or_else(|| Error::NoSubcommandSupplied("credential-definition".to_string()))?
    {
        CredentialDefinitionSubcommands::Create { schema_id } => agent
            .create(schema_id.to_string())
            .await
            .map(|cred_def| logger.log(cred_def.sent.credential_definition_id)),
    }
}
