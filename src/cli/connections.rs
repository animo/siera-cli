use crate::agent::agents::Agent;
use crate::typing::ConnectionsConfig;
use crate::utils::logger::Log;
use serde::Serialize;

pub async fn run(agent: &dyn Agent, config: ConnectionsConfig<'_>) {
    match config.id {
        Some(i) => {
            let connection = agent.get_connection_by_id(i).await;

            let buf = Vec::new();
            let formatter = serde_json::ser::PrettyFormatter::with_indent(b"  ");
            let mut ser = serde_json::Serializer::with_formatter(buf, formatter);

            connection.serialize(&mut ser).unwrap();

            Log::log(&String::from_utf8(ser.into_inner()).unwrap());
        }
        None => {
            let connections = agent.get_connections(config.alias).await.results;

            let buf = Vec::new();
            let formatter = serde_json::ser::PrettyFormatter::with_indent(b"  ");
            let mut ser = serde_json::Serializer::with_formatter(buf, formatter);

            connections.serialize(&mut ser).unwrap();

            Log::log(&String::from_utf8(ser.into_inner()).unwrap());
        }
    };
}
