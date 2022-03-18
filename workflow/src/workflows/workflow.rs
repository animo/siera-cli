use agent::modules::{
    connections::ConnectionModule, credential_definition::CredentialDefinitionModule,
    credentials::CredentialsModule,
};
use async_trait::async_trait;

#[async_trait]
pub trait Workflow {
    // TODO: fix return type
    async fn execute(
        &self,
        agent: impl ConnectionModule + CredentialsModule + CredentialDefinitionModule,
    ) -> Result<(), ()>;
}
