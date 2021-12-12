use crate::agent::agents::Agent;
use crate::typing::ConnectionsConfig;
use crate::utils::logger::Log;

pub async fn run(agent: &dyn Agent, config: ConnectionsConfig<'_>) {
    match config.id {
        Some(i) => {
            let connection = agent.get_connection_by_id(i).await;

            Log::pretty_log(&connection);
        }
        None => {
            let connections = agent.get_connections(config.alias).await.results;

            Log::pretty_log(&connections);
        }
    };
}
