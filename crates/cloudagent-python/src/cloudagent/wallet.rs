use crate::agent::CloudAgentPython;
use crate::fill_query;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use siera_agent::error::Result;
use siera_agent::modules::wallet::{
    CreateLocalDidOptions, DidEndpointResult, DidList, DidSpec, SetDidEndpointOptions,
    SingleDidResultResponse, WalletModule,
};

/// Response from the cloudagent that contains the wrapped schema
#[derive(Serialize, Deserialize, Debug)]
struct Response {
    /// Wallet wrapper
    result: DidSpec,
}

#[async_trait]
impl WalletModule for CloudAgentPython {
    async fn get_wallet_dids(&self, options: DidSpec) -> Result<DidList> {
        let url = self.create_url(&["wallet", "did"])?;

        let query = fill_query!(options, did, key_type, method, posture, verkey);

        let did_list: DidList = self.get(url, Some(query)).await?;

        Ok(did_list)
    }

    async fn create_local_did(
        &self,
        options: CreateLocalDidOptions,
    ) -> Result<SingleDidResultResponse> {
        let url = self.create_url(&["wallet", "did", "create"])?;

        let body = json!({

            "method": options.method,
            "options": options.options
        });

        self.post(url, None, Some(body)).await
    }

    async fn rotate_keypair(&self, did: String) -> Result<()> {
        let url = self.create_url(&["wallet", "did", "local", "rotate-keypair"])?;

        let _rotated_keypair: SingleDidResultResponse =
            self.patch(url, Some(Vec::from([("did", did)]))).await?;

        Ok(())
    }

    async fn fetch_public_did(&self) -> Result<DidSpec> {
        let url = self.create_url(&["wallet", "did", "public"])?;

        let public_did: SingleDidResultResponse = self.get(url, None).await?;

        Ok(public_did.result)
    }

    async fn assign_public_did(&self, did: String) -> Result<DidSpec> {
        let url = self.create_url(&["wallet", "did", "public"])?;

        let public_did_assign_result: SingleDidResultResponse = self
            .post(url, Some(Vec::from([("did", did)])), None)
            .await?;

        Ok(public_did_assign_result.result)
    }

    async fn fetch_did_endpoint(&self, did: String) -> Result<DidEndpointResult> {
        let url = self.create_url(&["wallet", "fetch-did-endpoint"])?;

        let did_endpoint: DidEndpointResult =
            self.get(url, Some(Vec::from([("did", did)]))).await?;

        Ok(did_endpoint)
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
