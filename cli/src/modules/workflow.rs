use agent_controller::modules::connections::ConnectionModule;
use agent_controller::modules::credential_definition::CredentialDefinitionModule;
use agent_controller::modules::credentials::CredentialsModule;
use agent_controller::modules::schema::SchemaModule;
use clap::{Args, Subcommand};

use crate::utils::logger::Log;
use crate::error::Result;
use crate::workflows::examples::offer_credential::OFFER_CREDENTIAL_EXAMPLE;
use crate::workflows::offer_credential::{WorkflowOfferCredential, offer_credential};

#[derive(Args)]
pub struct WorkflowOptions {
    #[clap(subcommand)]
    pub commands: WorkflowSubcommands,
}

#[derive(Subcommand, Debug)]
pub enum WorkflowSubcommands {
    OfferCredential {
        #[clap(short, long)]
        yaml_path: Option<String>,
    },
}


pub async fn parse_workflow_args(
    options: &WorkflowOptions,
    agent: impl CredentialsModule + ConnectionModule + CredentialDefinitionModule +SchemaModule ,
    logger: Log,
) -> Result<()> {
    match &options.commands {
        WorkflowSubcommands::OfferCredential { yaml_path } => {
            let workflow = match yaml_path {
                Some(yp) => {
                    let out = std::fs::read_to_string(yp)?;
                    serde_yaml::from_str::<WorkflowOfferCredential>(&out)
                },
                None => serde_yaml::from_str::<WorkflowOfferCredential>(OFFER_CREDENTIAL_EXAMPLE)
            };
            offer_credential(agent, workflow?).await?;
            logger.log("Done :D");

            Ok(()) 
        },
    }
}
