use async_trait::async_trait;

#[async_trait]
pub trait FeaturesModule {
    /// Requests all the features from the cloudagent
    async fn discover_features(&self) -> ();
}
