use agent_controller::modules::connections::ConnectionModule;
use agent_controller::modules::credential_definition::CredentialDefinitionModule;
use agent_controller::modules::credentials::CredentialsModule;
use agent_controller::modules::schema::SchemaModule;
use clap::{Args, Subcommand};

use crate::error::Result;
use crate::utils::logger::Log;
use crate::workflows::examples::offer_credential::OFFER_CREDENTIAL_EXAMPLE;
use crate::workflows::offer_credential::{offer_credential, WorkflowOfferCredential};

#[derive(Args)]
pub struct WorkflowOptions {
    #[clap(subcommand)]
    pub commands: WorkflowSubcommands,
}

#[derive(Subcommand, Debug)]
pub enum WorkflowSubcommands {
    OfferCredential {},
}

pub async fn parse_workflow_args(
    options: &WorkflowOptions,
    agent: impl CredentialsModule + ConnectionModule + CredentialDefinitionModule + SchemaModule,
    logger: Log,
) -> Result<()> {
    match &options.commands {
        WorkflowSubcommands::OfferCredential {} => {
            let workflow =
                serde_yaml::from_str::<WorkflowOfferCredential>(OFFER_CREDENTIAL_EXAMPLE)?;
            offer_credential(agent, workflow, logger).await
        }
    }
}
