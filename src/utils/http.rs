use crate::agent::http_agent::HttpAgent;
use crate::error::{throw, throw_from_http, Error};
use crate::utils::logger::Log;
use async_trait::async_trait;
use reqwest::{Client, RequestBuilder, Url};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// Interface providing HTTP call methods
#[async_trait]
pub trait HttpCalls {
    /// GET method
    async fn get<T: DeserializeOwned>(&self, url: Url, query: Option<Vec<(&str, String)>>) -> T;
    /// POST method
    async fn post<T: DeserializeOwned>(
        &self,
        url: Url,
        query: Option<Vec<(&str, String)>>,
        body: Option<Value>,
    ) -> T;
    /// SEND - general method for GET and POST
    async fn send<T: DeserializeOwned>(&self, client: RequestBuilder) -> T;
}

/// Call logic for http calls
#[async_trait]
impl HttpCalls for HttpAgent {
    /// Builds a get request and calls the sender
    async fn get<T: DeserializeOwned>(&self, url: Url, query: Option<Vec<(&str, String)>>) -> T {
        let client = match query {
            Some(q) => Client::new().get(url).query(&q),
            None => Client::new().get(url),
        };

        self.send::<T>(client).await
    }

    /// Builds a post request and calls the sender
    async fn post<T: DeserializeOwned>(
        &self,
        url: Url,
        query: Option<Vec<(&str, String)>>,
        body: Option<Value>,
    ) -> T {
        let client = Client::new().post(url).query(&query);

        let client = match body {
            Some(b) => client.json(&b),
            None => client,
        };

        self.send::<T>(client).await
    }

    /// Sends any request
    async fn send<T: DeserializeOwned>(&self, client: RequestBuilder) -> T {
        let logger = Log { should_copy: false };
        let client = match &self.api_key {
            Some(a) => client.header("X-API-KEY", a),
            None => client,
        };

        let response = client.send().await;

        match response {
            Ok(res) => {
                if res.status().is_success() {
                    return match res.json::<T>().await {
                        Ok(parsed) => parsed,
                        Err(e) => {
                            println!("{:?}", e);
                            throw(Error::ServerResponseParseError);
                        }
                    };
                } else if res.status().as_str() == "401" {
                    throw(Error::AuthenticationFailed)
                }
                logger.error(&res.text().await.unwrap());
            }
            Err(e) => throw_from_http(e),
        }
    }
}
