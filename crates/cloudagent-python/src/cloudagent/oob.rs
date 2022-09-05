use crate::agent::CloudAgentPython;
use agent::error::Result;
use agent::modules::oob::{
    OobConnection, OobConnectionCreateInvitationOptions, OobConnectionCreateInvitationResponse,
    OobModule,
    OobConnectionReceiveInvitationOptions,
};
use async_trait::async_trait;
use serde_json::json;

#[async_trait]
impl OobModule for CloudAgentPython {
    async fn create_invitation(
        &self,
        options: OobConnectionCreateInvitationOptions,
    ) -> Result<OobConnectionCreateInvitationResponse> {
        let url = self.create_url(&["out-of-band", "create-invitation"])?;
        let mut query: Vec<(&str, String)> = vec![];

        let body = if options.toolbox {
            query.push(("multi_use", false.to_string()));
            query.push(("auto_accept", true.to_string()));
            query.push(("alias", String::from("toolbox")));
            Some(json!({
                "handshake_protocols": [
                    "did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/didexchange/1.0",
                ] 
            }))
        } else {
            if options.multi_use {
                query.push(("multi_use", true.to_string()));
            }
            if options.auto_accept {
                query.push(("auto_accept", true.to_string()));
            }
            if let Some(alias) = &options.alias {
                query.push(("alias", alias.clone()));
            }
            if let Some(handshake_protocol) = &options.handshake_protocol {
                query.push(("handshake_protocols", json!({
                "handshake_protocols": [
                    handshake_protocol.clone(),
                ] 
                }).to_string() ));
            }
            Some(json!({
                "handshake_protocols": [
                    "did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/didexchange/1.0",
                ] 
            }))
        };

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
