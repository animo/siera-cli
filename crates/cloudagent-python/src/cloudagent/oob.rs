use crate::agent::CloudAgentPython;
use async_trait::async_trait;
use serde_json::json;
use siera_agent::error::Result;
use siera_agent::modules::oob::{
    OobConnection, OobConnectionCreateInvitationOptions, OobConnectionCreateInvitationResponse,
    OobConnectionReceiveInvitationOptions, OobModule,
};

#[async_trait]
impl OobModule for CloudAgentPython {
    async fn create_invitation(
        &self,
        options: OobConnectionCreateInvitationOptions,
    ) -> Result<OobConnectionCreateInvitationResponse> {
        let url = self.create_url(&["out-of-band", "create-invitation"])?;
        let mut query: Vec<(&str, String)> = vec![];

        if options.multi_use {
            query.push(("multi_use", true.to_string()));
        }
        if options.auto_accept {
            query.push(("auto_accept", true.to_string()));
        }
        if let Some(alias) = &options.alias {
            query.push(("alias", alias.clone()));
        }

        let body = Some(json!({
            "handshake_protocols": [
                options.handshake_protocol,
            ]
        }));

        self.post::<OobConnectionCreateInvitationResponse>(url, Some(query), body)
            .await
    }
    async fn receive_invitation(
        &self,
        invitation: OobConnectionReceiveInvitationOptions,
    ) -> Result<OobConnection> {
        let url = self.create_url(&["out-of-band", "receive-invitation"])?;

        self.post(url, None, Some(serde_json::to_value(invitation)?))
            .await
    }
}
