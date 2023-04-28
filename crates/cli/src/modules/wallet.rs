use crate::error::Result;
use crate::help_strings::HelpStrings;
use crate::utils::loader::{Loader, LoaderVariant};
use clap::{Args, Subcommand};
use siera_agent::modules::wallet::{
    CreateLocalDidOptions, Did, DidList, KeyType, SetDidEndpointOptions, WalletModule,
};
use siera_logger::pretty_stringify_obj;

/// Schema options and flags
#[derive(Args)]
#[clap(about = HelpStrings::Wallet)]
pub struct WalletOptions {
    /// All the subcommands of the schema cli
    #[clap(subcommand)]
    pub commands: WalletSubcommands,
}

/// Wallet subcommands
#[derive(Subcommand, Debug)]
pub enum WalletSubcommands {
    /// Get associated did for wallet
    #[clap(about = HelpStrings::WalletList)]
    List {
        /// A DID in question
        #[clap(short, long, help=HelpStrings::WalletListDid, required = false)]
        did: Option<String>,

        /// The key type of the wallet e.g. ed25519, bls12381g2
        #[clap(short, long, help=HelpStrings::WalletListKeyType, required = false, possible_values=&["ed25519", "bls12381g2"])]
        key_type: Option<String>,

        /// The did method to query for
        #[clap(short, long, help=HelpStrings::WalletListMethod, required = false, possible_values=&["did", "sov"])]
        method: Option<String>,

        /// Available values : public, posted, wallet_only
        #[clap(short, long, help=HelpStrings::WalletListPosture, required = false, possible_values=&["posted", "public", "wallet_only"])]
        posture: Option<String>,

        /// The verification key of interest
        #[clap(short, long, help=HelpStrings::WalletListVerkey, required = false)]
        verkey: Option<String>,
    },

    /// Create a local DID
    #[clap(about = HelpStrings::WalletCreate)]
    CreateLocalDid {
        /// The method to be used did or sov
        #[clap(long, short, help=HelpStrings::WalletCreateMethod, required = true, default_value="did", possible_values=&["did", "sov"])]
        method: String,

        /// The key type e.g. ed25519 or bls12381g2
        #[clap(long, short, help=HelpStrings::WalletListKeyType, required = true, default_value="ed25519", possible_values=&["ed25519", "bls12381g2"])]
        key_type: String,
    },

    /// Rotate the wallets key pair
    #[clap(about = HelpStrings::WalletRotateKeypair)]
    RotateKeyPair {
        /// The did to rotate the keypair for
        #[clap(short, long, help=HelpStrings::WalletListDid)]
        did: String,
    },

    /// Fetch the wallets public did
    #[clap(about = HelpStrings::WalletGetPublic)]
    FetchPublicDid {},

    /// Assign a DID as public
    #[clap(about = HelpStrings::WalletSetPublic)]
    AssignPublicDid {
        /// The DID to assign
        #[clap(long, short, help=HelpStrings::WalletListDid)]
        did: String,
    },

    /// Get the DID endpoint
    #[clap(about = HelpStrings::WalletFetchDidEndpoint)]
    FetchDidEndpoint {
        /// The DID to assign
        #[clap(long, short, help=HelpStrings::WalletListDid)]
        did: String,
    },

    /// Query DID endpoint of wallet
    #[clap(about = HelpStrings::WalletSetEndpoint)]
    SetDidEndpoint {
        /// The DID to assign
        #[clap(long, short, help=HelpStrings::WalletListDid)]
        did: String,

        /// The endpoint url for the did
        #[clap(long, short, help=HelpStrings::WalletEndpoint)]
        endpoint: String,

        /// The endpoint type
        #[clap(long, short, help=HelpStrings::WalletEndpointType, default_value="Endpoint")]
        endpoint_type: String,
    },
}

/// Subcommand Schema parser
pub async fn parse_wallet_args(
    options: &WalletOptions,
    agent: impl WalletModule + Send + Sync,
) -> Result<()> {
    let loader = Loader::start(&LoaderVariant::default());
    match &options.commands {
        WalletSubcommands::List {
            did,
            key_type,
            method,
            posture,
            verkey,
        } => {
            let options = Did {
                did: did.clone(),
                key_type: key_type.clone(),
                method: method.clone(),
                posture: posture.clone(),
                verkey: verkey.clone(),
            };
            agent
                .get_wallet_dids(options)
                .await
                .map(|response: DidList| {
                    loader.stop();
                    info!({ "message": "Found the following DID information"});
                    log!({ "response": response });
                    copy!("{}", pretty_stringify_obj(&response));
                })
        }
        WalletSubcommands::CreateLocalDid { method, key_type } => {
            let options = CreateLocalDidOptions {
                method: method.clone(),
                options: KeyType {
                    key_type: key_type.clone(),
                },
            };
            agent.create_local_did(options).await.map(|response| {
                loader.stop();
                info!({ "message": format!("Successfully created local DID: {:?}", response.did) });
                log!({ "response": response });
                copy!("{}", pretty_stringify_obj(&response));
            })
        }
        WalletSubcommands::RotateKeyPair { did } => {
            agent.rotate_keypair(did.clone()).await.map(|response| {
                loader.stop();
                info!({ "message": format!("Successfully rotated keypair for DID: {did}") });
                log!({ "response": response });
                copy!("{}", pretty_stringify_obj(response));
            })
        }
        WalletSubcommands::FetchPublicDid {} => agent.fetch_public_did().await.map(|response| {
            loader.stop();
            info!({ "message": "Wallet public DID" });
            log!({ "response": response });
            copy!("{}", pretty_stringify_obj(&response));
        }),
        WalletSubcommands::AssignPublicDid { did } => {
            agent.assign_public_did(did.clone()).await.map(|response| {
                loader.stop();
                info!({ "message": "Successfully assigned public DID" });
                log!({ "response": response });
                copy!("{}", pretty_stringify_obj(&response));
            })
        }
        WalletSubcommands::FetchDidEndpoint { did } => {
            agent.fetch_did_endpoint(did.clone()).await.map(|response| {
                loader.stop();
                info!({ "message": format!("DID endpoint for DID: {did}") });
                log!({ "response": response });
                copy!("{}", pretty_stringify_obj(&response));
            })
        }
        WalletSubcommands::SetDidEndpoint {
            did,
            endpoint,
            endpoint_type,
        } => {
            let options = SetDidEndpointOptions {
                did: did.clone(),
                endpoint: endpoint.clone(),
                endpoint_type: endpoint_type.clone(),
            };
            agent.set_did_endpoint(options).await.map(|response| {
                loader.stop();
                info!({ "message": format!("Set DID endpoint for DID: {did}") });
                log!({ "response": response });
                copy!("{}", pretty_stringify_obj(response));
            })
        }
    }
}
