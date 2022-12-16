use crate::error::Result;
use crate::help_strings::HelpStrings;
use crate::utils::loader::{Loader, LoaderVariant};
use clap::Args;
use siera_agent::modules::json_ld::{JsonLdModule, JsonLdSignOptions, JsonLdVerifyOptions};

/// JSON-LD options and flags
#[derive(Args)]
#[clap(about = HelpStrings::Message)]
pub struct JsonLdOptions {
    /// The verkey used for signing
    #[clap(short, long, help=HelpStrings::MessageMessage)]
    verkey: String,

    /// The JSON-LD doc to be signed or verified
    #[clap(short, long, help=HelpStrings::MessageId)]
    doc: Value,
}

#[derive(Subcommand, Debug)]
pub enum JsonLdSubcommands {
    // Sign a JSON-LD doc
    #[clap(about = HelpStrings::JsonLdSign)]
    Sign {
        /// The key type of the wallet
        #[clap(short, long, help=HelpStrings::JsonLdVerkey, required = true)]
        verkey: String,

        /// The doc to be signed
        /// NOTE: This is intentionally moved to the end to make it easier for the user to stream data into here
        #[clap(short, long, help=HelpStrings::JsonLdDoc, required = true)]
        doc: String,
    },
    // Verify a JSON-LD doc
    #[clap(about = HelpStrings::JsonLdVerify)]
    Verify {
        /// The key type of the wallet
        #[clap(short, long, help=HelpStrings::JsonLdVerkey, required = true)]
        verkey: String,

        /// The doc to be verified
        /// NOTE: This is intentionally moved to the end to make it easier for the user to stream data into here
        #[clap(short, long, help=HelpStrings::JsonLdDoc, required = true)]
        doc: String,
    },
}

/// Subcommand JSON-LD parser
pub async fn parse_json_ld_args(
    options: &JsonLdOptions,
    agent: impl JsonLdModule + Send + Sync,
) -> Result<()> {
    let loader = Loader::start(&LoaderVariant::default());
    match &options.commands {
        JsonLdSubcommands::Sign { verkey, doc } => {
            let options = JsonLdOptions {
                verkey: verkey.clone(),
                doc: doc.copy(),
            };
            agent.sign(options).await.map(|response| {
                loader.stop();
                log_info!(
                    "Successfully signed Document {}: ",
                    pretty_stringify_obj(&response)
                );
                log!("{}", pretty_stringify_obj(&response));
                log_json!(response)
            })
        }
        JsonLdSubcommands::Verify { verkey, doc } => {
            let options = JsonLdOptions {
                verkey: verkey.clone(),
                doc: doc.copy(),
            };
            agent.verify(options).await.map(|response| {
                loader.stop();
                log_info!(
                    "Successfully verified Document {}: ",
                    pretty_stringify_obj(&response)
                );
                log!("{}", pretty_stringify_obj(&response));
                log_json!(response)
            })
        }
    }
}
