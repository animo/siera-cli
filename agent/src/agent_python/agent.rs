use async_trait::async_trait;

use crate::cloud_agent::{CloudAgent, CloudAgentExtended};

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
