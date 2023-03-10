use crate::agent::CloudAgentAfjRest;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use siera_agent::error::Result;
use siera_agent::modules::connection::{
    Connection, ConnectionCreateInvitationOptions, ConnectionGetAllOptions, ConnectionModule,
    ConnectionReceiveInvitationOptions, Invitation,
};

/// Create invitation response
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// Invitation url
    pub invitation_url: String,
    /// Invitation
    pub invitation: Value,
    /// Connection record
    pub connection: Connection,
}

#[async_trait]
impl ConnectionModule for CloudAgentAfjRest {
    async fn get_all(&self, options: ConnectionGetAllOptions) -> Result<Vec<Connection>> {
        let has_defined_value = has_any_value_in_struct!(
            options;
            alias,
            connection_protocol,
            invitation_key,
            my_did,
            state,
            their_did,
            their_role
        );

        if has_defined_value {
            warn!({ "message": "Additional options are ignored on this endpoint"});
        }
        let url = self.create_url(&["connections"])?;

        self.get(url, None).await
    }

    async fn get_by_id(&self, id: String) -> Result<Connection> {
        let url = self.create_url(&["connections", &id])?;

        self.get(url, None).await
    }

    async fn create_invitation(
        &self,
        options: ConnectionCreateInvitationOptions,
    ) -> Result<Invitation> {
        let url = self.create_url(&["connections", "create-invitation"])?;

        let body = json!({
            "autoAcceptConnections": options.auto_accept,
            "alias": options.alias,
        });

        let invitation: Response = self.post(url, None, Some(body)).await?;

        Ok(Invitation {
            invitation_url: invitation.invitation_url,
            invitation: invitation.invitation,
            id: invitation.connection.id,
        })
    }

    async fn receive_invitation(
        &self,
        invitation: ConnectionReceiveInvitationOptions,
    ) -> Result<Connection> {
        let url = self.create_url(&["connections", "receive-invitation"])?;

        let body = json!({
            "invitation": {
                "label": invitation.label,
                "did": invitation.did,
                "recipientKeys": invitation.recipient_keys,
                "routingKeys": invitation.routing_keys,
                "serviceEndpoint": invitation.service_endpoint,
            },
        });

        self.post(url, None, Some(body)).await
    }
}
