use crate::error::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Options that are supplied when querying a wallet for DIDs
#[derive(Debug, Deserialize, Serialize)]
pub struct Did {
    /// The DID of interest
    pub did: Option<String>,

    // TODO: enum
    /// The key type to query for eg. ed25519, bls12381g2
    pub key_type: Option<String>,

    // TODO: enum
    /// DID method to query for. e.g. sov to only fetch indy/sov DIDs Available values : key, sov
    pub method: Option<String>,

    // TODO: enum
    /// Whether DID is current public DID, posted to ledger but current public DID, or local to the wallet
    /// Available values : public, posted, wallet_only
    pub posture: Option<String>,

    /// The verification key of interest
    pub verkey: Option<String>,
}

/// Key type in a JSON format k,v pair
#[derive(Debug, Deserialize, Serialize)]
pub struct KeyType {
    // TODO: enum
    /// The key type to query for eg. ed25519, bls12381g2
    pub key_type: String,
}

/// Options that are supplied when querying a wallet for DIDs
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateLocalDidOptions {
    /// DID method to query for. e.g. sov to only fetch indy/sov DIDs Available values : key, sov
    pub method: String,

    /// The key type to query for eg. ed25519, bls12381g2
    pub options: KeyType,
}

/// Options that are supplied when querying a wallet for DIDs
#[derive(Debug, Deserialize, Serialize)]
pub struct DidEndpoint {
    /// The DID of interest
    pub did: String,

    /// The endpoint url
    pub endpoint: Option<String>,
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
    async fn get_wallet_dids(&self, options: Did) -> Result<Vec<Did>>;

    /// Create a local DID
    async fn create_local_did(&self, options: CreateLocalDidOptions) -> Result<Did>;

    /// Rotate key pair
    async fn rotate_keypair(&self, did: String) -> Result<()>;

    /// Fetch public did
    async fn fetch_public_did(&self) -> Result<Did>;

    /// Assign the current public DID
    async fn assign_public_did(&self, did: String) -> Result<Did>;

    /// Query DID endpoint of wallet
    async fn fetch_did_endpoint(&self, did: String) -> Result<DidEndpoint>;

    /// Set DID endpoint of wallet
    async fn set_did_endpoint(&self, options: SetDidEndpointOptions) -> Result<()>;
}
