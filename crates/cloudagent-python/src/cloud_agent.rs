use agent::error::{Error, Result};
use reqwest::Url;

/// Cloudagent properties
#[derive(Debug)]
pub struct CloudAgent {
    /// base url of the cloudagent
    pub endpoint: String,

    /// admin Api key for the cloudagent
    pub api_key: Option<String>,

    /// Authorization token for a multi tenancy agent
    pub auth_token: Option<String>,
}

impl CloudAgent {
    /// Create a url based on the base url and a list of paths
    pub fn create_url(&self, paths: Vec<&str>) -> Result<Url> {
        let mut url = Url::parse(&self.endpoint)
            .map_err(|_| Box::new(Error::UnreachableUrl) as Box<dyn std::error::Error>)?;
        url.set_path(&paths.join("/"));
        Ok(url)
    }
}
