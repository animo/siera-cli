use crate::cloud_agent::CloudAgent;

/// ACA-Py supported versions
/// TODO: How do we want to deal with mulitple versions? Architecture wise.
#[derive(Debug)]
pub enum CloudAgentPythonVersion {
    /// ~0.7.3
    ZeroSevenThree,
}

/// Generic Aries cloudagent python structure
#[derive(Debug)]
pub struct CloudAgentPython {
    /// Default cloud agent structure
    pub cloud_agent: CloudAgent,

    /// ACA-Py version
    pub version: CloudAgentPythonVersion,
}

impl CloudAgentPython {
    /// Create a new instance of a  `CloudAgentPython`
    pub fn new(
        endpoint: impl AsRef<str>,
        api_key: Option<impl AsRef<str>>,
        auth_token: Option<String>,
        version: CloudAgentPythonVersion,
    ) -> Self {
        CloudAgentPython {
            cloud_agent: CloudAgent {
                endpoint: endpoint.as_ref().to_string(),
                api_key: api_key.map(|a| a.as_ref().to_string()),
                auth_token,
            },
            version,
        }
    }
}
