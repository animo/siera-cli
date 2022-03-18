use std::collections::HashMap;

use super::workflow::Workflow;
use agent::modules::{connections::ConnectionModule, credentials::CredentialsModule};
use async_trait::async_trait;

struct CredentialOfferWorkflow {
    connection_id: String,
    credential_deifnition_id: String,
    attributes: HashMap<String, String>,
}

#[async_trait]
impl Workflow for CredentialOfferWorkflow {
    async fn execute(&self, agent: impl ConnectionModule + CredentialsModule) -> Result<(), ()> {
        todo!()
    }
}
