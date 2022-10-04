use crate::agent::CloudAgentPython;
use agent::error::Result;
use agent::modules::proof::{ProofModule, ProofRequestOptions, ProofRequestResponse};
use async_trait::async_trait;
use serde_json::{json, Value};
use std::collections::BTreeMap;

#[async_trait]
impl ProofModule for CloudAgentPython {
    async fn send_request(&self, options: ProofRequestOptions) -> Result<ProofRequestResponse> {
        let url = self.create_url(&["present-proof", "send-request"])?;

        let mut requested_attributes: BTreeMap<String, Value> = BTreeMap::new();
        let mut requested_predicates: BTreeMap<String, Value> = BTreeMap::new();

        options.attributes.iter().for_each(|a| {
            requested_attributes.insert(a.clone(), json!({ "name": a, }));
        });

        options.predicates.iter().for_each(|p| {
            requested_predicates.insert(
                p.0.clone(),
                json!({ "name": p.0, "p_type": p.1, "p_value": p.2 }),
            );
        });

        let body = json!({
          "connection_id": options.connection_id,
          "proof_request": {
            "name": options.name,
            "version": "1.0",
            "requested_attributes": requested_attributes,
            "requested_predicates": requested_predicates,
          }
        });

        self.post(url, None, Some(body)).await
    }
}
