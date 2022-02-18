use async_trait::async_trait;

#[async_trait]
pub trait ConnectionModule {
    /// Gets all the connections
    async fn get_connections(&self) -> ();

    /// Get a connection by id
    async fn get_connection_by_id(&self, id: String) -> ();

    /// Create an invitation
    async fn create_invitation(&self, config: String) -> ();
}
