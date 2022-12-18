use crate::agent::CloudAgentPython;
use crate::fill_query;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use siera_agent::error::Result;
use siera_agent::modules::wallet::{
    CreateLocalDidOptions, Did, DidEndpoint, DidList, SetDidEndpointOptions, WalletModule,
};

/// Response from the cloudagent that contains the wrapped Did result
#[derive(Serialize, Deserialize, Debug)]
struct Response {
    /// Wallet wrapper
    result: Did,
}

/// Response from the cloudagent when requesting info about dids
/// of a wallet specific to Aca-py
#[derive(Debug, Deserialize, Serialize)]
pub struct DidListResults {
    /// The aca-py structure returning results
    results: DidList,
}

#[async_trait]
impl WalletModule for CloudAgentPython {
    async fn get_wallet_dids(&self, options: Did) -> Result<DidList> {
        let url = self.create_url(&["wallet", "did"])?;

        let query = fill_query!(options, did, key_type, method, posture, verkey);

        let did_list: DidListResults = self.get(url, Some(query)).await?;

        Ok(did_list.results)
    }

    async fn create_local_did(&self, options: CreateLocalDidOptions) -> Result<Did> {
        let url = self.create_url(&["wallet", "did", "create"])?;

        let body = json!({
            "method": options.method,
            "options": options.options
        });

        self.post(url, None, Some(body)).await
    }

    async fn rotate_keypair(&self, did: String) -> Result<()> {
        let url = self.create_url(&["wallet", "did", "local", "rotate-keypair"])?;

        self.patch(url, Some(Vec::from([("did", did)]))).await
    }

    async fn fetch_public_did(&self) -> Result<Did> {
        let url = self.create_url(&["wallet", "did", "public"])?;

        self.get(url, None).await
    }

    async fn assign_public_did(&self, did: String) -> Result<Did> {
        let url = self.create_url(&["wallet", "did", "public"])?;

        self.post(url, Some(Vec::from([("did", did)])), None).await
    }

    async fn fetch_did_endpoint(&self, did: String) -> Result<DidEndpoint> {
        let url = self.create_url(&["wallet", "fetch-did-endpoint"])?;

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
