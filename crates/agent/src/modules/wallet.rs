use crate::error::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Options that are supplied when querying a wallet for DIDs
#[derive(Debug, Deserialize, Serialize)]
pub struct ListWalletOptions {
    /// The DID of interest
    pub did: String,

    /// The key type to query for eg. ed25519, bls1238g2
    pub key_type: String,

    /// DID method to query for. e.g. sov to only fetch indy/sov DIDs Available values : key, sov
    pub method: String,

    /// Whether DID is current public DID, posted to ledger but current public DID, or local to the wallet
    /// Available values : public, posted, wallet_only
    pub posture: String,

    /// The verification key of interest
    pub verkey: String,
}

/// Response from the cloudagent when requesting info about dids
/// of a wallet
#[derive(Debug, Deserialize, Serialize)]
pub struct DidList {
    /// List of all the ids of every schema that the cloudagent has registered
    pub results: Vec<ListWalletOptions>,
}

/// Response from the cloudagent when requesting info about dids
/// of a wallet
#[derive(Debug, Deserialize, Serialize)]
pub struct SingleDidResultResponse {
    /// Single definition information about a DID of a wallet
    pub result: ListWalletOptions,
}

/// Key type in a JSON format k,v pair
#[derive(Debug, Deserialize, Serialize)]
pub struct KeyType {
    pub key_type: String,
}

/// Options that are supplied when querying a wallet for DIDs
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateLocalDidOptions {
    /// DID method to query for. e.g. sov to only fetch indy/sov DIDs Available values : key, sov
    pub method: String,

    /// The key type to query for eg. ed25519, bls1238g2
    pub options: KeyType,
}

/// Options that are supplied when querying a wallet for DIDs
#[derive(Debug, Deserialize, Serialize)]
pub struct DidEndpointResult {
    /// The DID of interest
    pub did: String,

    /// The endpoint url
    pub endpoint: String,
}

/// Options that are supplied when querying a wallet for DIDs
#[derive(Debug, Deserialize, Serialize)]
pub struct SetDidEndpointOptions {
    /// The DID of interest
    pub did: String,

    /// The endpoint url
    pub endpoint: String,

    ///The endpoint type eg. 'Endpoint'
    pub endpoint_type: String,
}

/// Generic cloudagent basic message module
#[async_trait]
pub trait WalletModule {
    /// Query a wallet for DIDs
    async fn get_wallet_dids(&self, options: ListWalletOptions) -> Result<DidList>;

    /// Create a local DID
    async fn create_local_did(
        &self,
        options: CreateLocalDidOptions,
    ) -> Result<SingleDidResultResponse>;

    /// Rotate key pair
    async fn rotate_keypair(&self, did: String) -> Result<()>;

    /// Fetch public did
    async fn fetch_public_did(&self) -> Result<SingleDidResultResponse>;

    /// Assign the current public DID
    async fn assign_public(&self, did: String) -> Result<SingleDidResultResponse>;

    /// Query DID endpoint of wallet
    async fn get_did_endpoint(&self, did: String) -> Result<DidEndpointResult>;

    /// Set DID endpoint of wallet
    async fn set_did_endpoint(&self, options: SetDidEndpointOptions) -> Result<()>;
}
