use crate::{
    agent, error,
    typing::{self, InvitiationOptions},
    utils::logger::Log,
};

// Cli runner entrypoint
pub async fn run(json: typing::Config) {
    // Should we run with or without a connection
    // create agent
    let agent = agent::Agent::new(&json.agent_endpoint);

    // Checks if the provided agent is valid
    if let Err(_) = agent.check_endpoint().await {
        error::throw(error::Error::InvalidEndpoint)
    }
    run_with_endpoint(agent, json).await
}

async fn run_with_endpoint(agent: agent::Agent, json: typing::Config) {
    let invitation_config = match json.invitation_options {
        Some(inv) => inv,
        // Default: mutli use and accept on
        None => InvitiationOptions {
            multi_use: Some(String::from("true")),
            alias: None,
            auto_accept: Some(String::from("true")),
        },
    };

    let invitation = agent.create_invitation(invitation_config).await;
    Log::output(&invitation.invitation_url)
}
