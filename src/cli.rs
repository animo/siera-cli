use super::{agent, error, typing, util};

// Cli runner entrypoint
pub async fn run(json: typing::Config) {
    let agent_endpoint = &json.agent_endpoint;
    // Check if the provided agent url is valid
    let agent = agent::Agent::new(agent_endpoint);

    // Checks if the provided agent is valid
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
    let invitation = agent
        .create_invitation_url(Some("Karel"), Some("true"), Some("true"))
        .await;
    util::print_invitation_and_qr_for_invitation(invitation);
}
