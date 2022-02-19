use async_trait::async_trait;

use crate::{
    error::{AgentResult, Error},
    modules::connections::{ConnectionCreateInvitationConfig, ConnectionModule},
};

use super::agent::CloudAgentPython;

#[async_trait]
impl ConnectionModule for CloudAgentPython {
    async fn get_connections(&self) -> AgentResult<()> {
        todo!()
    }

    async fn get_connection_by_id(&self, _id: String) -> AgentResult<()> {
        todo!()
    }

    async fn create_invitation(
        &self,
        _config: ConnectionCreateInvitationConfig,
    ) -> AgentResult<String> {
        Err(Error::InvalidEndpoint)
    }
}
