use agent::modules::connections::ConnectionModule;
use agent::modules::credential_definition::CredentialDefinitionModule;
use agent::modules::credentials::CredentialsModule;
use agent::modules::schema::SchemaModule;
use clap::{Args, Subcommand};
use colored::*;
use std::collections::HashMap;
use workflow::workflows::credential_offer::CredentialOfferWorkflow;

use crate::error::Result;
use crate::utils::loader::{Loader, LoaderVariant};

#[derive(Args)]
pub struct WorkflowOptions {
    #[clap(subcommand)]
    pub commands: WorkflowSubcommands,
}

#[derive(Subcommand, Debug)]
pub enum WorkflowSubcommands {
    CredentialOffer {
        #[clap(long, short)]
        connection_id: String,
    },
}

pub async fn parse_workflow_args(
    options: &WorkflowOptions,
    agent: impl ConnectionModule + CredentialsModule + SchemaModule + CredentialDefinitionModule,
) -> Result<()> {
    let loader = Loader::start(LoaderVariant::default());

    match &options.commands {
        WorkflowSubcommands::CredentialOffer { connection_id } => {
            // Mock credential
            let mut attributes: HashMap<String, String> = HashMap::new();
            attributes.insert(String::from("Name"), String::from("Joyce Brown"));
            attributes.insert(String::from("Date Of Birth"), String::from("19890321"));
            attributes.insert(String::from("Street"), String::from("Main Road 207"));
            attributes.insert(String::from("City"), String::from("New York"));
            attributes.insert(String::from("Bank"), String::from("qBank New York"));
            attributes.insert(
                String::from("Card Number"),
                String::from("4537-6696-0666-0146"),
            );
            attributes.insert(String::from("Security Code"), String::from("063"));
            attributes.insert(String::from("Valid Until"), String::from("20251212"));

            let options = CredentialOfferWorkflow {
                connection_id: connection_id.to_string(),
                attributes,
            };

            options.execute(agent).await?;
            println!("{} executed workflow", "Successfully".green());
            loader.stop();
            Ok(())
        }
    }
}
