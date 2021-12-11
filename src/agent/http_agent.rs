use crate::agent::agents::{Agent, HttpAgentExtended};
use crate::typing::{self, Connection, Connections, Feature};
use crate::utils::http;
use async_trait::async_trait;
use reqwest::Url;
use serde_json::json;

/// HTTP cloudagent
#[derive(Debug, Clone)]
pub struct HttpAgent {
    /// base url of the cloudagent
    url: String,
}

/// All the available endpoints
struct Endpoint;

/// Default value for every endpoint
impl Endpoint {
    /// base + connections
    fn connections(url: &str) -> Url {
        reqwest::Url::parse(url)
            .unwrap_or_else(|_| panic!("Could not parse {}", url))
            .join("connections")
            .unwrap_or_else(|_| panic!("Could not join on connections"))
    }
    /// base + connections + :id
    fn get_connection_by_id(url: &str, id: &str) -> Url {
        reqwest::Url::parse(url)
            .unwrap_or_else(|_| panic!("Could not parse {}", url))
            .join("connections/")
            .unwrap_or_else(|_| panic!("Could not join on connections"))
            .join(id)
            .unwrap_or_else(|_| panic!("Could not join on {}", id))
    }
    /// base + connections + create-invitation
    fn create_invitation(url: &str) -> Url {
        reqwest::Url::parse(url)
            .unwrap_or_else(|_| panic!("Could not parse {}", url))
            .join("connections/")
            .unwrap_or_else(|_| panic!("Could not join on connections"))
            .join("create-invitation")
            .unwrap_or_else(|_| panic!("Could not join on create-invitation"))
    }
    /// base + features
    fn discover_features(url: &str) -> Url {
        reqwest::Url::parse(url)
            .unwrap_or_else(|_| panic!("Could not parse {}", url))
            .join("features")
            .unwrap_or_else(|_| panic!("Could not join on features"))
    }
}

#[async_trait]
impl HttpAgentExtended for HttpAgent {
    fn new(endpoint: &str) -> Self {
        HttpAgent {
            url: endpoint.to_owned(),
        }
    }

    /// Check if the endpoint is valid
    async fn check_endpoint(&self) -> () {
        http::get::<Connections>(Endpoint::connections(&self.url), None).await;
    }
}

#[async_trait]
impl Agent for HttpAgent {
    /// Gets all the connections
    async fn get_connections(&self, filter: Option<&str>) -> Connections {
        let mut query: Vec<(&str, String)> = vec![];

        if let Some(alias) = filter {
            query.push(("alias", alias.to_string()));
        }

        http::get::<Connections>(Endpoint::connections(&self.url), Some(query)).await
    }

    /// Get a connection by id
    async fn get_connection_by_id(&self, id: &str) -> typing::Connection {
        http::get::<Connection>(Endpoint::get_connection_by_id(&self.url, id), None).await
    }

    /// Prints an invitation, as url or qr, in stdout
    async fn create_invitation(&self, config: &typing::InvitationConfig<'_>) -> typing::Invitation {
        let mut query: Vec<(&str, String)> = vec![];
        let mut body = None;

        if config.toolbox {
            query.push(("multi_use", false.to_string()));
            query.push(("auto_accept", true.to_string()));
            query.push(("alias", String::from("toolbox")));

            body = Some(json!({
                "metadata": {
                    "group": "admin"
                }
            }));
        } else {
            let multi_use = ("multi_use", config.multi_use.to_string());
            let auto_accept = ("auto_accept", config.auto_accept.to_string());

            query.push(multi_use);
            query.push(auto_accept);

            if let Some(alias) = config.alias {
                query.push(("alias", alias.to_string()));
            }
        }

        http::post(Endpoint::create_invitation(&self.url), query, body).await
    }

    /// Requests all the features from the cloudagent
    async fn discover_features(&self) -> Feature {
        http::get::<Feature>(Endpoint::discover_features(&self.url), None).await
    }
}
