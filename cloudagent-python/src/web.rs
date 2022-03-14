use crate::cloud_agent::CloudAgent;
use agent::error::{Error, Result};
use log::trace;
use reqwest::{Client, RequestBuilder, Url};
use serde::de::DeserializeOwned;
use serde_json::Value;

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

        trace!("About to send request:\n{:#?}", client);
        match client.send().await {
            Ok(res) => {
                let status_code = res.status().as_u16();
                trace!("Got {} response:\n{:#?}", status_code, res);
                match status_code {
                    200..=299 => res
                        .json()
                        .await
                        .map_err(|_| Error::UnableToParseResponse.into()),
                    // Issue credential message when attributes are not correct
                    400 => Err(res.text().await?.into()),
                    401 => Err(Error::AuthorizationFailed.into()),
                    404 => Err(Error::UrlDoesNotExist.into()),
                    // TODO: This response is quite ugly. Is it only used at cred_def_id?
                    422 => Err(res.text().await?.into()),
                    503 => Err(Error::HttpServiceUnavailable.into()),
                    500..=599 => Err(Error::InternalServerError.into()),
                    _ => Err(Error::UnknownResponseStatusCode(res.text().await?).into()),
                }
            }
            Err(e) => {
                trace!("Request failed {}", e);
                Err(Error::UnreachableUrl.into())
            }
        }
    }
}
