use reqwest::{Client, Url};

// Handle calling of any endpoint with get
//pub async fn call_get_endpoint(
//url: Url,
//query: Option<Vec<(&str, &str)>>,
//) -> Result<reqwest::Response, reqwest::Error> {
//let client = match query {
//Some(q) => Client::new().get(url).query(&q),
//None => Client::new().get(url),
//};
//client.send().await
//}

// Handle calling of any endpoint with post
pub async fn call_post_endpoint(
    url: Url,
    query: Vec<(&str, String)>,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = Client::new().post(url);
    client.query(&query).send().await
}
