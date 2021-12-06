use std::collections::HashMap;

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
    //fn get_connection_by_id(agent: &Agent, id: String) -> Url {
    //reqwest::Url::parse(&agent.url)
    //.unwrap()
    //.join("connections/")
    //.unwrap()
    //.join(&id.to_string())
    //.unwrap()
    //}
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
#[derive(Debug)]
pub struct Agent {
    url: String,
}

// Implementation for the Agent structure
impl Agent {
    // Create a new agent
    pub fn new(endpoint: &str) -> Agent {
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
    //pub async fn get_connections(&self) -> typing::Connections {
    //match http::call_get_endpoint(Endpoint::connections(&self), None).await {
    //Ok(res) => match res.json().await {
    //Ok(parsed) => parsed,
    //Err(_) => error::throw(Error::ServerResponseParseError),
    //},
    //Err(_) => error::throw(Error::ConnectionsUnretrieveable),
    //}
    //}

    // Get connection by id
    //pub async fn get_connection_by_id(&self, id: String) -> typing::Connection {
    //match http::call_get_endpoint(Endpoint::get_connection_by_id(&self, id), None).await {
    //Ok(res) => match res.json().await {
    //Ok(parsed) => parsed,
    //Err(_) => error::throw(error::Error::ServerResponseParseError),
    //},
    //Err(_) => error::throw(Error::ConnectionDoesNotExist),
    //}
    //}
    // Create invitation URL
    pub async fn create_invitation(
        &self,
        config: &typing::InviteConfiguration<'_>,
    ) -> typing::Invitation {
        let mut query: Vec<(&str, String)> = vec![];
        let mut body = None;

        if config.toolbox {
            query.push(("multi_use", false.to_string()));
            query.push(("auto_accept", true.to_string()));
            query.push(("alias", String::from("toolbox")));

            let mut a = HashMap::new();
            let mut b = HashMap::new();

            b.insert("group", "admin");
            a.insert("metadata", b);

            body = Some(a);
        } else {
            let multi_use = ("multi_use", config.multi_use.to_string());
            let auto_accept = ("auto_accept", config.auto_accept.to_string());

            query.push(multi_use);
            query.push(auto_accept);

            match config.alias {
                Some(alias) => query.push(("alias", alias.to_string())),
                _ => (),
            };
        }

        match http::call_post_endpoint(Endpoint::create_invitation(&self), query, body).await {
            Ok(res) => match res.json().await {
                Ok(parsed) => parsed,
                //Err(_) => error::throw(error::Error::ServerResponseParseError),
                Err(e) => panic!("{:?}", e),
            },
            Err(_) => error::throw(error::Error::CannotCreateInvitation),
        }
    }
}
