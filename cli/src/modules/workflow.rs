use agent::modules::connections::{
    ConnectionCreateInvitationOptions, ConnectionModule, ConnectionReceiveInvitationOptions,
};
use agent::modules::credential_definition::CredentialDefinitionModule;
use agent::modules::credentials::CredentialsModule;
use agent::modules::schema::SchemaModule;
use clap::{Args, Subcommand};
use colored::*;
use log::{debug, trace};
use std::collections::HashMap;
use workflow::workflows::credential_offer::CredentialOfferWorkflow;

use crate::copy;
use crate::error::{Error, Result};
use crate::modules::connections::invite_url_to_object;
use crate::utils::loader::{Loader, LoaderVariant};
use crate::utils::qr;

#[derive(Args)]
pub struct WorkflowOptions {
    #[clap(subcommand)]
    pub commands: WorkflowSubcommands,
}

#[derive(Subcommand, Debug)]
pub enum WorkflowSubcommands {
    CredentialOffer {
        #[clap(long, short)]
        connection_id: Option<String>,

        #[clap(long, short)]
        timeout: Option<u32>,

        #[clap(long = "self", short = 's')]
        sent_to_self: bool,
    },
}

pub async fn parse_workflow_args(
    options: &WorkflowOptions,
    agent: impl ConnectionModule + CredentialsModule + SchemaModule + CredentialDefinitionModule,
) -> Result<()> {
    let loader = Loader::start(LoaderVariant::default());

    match &options.commands {
        WorkflowSubcommands::CredentialOffer {
            connection_id,
            timeout,
            sent_to_self,
        } => match connection_id {
            Some(c) => credential_offer(c.to_owned(), agent).await?,
            None => {
                let connection = agent
                    .create_invitation(ConnectionCreateInvitationOptions {
                        auto_accept: true,
                        alias: Some(String::from("workflow")),
                        ..Default::default()
                    })
                    .await?;
                if *sent_to_self {
                    let invitation_object = invite_url_to_object(connection.invitation_url)?;
                    agent.receive_invitation(invitation_object).await?;
                } else {
                    qr::print_qr_code(&connection.invitation_url)?;
                    println!("Connection id: {}", connection.connection_id);
                    println!(
                        "Use this invitation to connect with your agent.\n{}",
                        connection.invitation_url
                    );
                    copy!("{}", connection.invitation_url);
                }
                let limit = timeout.map(|t| t / 1000).unwrap_or(10);
                debug!("Looping {} times", limit);
                for i in 1..=limit {
                    let connection =
                        ConnectionModule::get_by_id(&agent, connection.connection_id.to_owned())
                            .await?;
                    if connection.state != "active" && connection.state != "response" {
                        trace!("Connection state is not active");
                        std::thread::sleep(std::time::Duration::from_millis(1000));
                    } else {
                        println!("Invitation {}!", "accepted".green());
                        credential_offer(connection.connection_id, agent).await?;
                        break;
                    }
                    if i == limit {
                        return Err(Error::InactiveConnection.into());
                    }
                }
            }
        },
    };
    println!("{} executed workflow", "Successfully".green());
    println!("{}: It might take a few seconds for the credential to arrive", "Note".cyan());
    loader.stop();
    Ok(())
}

async fn credential_offer(
    connection_id: String,
    agent: impl ConnectionModule + CredentialsModule + SchemaModule + CredentialDefinitionModule,
) -> Result<()> {
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
    debug!("Mock credential:\n{:#?}", attributes);

    let options = CredentialOfferWorkflow {
        connection_id,
        attributes,
    };

    options.execute(agent).await
}
