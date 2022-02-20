use anyhow::Result;

use crate::cloud_agent::CloudAgent;

/// ACA-Py supported versions
/// TODO: How do we want to deal with mulitple versions? Architecture wise.
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

impl CloudAgentPython {
    pub async fn new(
        endpoint: impl AsRef<str>,
        api_key: Option<impl AsRef<str>>,
        version: CloudAgentPythonVersion,
    ) -> Result<Self> {
        let agent = CloudAgentPython {
            cloud_agent: CloudAgent {
                endpoint: endpoint.as_ref().to_string(),
                api_key: api_key.map(|a| a.as_ref().to_string()),
            },
            version,
        };

        Ok(agent)
    }
}
