use async_trait::async_trait;

use crate::{
    cloud_agent::{CloudAgent, CloudAgentExtended},
    modules::{connections::ConnectionModule, features::FeaturesModule},
};

/// ACA-Py supported versions
#[derive(Debug)]
pub enum CloudAgentPythonVersion {
    /// ~0.6.0
    ZeroSixZero,
}

#[derive(Debug)]
pub struct CloudAgentPython {
    /// Default cloud agent structure
    pub cloud_agent: CloudAgent,

    /// ACA-Py version
    pub version: CloudAgentPythonVersion,
}

#[async_trait]
impl CloudAgentExtended for CloudAgentPython {
    async fn check_endpoint(&self) {
        todo!()
    }
}

impl CloudAgentPython {
    pub fn new(
        endpoint: impl AsRef<str>,
        api_key: Option<impl AsRef<str>>,
        version: CloudAgentPythonVersion,
    ) -> Self {
        CloudAgentPython {
            cloud_agent: CloudAgent {
                endpoint: endpoint.as_ref().to_string(),
                api_key: api_key.map(|a| a.as_ref().to_string()),
            },
            version,
        }
    }
}

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

#[async_trait]
impl FeaturesModule for CloudAgentPython {
    async fn discover_features(&self) -> () {
        todo!()
    }
}
