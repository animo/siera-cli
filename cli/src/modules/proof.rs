use crate::copy;
use crate::error::{Error, Result};
use crate::help_strings::HelpStrings;
use crate::utils::loader::{Loader, LoaderVariant};
use crate::utils::logger::pretty_stringify_obj;
use agent::modules::proof::{Predicate, ProofModule, ProofRequestOptions};
use clap::{Args, Subcommand};
use colored::*;
use log::{debug, info};

#[derive(Args)]
pub struct ProofOptions {
    #[clap(subcommand)]
    pub commands: ProofSubcommands,
}

#[derive(Subcommand, Debug)]
#[clap(about = HelpStrings::Proof)]
pub enum ProofSubcommands {
    #[clap(about = HelpStrings::ProofRequest)]
    Request {
        #[clap(short, long, help = HelpStrings::ProofRequestConnectionId)]
        connection_id: String,
        #[clap(short, long, default_value = "proof-request", help = HelpStrings::ProofRequestName)]
        name: String,
        #[clap(short, long, help = HelpStrings::ProofRequestPredicate)]
        predicate: Vec<Predicate>,
        #[clap(short, long, help = HelpStrings::ProofRequestAttribute)]
        attribute: Vec<String>,
    },
}

pub async fn parse_proof_args(commands: &ProofSubcommands, agent: impl ProofModule) -> Result<()> {
    let loader = Loader::start(LoaderVariant::default());
    match commands {
        ProofSubcommands::Request {
            connection_id,
            attribute,
            predicate,
            name,
        } => {
            let predicate: Result<Vec<(String, String, i32)>> = predicate
                .iter()
                .map(|p| {
                    Ok((
                        p.0.to_owned(),
                        p.1.to_owned(),
                        p.2.parse::<i32>().map_err(|_| {
                            Error::PredicateValueNonNumber(p.0.to_owned(), p.2.to_owned())
                        })?,
                    ))
                })
                .collect();
            let proof_request_options = ProofRequestOptions {
                connection_id: connection_id.to_owned(),
                name: name.to_owned(),
                attributes: attribute.to_vec(),
                predicates: predicate?,
            };
            agent
                .send_request(proof_request_options)
                .await
                .map(|proof| {
                    debug!("{}", pretty_stringify_obj(&proof));
                    info!(
                        "{} requested a proof. proof exchange id: ",
                        "Sucessefully".green()
                    );
                    println!("{}", &proof.presentation_exchange_id);
                    copy!("{}", &proof.presentation_exchange_id);
                })?;
            loader.stop();
            Ok(())
        }
    }
}
