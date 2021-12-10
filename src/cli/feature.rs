use crate::agent::agent::Agent;
use crate::utils::logger::Log;
use crate::HttpAgent;

pub async fn run(agent: &HttpAgent) {
    let features = agent.discover_features().await;
    for (_, item) in features.results.iter().enumerate() {
        Log::log(item.0);
    }
}
