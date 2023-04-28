use crate::agent::CloudAgentPython;
use reqwest::{Client, RequestBuilder, Url};
use serde::de::DeserializeOwned;
use serde_json::Value;
use siera_agent::error::{Error, Result};
use std::fmt::Debug;

/// Call logic for http calls
impl CloudAgentPython {
    /// Builds a get request and calls the sender
    ///
    /// # Errors
    ///
    /// When it could not fulfill a GET request
    pub async fn get<T: DeserializeOwned + Debug>(
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

    /// Builds a patch request and calls the sender
    ///
    /// # Errors
    ///
    /// When it could not fulfill a PATCH request
    pub async fn patch<T: DeserializeOwned + Debug>(
        &self,
        url: Url,
        query: Option<Vec<(&str, String)>>,
    ) -> Result<T> {
        let client = match &query {
            Some(q) => Client::new().patch(url).query(&q),
            None => Client::new().patch(url),
        };

        trace!({ "message": "Patch request query", "query": query});

        self.send::<T>(client).await
    }

    /// Builds a post request and calls the sender
    ///
    /// # Errors
    ///
    /// When it could not fulfill a POST request
    pub async fn post<T: DeserializeOwned + Debug>(
        &self,
        url: Url,
        query: Option<Vec<(&str, String)>>,
        body: Option<Value>,
    ) -> Result<T> {
        let client = Client::new().post(url).query(&query);

        let client = match &body {
            Some(b) => client.json(&b),
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
    pub async fn send<T: DeserializeOwned + Debug>(&self, client: RequestBuilder) -> Result<T> {
        let client = match &self.api_key {
            Some(a) => client.header("X-API-KEY", a),
            None => client,
        };

        let client = match &self.auth_token {
            Some(token) => client.header("Authorization", format!("Bearer {token}")),
            None => client,
        };

        trace!({ "message": "About to send request" });
        match client.send().await {
            Ok(response) => {
                let status_code = response.status().as_u16();
                debug!({ "status_code": status_code });
                match status_code {
                    200..=299 => {
                        let parsed: Value = response.json().await?;
                        debug!({ "response": parsed });
                        let parsed = if parsed.is_null() {
                            serde_json::json!(())
                        } else {
                            parsed
                        };
                        serde_json::from_value(parsed).map_err(|e| {
                            warn!({"error": e.to_string() });
                            Error::UnableToParseResponse.into()
                        })
                    }
                    // Issue credential message when attributes are not correct
                    400 => Err(response.text().await?.into()),
                    401 => Err(Error::AuthorizationFailed.into()),
                    404 => Err(Error::UrlDoesNotExist.into()),
                    422 => Err(response.text().await?.into()),
                    503 => Err(Error::HttpServiceUnavailable.into()),
                    500..=599 => Err(Error::InternalServerError(
                        response.status().as_u16(),
                        response.text().await?,
                    )
                    .into()),
                    _ => Err(Error::UnknownResponseStatusCode(response.text().await?).into()),
                }
            }
            Err(e) => {
                warn!({ "message": "request failed", "error": e.to_string() });
                Err(Error::UnreachableUrl.into())
            }
        }
    }
}
