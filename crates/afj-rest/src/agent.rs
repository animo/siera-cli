use agent::error::{Error, Result};
use reqwest::Url;

/// AFJ REST agent
#[derive(Debug)]
pub struct CloudAgentAfjRest {
    /// base url of the cloudagent
    pub endpoint: String,

    /// admin Api key for the cloudagent
    pub api_key: Option<String>,

    /// Authorization token for a multi tenancy agent
    pub auth_token: Option<String>,

    /// Agent version
    pub version: CloudAgentAfjRestVersion,
}

/// AFJ REST supported versions
/// TODO: How do we want to deal with mulitple versions? Architecture wise.
#[derive(Debug)]
pub enum CloudAgentAfjRestVersion {
    /// ~0.8.0
    ZeroEightZero,
}

impl CloudAgentAfjRest {
    /// Create a new instance of an AFJ REST agent
    #[must_use]
    pub const fn new(
        endpoint: String,
        version: CloudAgentAfjRestVersion,
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
    /// When it could not parse the url
    pub fn create_url(&self, paths: &[&str]) -> Result<Url> {
        let mut url = Url::parse(&self.endpoint)
            .map_err(|_| Box::new(Error::UnreachableUrl) as Box<dyn std::error::Error>)?;
        url.set_path(&paths.join("/"));
        Ok(url)
    }
}
