use crate::agent::CloudAgentPython;
use crate::fill_query;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use siera_agent::error::Result;

/// Response from the cloudagent when requesting info about dids
/// of a wallet
#[derive(Debug, Deserialize, Serialize)]
pub struct DidList {
    /// List of all the ids of every schema that the cloudagent has registered
    pub results: Vec<Did>,
}

/// Response from the cloudagent when requesting info about dids
/// of a wallet
#[derive(Debug, Deserialize, Serialize)]
pub struct DidResult {
    /// Single definition information about a DID of a wallet
    pub result: Did,
}

#[async_trait]
impl WalletModule for CloudAgentPython {
    async fn get_wallet_dids(&self, options: Did) -> Result<Vec<Did>> {
        let url = self.create_url(&["wallet", "did"])?;

        let query = fill_query!(options, did, key_type, method, posture, verkey);

        let did_list: DidList = self.get(url, Some(query)).await?;

        Ok(did_list.results)
    }

    async fn create_local_did(&self, options: CreateLocalDidOptions) -> Result<Did> {
        let url = self.create_url(&["wallet", "did", "create"])?;

        let body = json!({

            "method": options.method,
            "options": options.options
        });

        let did_result: DidResult = self.post(url, None, Some(body)).await?;

        Ok(did_result.result)
    }

    async fn rotate_keypair(&self, did: String) -> Result<()> {
        let url = self.create_url(&["wallet", "did", "local", "rotate-keypair"])?;

        self.patch(url, Some(Vec::from([("did", did)]))).await
    }

    async fn fetch_public_did(&self) -> Result<Did> {
        let url = self.create_url(&["wallet", "did", "public"])?;

        let public_did: DidResult = self.get(url, None).await?;

        Ok(public_did.result)
    }

    async fn assign_public_did(&self, did: String) -> Result<Did> {
        let url = self.create_url(&["wallet", "did", "public"])?;

        let public_did_assign_result: DidResult = self
            .post(url, Some(Vec::from([("did", did)])), None)
            .await?;

        Ok(public_did_assign_result.result)
    }

    async fn fetch_did_endpoint(&self, did: String) -> Result<DidEndpoint> {
        let url = self.create_url(&["wallet", "get-did-endpoint"])?;

        self.get(url, Some(Vec::from([("did", did)]))).await
    }

    async fn set_did_endpoint(&self, options: SetDidEndpointOptions) -> Result<()> {
        let url = self.create_url(&["wallet", "set-did-endpoint"])?;

        let body = json!({
            "did": options.did,
            "endpoint": options.endpoint,
            "endpoint_type": options.endpoint_type
        });

        self.post(url, None, Some(body)).await
    }
}
