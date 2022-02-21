use async_trait::async_trait;
use serde_json::{json, Value};

use crate::{
    error::Result,
    modules::credentials::{
        CredentialOfferResponse, CredentialsModule, CredentialsOfferOptions, SendProposalOptions,
    },
};

use super::agent::CloudAgentPython;

#[async_trait]
impl CredentialsModule for CloudAgentPython {
    async fn send_offer(
        &self,
        options: CredentialsOfferOptions,
    ) -> Result<CredentialOfferResponse> {
        let url = self
            .cloud_agent
            .create_url(vec!["issue-credential", "send-offer"])?;

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

        self.cloud_agent.post(url, None, Some(body)).await
    }

    async fn send_proposal(&self, _options: SendProposalOptions) -> Result<Value> {
        let _url = self
            .cloud_agent
            .create_url(vec!["issue-credential", "send-proposal"])?;

        todo!()
    }
}
