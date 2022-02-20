/// Cloudagent properties
#[derive(Debug, Clone)]
pub struct CloudAgent {
    /// base url of the cloudagent
    pub endpoint: String,

    /// admin Api key for the cloudagent
    pub api_key: Option<String>,
}
