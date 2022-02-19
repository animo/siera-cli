use reqwest::{Client, RequestBuilder, Url};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::{
    cloud_agent::CloudAgent,
    error::{AgentResult, Error},
};

pub fn create_url(arr: Vec<&str>) -> AgentResult<Url> {
    let url = arr.join("/");
    reqwest::Url::parse(&url).map_err(|e| e.to_string())
}

/// Call logic for http calls
impl CloudAgent {
    /// Builds a get request and calls the sender
    pub async fn get<T: DeserializeOwned>(
        &self,
        url: Url,
        query: Option<Vec<(&str, String)>>,
    ) -> AgentResult<T> {
        let client = match query {
            Some(q) => Client::new().get(url).query(&q),
            None => Client::new().get(url),
        };

        self.send::<T>(client).await
    }

    /// Builds a post request and calls the sender
    pub async fn post<T: DeserializeOwned>(
        &self,
        url: Url,
        query: Option<Vec<(&str, String)>>,
        body: Option<Value>,
    ) -> AgentResult<T> {
        let client = Client::new().post(url).query(&query);

        let client = match body {
            Some(b) => client.json(&b),
            None => client,
        };

        self.send::<T>(client).await
    }

    /// Sends any request
    pub async fn send<T: DeserializeOwned>(&self, client: RequestBuilder) -> AgentResult<T> {
        let client = match &self.api_key {
            Some(a) => client.header("X-API-KEY", a),
            None => client,
        };

        match client.send().await {
            Ok(res) => {
                if res.status().is_success() {
                    return res.json::<T>().await.map_err(|e| e.to_string());
                }
                if res.status() == 401 {
                    return Err(Error::AuthenticationFailed.to_string());
                }
                Err(Error::InvalidEndpoint.to_string())
            }
            Err(e) => Err(e.to_string()),
        }
    }
}
