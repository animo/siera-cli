use clap::Args;

use crate::error::Result;
use crate::utils::logger::Log;

#[derive(Args)]
pub struct ConfigurationOptions {
    #[clap(short, long)]
    initialise: bool,
}

// TODO: we should implement `from` so we can use todo and have a cleaner api
pub async fn parse_configuration_args(options: &ConfigurationOptions, _logger: Log) -> Result<()> {
    if options.initialise {
        initialise()?
    }

    Ok(())
}

fn initialise() -> Result<()> {
    Ok(())
}
