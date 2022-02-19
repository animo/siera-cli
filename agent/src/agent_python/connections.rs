use async_trait::async_trait;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

use crate::error::AgentResult;
use crate::modules::connections::{
    ConnectionCreateInvitationConfig, ConnectionEndpoints, ConnectionModule,
};
use crate::utils::web::create_url;

use super::agent::CloudAgentPython;

/// Type of the received invitation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Invitation {
    /// Connection id
    pub connection_id: String,

    /// Invitation object
    pub invitation: Map<String, Value>,

    /// Invitation url that can be used to accept it by another party
    pub invitation_url: String,

    /// Alias for the given invitation
    pub alias: Option<String>,
}

#[async_trait]
impl ConnectionModule for CloudAgentPython {
    async fn get_connections(&self) -> AgentResult<()> {
        todo!()
    }

    async fn get_connection_by_id(&self, _id: String) -> AgentResult<()> {
        todo!()
    }

    async fn create_invitation(
        &self,
        config: ConnectionCreateInvitationConfig,
    ) -> AgentResult<String> {
        let endpoint = self.endpoint_create_invitation()?;
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
        let invite = self
            .cloud_agent
            .post::<Invitation>(endpoint, Some(query), body)
            .await?;

        Ok(invite.invitation_url)
    }
}

impl ConnectionEndpoints for CloudAgentPython {
    /// base + connections
    fn endpoint_get_connections(&self) -> AgentResult<Url> {
        create_url(vec![&self.cloud_agent.endpoint, "connections"])
    }
    /// base + connections + :id
    fn endpoint_get_connection_by_id(&self, id: &str) -> AgentResult<Url> {
        create_url(vec![&self.cloud_agent.endpoint, "connections", id])
    }
    /// base + connections + create-invitation
    fn endpoint_create_invitation(&self) -> AgentResult<Url> {
        create_url(vec![
            &self.cloud_agent.endpoint,
            "connections",
            "create-invitation",
        ])
    }
}
