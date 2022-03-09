use super::agent::CloudAgentPython;
use agent::error::Result;
use agent::modules::connections::{
    ConnectionCreateInvitationOptions, ConnectionModule, GetConnectionByIdResponse,
    GetConnectionsResponse, Invitation,
};
use async_trait::async_trait;
use serde_json::json;

#[async_trait]
impl ConnectionModule for CloudAgentPython {
    async fn get_connections(&self) -> Result<GetConnectionsResponse> {
        let url = self.cloud_agent.create_url(vec!["connections"])?;
        self.cloud_agent.get(url, None).await
    }

    async fn get_connection_by_id(&self, id: String) -> Result<GetConnectionByIdResponse> {
        let url = self.cloud_agent.create_url(vec!["connections", &id])?;
        self.cloud_agent
            .get::<GetConnectionByIdResponse>(url, None)
            .await
    }

    async fn create_invitation(
        &self,
        options: ConnectionCreateInvitationOptions,
    ) -> Result<(String, String)> {
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
        let invite = self
            .cloud_agent
            .post::<Invitation>(url, Some(query), body)
            .await?;

        Ok((invite.connection_id, invite.invitation_url))
    }
}
