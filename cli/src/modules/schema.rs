use agent::modules::schema::SchemaModule;
use clap::Args;

use crate::{error::Result, utils::logger::Log};

#[derive(Args)]
pub struct SchemaOptions {}

pub async fn parse_schema_args(agent: impl SchemaModule, logger: Log) -> Result<()> {
    agent
        .schema()
        .await
        .map(|schema| {
            logger.log_pretty(schema);
            ()
        })
        .map_err(|e| e.into())
}
