use reqwest::Url;
use siera_agent::error::{Error, Result};

/// Cloudagent Python Agent
#[derive(Debug)]
pub struct CloudAgentPython {
    /// base url of the cloudagent
    pub endpoint: String,

    /// admin Api key for the cloudagent
    pub api_key: Option<String>,

    /// Authorization token for a multi tenancy agent
    pub auth_token: Option<String>,

    /// Agent version
    pub version: CloudAgentPythonVersion,
}

/// ACA-Py supported versions
/// TODO: How do we want to deal with mulitple versions? Architecture wise.
#[derive(Debug)]
pub enum CloudAgentPythonVersion {
    /// ~0.7.3
    ZeroSevenThree,
}

impl CloudAgentPython {
    /// Create a new instance of an aries cloudagent python
    #[must_use]
    pub const fn new(
        endpoint: String,
        version: CloudAgentPythonVersion,
        api_key: Option<String>,
        auth_token: Option<String>,
    ) -> Self {
        Self {
            endpoint,
            api_key,
            auth_token,
            version,
        }
    }

    /// Create a url based on the base url and a list of paths
    ///
    /// # Errors
    ///
    /// When the url could not be constructed
    pub fn create_url(&self, paths: &[&str]) -> Result<Url> {
        let mut url = Url::parse(&self.endpoint)
            .map_err(|_| Box::new(Error::UnreachableUrl) as Box<dyn std::error::Error>)?;
        url.set_path(&paths.join("/"));
        Ok(url)
    }
}
