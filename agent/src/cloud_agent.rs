use reqwest::Url;

use crate::error::{self, Result};

/// Cloudagent properties
#[derive(Debug, Clone)]
pub struct CloudAgent {
    /// base url of the cloudagent
    pub endpoint: String,

    /// admin Api key for the cloudagent
    pub api_key: Option<String>,
}

impl CloudAgent {
    pub fn create_url(&self, paths: Vec<&str>) -> Result<Url> {
        let mut url = Url::parse(&self.endpoint)
            .map_err(|_| Box::new(error::Error::UnreachableUrl) as Box<dyn std::error::Error>)?;
        url.set_path(&paths.join("/"));
        Ok(url)
    }
}
