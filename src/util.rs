use super::{error, typing};
use reqwest::{Client, Url};

// Handle calling of any endpoint with get
pub async fn call_get_endpoint(
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
pub async fn call_post_endpoint(
    url: Url,
    query: Option<Vec<(&str, Option<&str>)>>,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = Client::new().post(url);
    match query {
        Some(q) => {
            let qw = q
                .into_iter()
                .filter(|param| param.1.is_some())
                .map(|param| (param.0, param.1.unwrap()))
                .collect::<Vec<(&str, &str)>>();
            client.query(&qw).send().await
        }
        None => client.send().await,
    }
}

// Parse `Invitation to a `qr_code`
pub fn print_invitation_and_qr_for_invitation(invitation: typing::Invitation) {
    // Can use unwrap here because the url is supplied by the endpoint
    let url = reqwest::Url::parse(&invitation.invitation_url.to_string()).unwrap();

    if let Err(_) = qr2term::print_qr(&url.as_ref()) {
        error::throw(error::Error::CannotCreateQrCode)
    }
}
