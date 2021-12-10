use crate::agent::agent::{Agent, HttpAgentExtended};
use crate::error;
use crate::typing::{self, Connection, Connections, Feature};
use crate::utils::http;
use async_trait::async_trait;
use reqwest::Url;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct HttpAgent {
    url: String,
}

// All the available endpoints
struct Endpoint;

// Default value for every endpoint
// TODO: Not the most efficient mehtod (creates a new instance for every function call)
impl Endpoint {
    fn connections(agent: &HttpAgent) -> Url {
        reqwest::Url::parse(&agent.url)
            .expect(&format!("Could not join on {}", agent.url).to_string())
            .join("connections")
            .expect(&format!("Could not join on connections").to_string())
    }
    fn get_connection_by_id(agent: &HttpAgent, id: String) -> Url {
        reqwest::Url::parse(&agent.url)
            .expect(&format!("Could not join on {}", agent.url).to_string())
            .join("connections/")
            .expect(&format!("Could not join on connections/").to_string())
            .join(&id.to_string())
            .expect(&format!("Could not join on {}", id).to_string())
    }
    fn create_invitation(agent: &HttpAgent) -> Url {
        reqwest::Url::parse(&agent.url)
            .expect(&format!("Could not join on {}", agent.url).to_string())
            .join("connections/")
            .expect(&format!("Could not join on connections/").to_string())
            .join("create-invitation")
            .expect(&format!("Could not join on create-invitation").to_string())
    }
    fn discover_features(agent: &HttpAgent) -> Url {
        reqwest::Url::parse(&agent.url)
            .expect(&format!("Could not join on {}", agent.url).to_string())
            .join("features")
            .expect(&format!("Could not join on features").to_string())
    }
}

#[async_trait]
impl HttpAgentExtended for HttpAgent {
    fn new(endpoint: &str) -> Self {
        HttpAgent {
            url: endpoint.to_owned(),
        }
    }

    async fn check_endpoint(&self) -> typing::Result<()> {
        match reqwest::get(Endpoint::connections(&self)).await {
            Ok(res) => {
                if res.status().is_success() {
                    return Ok(());
                }

                Err(error::Error::InvalidUrl)
            }
            Err(_) => Err(error::Error::InvalidUrl),
        }
    }
}

#[async_trait]
impl Agent for HttpAgent {
    async fn get_connections(&self) -> Connections {
        http::get::<Connections>(Endpoint::connections(&self), None).await
    }

    async fn get_connection_by_id(&self, id: String) -> typing::Connection {
        http::get::<Connection>(Endpoint::get_connection_by_id(&self, id), None).await
    }

    async fn create_invitation(
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

            config
                .alias
                .and_then(|alias| Some(query.push(("alias", alias.to_string()))));
        }

        http::post(Endpoint::create_invitation(&self), query, body).await
    }

    async fn discover_features(&self) -> Feature {
        http::get::<Feature>(Endpoint::discover_features(&self), None).await
    }
}
