use async_trait::async_trait;

use crate::modules::connections::ConnectionModule;

use super::agent::CloudAgentPython;

#[async_trait]
impl ConnectionModule for CloudAgentPython {
    async fn get_connections(&self) -> () {
        todo!()
    }

    async fn get_connection_by_id(&self, _id: String) -> () {
        todo!()
    }

    async fn create_invitation(&self, _config: String) -> () {
        todo!()
    }
}
