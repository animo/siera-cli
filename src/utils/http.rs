use std::collections::HashMap;

use reqwest::{Client, Url};
use serde::de::DeserializeOwned;

use crate::error::{throw, Error};

// Handle calling of any endpoint with get
pub async fn get<T: DeserializeOwned>(url: Url, query: Option<Vec<(&str, &str)>>) -> T {
    let client = match query {
        Some(q) => Client::new().get(url).query(&q),
        None => Client::new().get(url),
    };

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

// Handle calling of any endpoint with post
pub async fn post<T: DeserializeOwned>(
    url: Url,
    query: Vec<(&str, String)>,
    body: Option<HashMap<&str, HashMap<&str, &str>>>,
) -> T {
    let client = Client::new().post(url).query(&query);

    let response = match body {
        Some(b) => client.json(&b).send().await,
        None => client.send().await,
    };

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
