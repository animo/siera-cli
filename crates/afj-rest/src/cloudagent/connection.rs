use crate::agent::CloudAgentAfjRest;
use agent::error::Result;
use agent::modules::connection::{
    Connection, ConnectionCreateInvitationOptions, ConnectionCreateInvitationResponse,
    ConnectionGetAllOptions, ConnectionModule, ConnectionReceiveInvitationOptions,
};
use async_trait::async_trait;
use serde_json::json;

#[async_trait]
impl ConnectionModule for CloudAgentAfjRest {
    async fn get_all(&self, options: ConnectionGetAllOptions) -> Result<Vec<Connection>> {
        let url = self.create_url(vec!["connections"])?;

        self.get(url, None).await
    }

    async fn get_by_id(&self, id: String) -> Result<Connection> {
        let url = self.create_url(vec!["connections", &id])?;

        self.get(url, None).await
    }

    async fn create_invitation(
        &self,
        options: ConnectionCreateInvitationOptions,
    ) -> Result<ConnectionCreateInvitationResponse> {
        let url = self.create_url(vec!["connections", "create-invitation"])?;

        let body = json!({
            "autoAcceptConnections": options.auto_accept,
            "alias": options.alias,

        });

        self.post(url, None, Some(body)).await
    }

    async fn receive_invitation(
        &self,
        invitation: ConnectionReceiveInvitationOptions,
    ) -> Result<Connection> {
        let url = self.create_url(vec!["connections", "receive-invitation"])?;

        let body = json!({
            "autoAcceptConnections": true,
        });

        self.post(url, None, Some(body)).await
    }
}
