use crate::agent::CloudAgentAfjRest;
use reqwest::{Client, RequestBuilder, Url};
use serde::de::DeserializeOwned;
use serde_json::Value;
use siera_agent::error::{Error, Result};

/// Call logic for http calls
impl CloudAgentAfjRest {
    /// Builds a get request and calls the sender
    ///
    /// # Errors
    ///
    /// When it could not fulfill a GET request
    pub async fn get<T: DeserializeOwned>(
        &self,
        url: Url,
        query: Option<Vec<(&str, String)>>,
    ) -> Result<T> {
        let client = match &query {
            Some(q) => Client::new().get(url).query(&q),
            None => Client::new().get(url),
        };

        trace!({ "message": "Get request query", "query": query });

        self.send::<T>(client).await
    }

    /// Builds a post request and calls the sender
    ///
    /// # Errors
    ///
    /// When it could not fulfill a POST request
    pub async fn post<T: DeserializeOwned>(
        &self,
        url: Url,
        query: Option<Vec<(&str, String)>>,
        body: Option<Value>,
    ) -> Result<T> {
        let client = Client::new().post(url).query(&query);

        let client = match body {
            Some(ref b) => client.json(b),
            None => client,
        };

        trace!({ "message": "Post request body", "body": body, "query": query });

        self.send::<T>(client).await
    }

    /// Sends any request
    ///
    /// # Errors
    ///
    /// When it could not fulfill the given request
    pub async fn send<T: DeserializeOwned>(&self, client: RequestBuilder) -> Result<T> {
        trace!({ "message": "About to send request" });

        match client.send().await {
            Ok(res) => {
                let status_code = res.status().as_u16();
                debug!({ "status_code": status_code });
                match status_code {
                    200..=299 => res.json().await.map_err(|e| {
                        warn!({"error": e.to_string() });
                        Error::UnableToParseResponse.into()
                    }),
                    // Issue credential message when attributes are not correct
                    400 => Err(res.text().await?.into()),
                    401 => Err(Error::AuthorizationFailed.into()),
                    404 => Err(Error::UrlDoesNotExist.into()),
                    500..=599 => Err(Error::InternalServerError(
                        res.status().as_u16(),
                        res.text().await?,
                    )
                    .into()),
                    _ => Err(Error::UnknownResponseStatusCode(res.text().await?).into()),
                }
            }
            Err(e) => {
                warn!({ "message": "request failed", "error": e.to_string()});
                Err(Error::UnreachableUrl.into())
            }
        }
    }
}
