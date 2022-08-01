use crate::agent::CloudAgentPython;
use agent::error::Result;
use agent::modules::credential::{
    CredentialModule, CredentialOfferOptions, CredentialOfferResponse,
};
use async_trait::async_trait;
use serde_json::json;

#[async_trait]
impl CredentialModule for CloudAgentPython {
    async fn send_offer(&self, options: CredentialOfferOptions) -> Result<CredentialOfferResponse> {
        let url = self.create_url(&["issue-credential", "send-offer"])?;

        let mut attributes = vec![];
        for (i, key) in options.keys.iter().enumerate() {
            let value = &options.values[i];
            attributes.push(json!({"name": key, "value": value}));
        }

        let body = json!({
          "connection_id": options.connection_id,
          "cred_def_id": options.cred_def_id,
          "credential_preview": {
            "@type": "issue-credential/1.0/credential-preview",
            "attributes": attributes,
          },
        });

        self.post(url, None, Some(body)).await
    }
}
