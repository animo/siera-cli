use super::{error, error::Error, typing, typing::Result, utils::http};
use reqwest::Url;

// All the available endpoints
struct Endpoint {}

// Default value for every endpoint
// TODO: Not the most efficient mehtod (creates a new instance for every function call)
impl Endpoint {
    fn connections(agent: &Agent) -> Url {
        reqwest::Url::parse(&agent.url)
            .unwrap()
            .join("connections")
            .unwrap()
    }
    fn get_connection_by_id(agent: &Agent, id: String) -> Url {
        reqwest::Url::parse(&agent.url)
            .unwrap()
            .join("connections/")
            .unwrap()
            .join(&id.to_string())
            .unwrap()
    }
    fn create_invitation(agent: &Agent) -> Url {
        reqwest::Url::parse(&agent.url)
            .unwrap()
            .join("connections/")
            .unwrap()
            .join("create-invitation")
            .unwrap()
    }
}

// Structure of an Agent instance
pub struct Agent {
    url: String,
}

// Implementation for the Agent structure
impl Agent {
    // Create a new agent
    pub fn new(endpoint: &String) -> Agent {
        Agent {
            url: endpoint.to_owned(),
        }
    }

    /// Check to see whether the provided agent endpoint is valid
    pub async fn check_endpoint(&self) -> Result<bool> {
        match reqwest::get(Endpoint::connections(&self)).await {
            Ok(res) => {
                if res.status() == 404 {
                    return Err(Error::InvalidUrl);
                }
                Ok(true)
            }
            Err(_) => Err(Error::InvalidUrl),
        }
    }

    // Get All connections
    pub async fn get_connections(&self) -> typing::Connections {
        match http::call_get_endpoint(Endpoint::connections(&self), None).await {
            Ok(res) => match res.json().await {
                Ok(parsed) => parsed,
                Err(_) => error::throw(Error::ServerResponseParseError),
            },
            Err(_) => error::throw(Error::ConnectionsUnretrieveable),
        }
    }

    // Get connection by id
    pub async fn get_connection_by_id(&self, id: String) -> typing::Connection {
        match http::call_get_endpoint(Endpoint::get_connection_by_id(&self, id), None).await {
            Ok(res) => match res.json().await {
                Ok(parsed) => parsed,
                Err(_) => error::throw(error::Error::ServerResponseParseError),
            },
            Err(_) => error::throw(Error::ConnectionDoesNotExist),
        }
    }
    // Create invitation URL
    pub async fn create_invitation(
        &self,
        config: typing::InvitiationOptions,
    ) -> typing::Invitation {
        let query = vec![
            ("alias", config.alias),
            ("multi_use", config.multi_use),
            ("auto_accept", config.auto_accept),
        ];

        match http::call_post_endpoint(Endpoint::create_invitation(&self), Some(query)).await {
            Ok(res) => match res.json().await {
                Ok(parsed) => parsed,
                Err(_) => error::throw(error::Error::ServerResponseParseError),
            },
            Err(_) => error::throw(error::Error::CannotCreateInvitation),
        }
    }
}
