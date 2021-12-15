use reqwest::{Client, RequestBuilder, Url};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::error::{throw, Error};

/// Builds a get request and calls the sender
pub async fn get<T: DeserializeOwned>(url: Url, query: Option<Vec<(&str, String)>>) -> T {
    let client = match query {
        Some(q) => Client::new().get(url).query(&q),
        None => Client::new().get(url),
    };

    send::<T>(client).await
}

/// Builds a post request and calls the sender
pub async fn post<T: DeserializeOwned>(
    url: Url,
    query: Option<Vec<(&str, String)>>,
    body: Option<Value>,
) -> T {
    let client = Client::new().post(url).query(&query);

    let client = match body {
        Some(b) => client.json(&b),
        None => client,
    };

    send::<T>(client).await
}

/// Sends any request
async fn send<T: DeserializeOwned>(client: RequestBuilder) -> T {
    let response = client.send().await;

    match response {
        Ok(res) => {
            if res.status().is_success() {
                return match res.json().await {
                    Ok(parsed) => parsed,
                    Err(_) => throw(Error::ServerResponseParseError),
                };
            }
            throw(Error::InternalServerError)
        }
        Err(_) => throw(Error::InternalServerError),
    }
}
