use anyhow::{bail, Error, Result};
use reqwest::{Client, RequestBuilder, StatusCode, Url};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::{cloud_agent::CloudAgent, error};

pub fn create_url(arr: Vec<&str>) -> Result<Url> {
    let url = arr.join("/");
    //remove
    reqwest::Url::parse(&url).map_err(|e| Error::new(e))
}

/// Call logic for http calls
impl CloudAgent {
    /// Builds a get request and calls the sender
    pub async fn get<T: DeserializeOwned>(
        &self,
        url: Url,
        query: Option<Vec<(&str, String)>>,
    ) -> Result<T> {
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
    ) -> Result<T> {
        let client = Client::new().post(url).query(&query);

        let client = match body {
            Some(b) => client.json(&b),
            None => client,
        };

        self.send::<T>(client).await
    }

    /// Sends any request
    pub async fn send<T: DeserializeOwned>(&self, client: RequestBuilder) -> Result<T> {
        let client = match &self.api_key {
            Some(a) => client.header("X-API-KEY", a),
            None => client,
        };

        // TODO: check errors. we want to have some response from the server logged.
        //       like with issue credential which fields are missing
        match client.send().await {
            Ok(res) => match res.status() {
                StatusCode::OK => res
                    .json::<T>()
                    .await
                    .or_else(|_| bail!(error::Error::UnableToParseResponse)),
                StatusCode::UNAUTHORIZED => bail!(error::Error::AuthorizationFailed),
                StatusCode::NOT_FOUND => bail!(error::Error::UrlDoesNotExist),
                StatusCode::INTERNAL_SERVER_ERROR => bail!(error::Error::InternalServerError),
                _ => bail!(error::Error::UnknownResponseStatusCode),
            },
            Err(e) => Err(Error::new(e)),
        }
    }
}
