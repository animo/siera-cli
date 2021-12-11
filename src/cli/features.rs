use crate::agent::agents::Agent;
use crate::utils::logger::Log;

pub async fn run(agent: &dyn Agent) {
    let features = agent.discover_features().await;
    for (_, item) in features.results.iter().enumerate() {
        Log::log(item.0);
    }
}
