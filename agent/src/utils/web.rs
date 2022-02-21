use crate::error::{self, Result};
use reqwest::{Client, RequestBuilder, Url};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::cloud_agent::CloudAgent;

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
        //       most likely this is from the unable to parse response.
        // tip: bail! is like format! so we can make some cool stuff here
        match client.send().await {
            Ok(res) => match res.status().as_u16() {
                200..=299 => res.json::<T>().await.map_err(|e| {
                    println!("{:?}", e);
                    return error::Error::UnableToParseResponse.into();
                }),
                401 => Err(error::Error::AuthorizationFailed.into()),
                404 => Err(error::Error::UrlDoesNotExist.into()),
                // TODO: This response is quite ugly. Is it only used at cred_def_id?
                422 => Err(res.text().await?.into()),
                500..=599 => Err(error::Error::InternalServerError.into()),
                _ => Err(error::Error::UnknownResponseStatusCode.into()),
            },
            Err(_) => Err(error::Error::UnreachableUrl.into()),
        }
    }
}
