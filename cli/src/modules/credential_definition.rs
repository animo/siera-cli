use agent_controller::modules::credential_definition::CredentialDefinitionModule;
use clap::{Args, Subcommand};
use serde_json::json;

use crate::{error::Result, utils::logger::Log};

#[derive(Args)]
pub struct CredentialDefinitionOptions {
    #[clap(subcommand)]
    pub commands: CredentialDefinitionSubcommands,
}

#[derive(Subcommand, Debug)]
pub enum CredentialDefinitionSubcommands {
    Create {
        #[clap(short, long)]
        schema_id: String,
    },
    Get {
        #[clap(short, long)]
        id: String,
    },
}

pub async fn parse_credential_definition_args(
    commands: &CredentialDefinitionSubcommands,
    agent: impl CredentialDefinitionModule,
    logger: Log,
) -> Result<()> {
    match commands {
        CredentialDefinitionSubcommands::Create { schema_id } => agent
            .create(schema_id.to_string())
            .await
            .map(|cred_def| logger.log(cred_def.sent.credential_definition_id)),
        CredentialDefinitionSubcommands::Get { id } => {
            agent.get_by_id(id.to_string()).await.map(|cred_def| {
                let loggable = json!({
                    "id": cred_def.credential_definition.id,
                    "schema_id": cred_def.credential_definition.schema_id,
                    "type": cred_def.credential_definition.type_field,
                    "tag": cred_def.credential_definition.tag,
                    "ver": cred_def.credential_definition.ver,
                });
                logger.log_pretty(loggable);
            })
        }
    }
}
