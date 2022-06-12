use agent::error::{Error, Result};
use reqwest::Url;

/// Cloudagent Python Agent
#[derive(Debug, Default)]
pub struct CloudAgentPython {
    /// base url of the cloudagent
    pub endpoint: String,

    /// admin Api key for the cloudagent
    pub api_key: Option<String>,

    /// Authorization token for a multi tenancy agent
    pub auth_token: Option<String>,
}

/// ACA-Py supported versions
/// TODO: How do we want to deal with mulitple versions? Architecture wise.
#[derive(Debug)]
pub enum CloudAgentPythonVersion {
    /// ~0.7.3
    ZeroSevenThree,
}

impl From<CloudAgentPythonVersion> for String {
    fn from(version: CloudAgentPythonVersion) -> Self {
        let s = match version {
            CloudAgentPythonVersion::ZeroSevenThree => "0.7.3",
        };

        Self::from(s)
    }
}

impl CloudAgentPython {
    /// Create a new instance of an aries cloudagent python
    pub fn new(endpoint: String, api_key: Option<String>, auth_token: Option<String>) -> Self {
        Self {
            endpoint,
            api_key,
            auth_token,
        }
    }

    /// Create a url based on the base url and a list of paths
    pub fn create_url(&self, paths: Vec<&str>) -> Result<Url> {
        let mut url = Url::parse(&self.endpoint)
            .map_err(|_| Box::new(Error::UnreachableUrl) as Box<dyn std::error::Error>)?;
        url.set_path(&paths.join("/"));
        Ok(url)
    }
}
