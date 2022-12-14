use crate::error::Result;
use crate::help_strings::HelpStrings;
use crate::utils::loader::{Loader, LoaderVariant};
use clap::Args;
use siera_agent::modules::feature::FeatureModule;
use siera_logger::{pretty_stringify_obj, LogLevel};

/// Automation options and flags
#[derive(Args)]
#[clap(about = HelpStrings::Features)]
pub struct FeaturesOptions {}

/// Subcommand Feature parser
pub async fn parse_features_args(agent: impl FeatureModule + Send + Sync) -> Result<()> {
    let log_level = &::siera_logger::STATE.read().unwrap().level;
    let loader: Option<Loader> = match log_level {
        LogLevel::Json => None,
        _ => Loader::start(&LoaderVariant::default()).into(),
    };
    agent.discover_features().await.map(|features| {
        if let Some(l) = loader {
            l.stop()
        }
        log_debug!("{}", pretty_stringify_obj(&features));
        log_json!("{}", pretty_stringify_obj(&features));
        features.disclose.protocols.iter().for_each(|p| {
            log!("{}", p.pid);
        });
    })
}
