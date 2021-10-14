use crate::{
    agent, error, typing,
    utils::{logger::Log, qr},
};
use std::{thread, time::Duration};

// Cli runner entrypoint
pub async fn run(json: typing::Config) {
    let agent_endpoint = &json.agent_endpoint;

    // Create agent
    let agent = agent::Agent::new(agent_endpoint);

    // Checks if the provided agent endpoint is valid
    if let Err(_) = agent.check_endpoint().await {
        error::throw(error::Error::InvalidEndpoint)
    }

    // Should we run with or without a connection
    if json.connection_id.is_some() {
        run_with_connection(agent, json).await;
    } else {
        run_without_connection(agent, json).await;
    }
}

// Running when a specific connection is added to the config
async fn run_with_connection(agent: agent::Agent, json: typing::Config) {
    // unwrap here because we checked in `run` if it contains a value
    let connection = agent
        .get_connection_by_id(json.connection_id.unwrap())
        .await;
    println!("{:?}", connection);
}

// Running when no connection is added to the config
async fn run_without_connection(agent: agent::Agent, json: typing::Config) {
    // Assert that invitation object exists in the config
    let invitation_config = match json.invitation_options {
        Some(inv) => inv,
        None => error::throw(error::Error::InvalidInvitationConfiguration),
    };

    let invitation = agent.create_invitation(invitation_config).await;

    qr::print_invitation_and_qr_for_invitation(&invitation);

    let result = loop {
        let result = agent
            .get_connection_by_id(invitation.connection_id.clone())
            .await;
        if result.state == "active" {
            break result;
        } else {
            thread::sleep(Duration::from_secs(5))
        }
    };
    Log::log(&result.state)
}