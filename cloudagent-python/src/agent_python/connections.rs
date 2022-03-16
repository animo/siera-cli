use super::agent::CloudAgentPython;
use agent::error::Result;
use agent::modules::connections::{
    Connection, ConnectionCreateInvitationOptions, ConnectionCreateInvitationResponse,
    ConnectionGetAllResponse, ConnectionModule, ConnectionReceiveInvitationOptions,
};
use async_trait::async_trait;
use serde_json::json;

#[async_trait]
impl ConnectionModule for CloudAgentPython {
    async fn get_all(&self) -> Result<ConnectionGetAllResponse> {
        let url = self.cloud_agent.create_url(vec!["connections"])?;
        self.cloud_agent.get(url, None).await
    }

    async fn get_by_id(&self, id: String) -> Result<Connection> {
        let url = self.cloud_agent.create_url(vec!["connections", &id])?;
        self.cloud_agent.get::<Connection>(url, None).await
    }

    async fn create_invitation(
        &self,
        options: ConnectionCreateInvitationOptions,
    ) -> Result<ConnectionCreateInvitationResponse> {
        let url = self
            .cloud_agent
            .create_url(vec!["connections", "create-invitation"])?;
        let mut query: Vec<(&str, String)> = vec![];
        let mut body = None;
        if options.toolbox {
            query.push(("multi_use", false.to_string()));
            query.push(("auto_accept", true.to_string()));
            query.push(("alias", String::from("toolbox")));

            body = Some(json!({
                "metadata": {
                    "group": "admin"
                }
            }));
        } else {
            if options.multi_use {
                query.push(("multi_use", true.to_string()));
            }
            if options.auto_accept {
                query.push(("auto_accept", true.to_string()))
            }
            if let Some(alias) = &options.alias {
                query.push(("alias", alias.to_string()));
            }
        }
        self.cloud_agent
            .post::<ConnectionCreateInvitationResponse>(url, Some(query), body)
            .await
    }
    async fn receive_invitation(
        &self,
        invitation: ConnectionReceiveInvitationOptions,
    ) -> Result<Connection> {
        let url = self
            .cloud_agent
            .create_url(vec!["connections", "receive-invitation"])?;
        self.cloud_agent
            .post(url, None, Some(serde_json::to_value(invitation)?))
            .await
    }
}
