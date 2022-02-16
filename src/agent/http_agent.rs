use crate::agent::agents::{Agent, HttpAgentExtended};
use crate::cli::connections::Connections;
use crate::cli::connections::{Connection, ConnectionsConfig};
use crate::cli::credential_definition::CredentialDefinitionConfig;
use crate::cli::credential_definition::{CredentialDefinition, CredentialDefinitions};
use crate::cli::features::Features;
use crate::cli::invite::Invitation;
use crate::cli::invite::InvitationConfig;
use crate::cli::issue_credential::IssueCredentialConfig;
use crate::cli::message::MessageConfig;
use crate::cli::schema::Schema;
use crate::cli::schema::SchemaConfig;
use crate::utils::http::HttpCalls;
use crate::utils::logger::Log;
use async_trait::async_trait;
use reqwest::Url;
use serde_json::{json, Value};

use super::agents::BaseAgent;

/// HTTP cloudagent
#[derive(Debug, Clone)]
pub struct HttpAgent {
    /// Base agent for generic data
    pub base_agent: BaseAgent,

    /// base url of the cloudagent
    pub url: String,

    /// admin Api key for the cloudagent
    pub api_key: Option<String>,
}

// TODO: remove access slashes
//       macroify
/// Creates an url from an array
fn create_url(arr: Vec<&str>) -> Url {
    let url = arr.join("/");
    //TODO: error here that the url is invalid
    reqwest::Url::parse(&url).unwrap()
}

/// All the available endpoints
struct Endpoint;

/// Default value for every endpoint
impl Endpoint {
    /// base + connections
    fn connections(url: &str) -> Url {
        create_url(vec![url, "connections"])
    }
    /// base + connections + :id
    fn get_connection_by_id(url: &str, id: &str) -> Url {
        create_url(vec![url, "connections", id])
    }
    /// base + connections + create-invitation
    fn create_invitation(url: &str) -> Url {
        create_url(vec![url, "connections", "create-invitation"])
    }
    /// base + features
    fn discover_features(url: &str) -> Url {
        create_url(vec![url, "features"])
    }
    /// base + connections + :id + send-message
    fn basic_message(url: &str, id: &str) -> Url {
        create_url(vec![url, "connections", id, "send-message"])
    }
    /// base + issue-credential + send-offer
    fn credential_offer(url: &str) -> Url {
        create_url(vec![url, "issue-credential", "send-offer"])
    }
    /// base + schemas
    fn schema(url: &str) -> Url {
        create_url(vec![url, "schemas"])
    }
    /// base + credential-definitions
    fn credential_definition(url: &str) -> Url {
        create_url(vec![url, "credential-definitions"])
    }
    /// base + credential-definitions
    fn credential_definition_created(url: &str) -> Url {
        create_url(vec![url, "credential-definitions", "created"])
    }
}

#[async_trait]
impl HttpAgentExtended for HttpAgent {
    fn new(base_agent: BaseAgent, endpoint: String, api_key: Option<String>) -> Self {
        HttpAgent {
            base_agent,
            url: endpoint,
            api_key,
        }
    }

    /// Check if the endpoint is valid
    async fn check_endpoint(&self) -> () {
        self.get::<Connections>(Endpoint::connections(&self.url), None)
            .await;
    }
}

#[async_trait]
impl Agent for HttpAgent {
    /// Function to log the output
    fn logger(&self) -> Log {
        self.base_agent.logger
    }

    /// Gets all the connections
    async fn get_connections(&self, filter: ConnectionsConfig) -> Connections {
        let mut query: Vec<(&str, String)> = vec![];

        if let Some(alias) = filter.alias {
            query.push(("alias", alias))
        }

        if let Some(invitation_key) = filter.invitation_key {
            query.push(("invitation_key", invitation_key))
        }

        if let Some(my_did) = filter.my_did {
            query.push(("my_did", my_did))
        }

        if let Some(their_did) = filter.their_did {
            query.push(("their_did", their_did))
        }

        if let Some(their_role) = filter.their_role {
            query.push(("their_role", their_role))
        }

        if let Some(state) = filter.state {
            query.push(("state", state))
        }

        self.get::<Connections>(Endpoint::connections(&self.url), Some(query))
            .await
    }

    /// Get a connection by id
    async fn get_connection_by_id(&self, id: String) -> Connection {
        self.get::<Connection>(Endpoint::get_connection_by_id(&self.url, &id), None)
            .await
    }

    /// Prints an invitation, as url or qr, in stdout
    async fn create_invitation(&self, config: &InvitationConfig) -> Invitation {
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
            if config.multi_use {
                query.push(("multi_use", true.to_string()));
            }
            if config.auto_accept {
                query.push(("auto_accept", true.to_string()))
            }
            if let Some(alias) = &config.alias {
                query.push(("alias", alias.to_string()));
            }
        }

        self.post(Endpoint::create_invitation(&self.url), Some(query), body)
            .await
    }

    /// Requests all the features from the cloudagent
    async fn discover_features(&self) -> Features {
        self.get::<Features>(Endpoint::discover_features(&self.url), None)
            .await
    }

    /// Send a basic message to another agent
    async fn send_message(&self, config: &MessageConfig) -> () {
        let body = json!({
            "content": config.message,
        });

        self.post::<serde_json::Value>(
            Endpoint::basic_message(&self.url, &config.connection_id),
            None,
            Some(body),
        )
        .await;
    }

    async fn credential(&self, config: &IssueCredentialConfig) -> Value {
        let body = json!({
          "connection_id": config.connection_id,
          "cred_def_id": config.credential_definition_id,
          "credential_preview": {
            "@type": "issue-credential/1.0/credential-preview",
            "attributes": config.attributes
          }
        });

        self.post::<Value>(Endpoint::credential_offer(&self.url), None, Some(body))
            .await
    }

    async fn schema(&self, config: &SchemaConfig) -> Schema {
        let body = json!({
          "attributes": config.attributes,
          "schema_name": config.name,
          "schema_version": config.version
        });

        self.post::<Schema>(Endpoint::schema(&self.url), None, Some(body))
            .await
    }

    async fn credential_definition(
        &self,
        config: &CredentialDefinitionConfig,
    ) -> CredentialDefinition {
        let body = json!({
          "schema_id": config.schema_id,
          "support_revocation": false,
          "tag": config.tag
        });

        self.post::<CredentialDefinition>(
            Endpoint::credential_definition(&self.url),
            None,
            Some(body),
        )
        .await
    }

    async fn credential_definitions(&self) -> CredentialDefinitions {
        self.get::<CredentialDefinitions>(Endpoint::credential_definition_created(&self.url), None)
            .await
    }
}
