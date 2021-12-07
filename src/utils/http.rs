use std::collections::HashMap;

use reqwest::{Client, Url};

// Handle calling of any endpoint with get
pub async fn get(
    url: Url,
    query: Option<Vec<(&str, &str)>>,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = match query {
        Some(q) => Client::new().get(url).query(&q),
        None => Client::new().get(url),
    };
    client.send().await
}

// Handle calling of any endpoint with post
pub async fn post(
    url: Url,
    query: Vec<(&str, String)>,
    body: Option<HashMap<&str, HashMap<&str, &str>>>,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = Client::new().post(url);
    client.query(&query).json(&body).send().await
}
