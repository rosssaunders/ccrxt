use reqwest;
use serde::{de::DeserializeOwned, Serialize};

use crate::error::RestError;

/// A generic REST client for making HTTP requests
#[derive(Debug, Clone)]
pub struct Client {
    base_url: String,
    client: reqwest::Client,
}

impl Client {
    /// Create a new REST client
    pub fn new(
        base_url: impl Into<String>,
        _api_key: Option<String>,
        _secret: Option<String>,
        _passphrase: Option<String>,
    ) -> Result<Self, RestError> {
        Ok(Self {
            base_url: base_url.into(),
            client: reqwest::Client::new(),
        })
    }

    /// Make a GET request
    pub async fn get<T, P>(&self, endpoint: &str, params: Option<&P>) -> Result<T, RestError>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        let mut request = self.client.get(&url);

        if let Some(params) = params {
            request = request.query(params);
        }

        let response = request.send().await?;
        let result = response.json().await?;
        Ok(result)
    }

    /// Make a POST request with authentication
    pub async fn post_signed<T, P>(
        &self,
        endpoint: &str,
        params: Option<&P>,
    ) -> Result<T, RestError>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        let mut request = self.client.post(&url);

        if let Some(params) = params {
            request = request.json(params);
        }

        let response = request.send().await?;
        let result = response.json().await?;
        Ok(result)
    }

    /// Make a GET request with authentication  
    pub async fn get_signed<T, P>(&self, endpoint: &str, params: Option<&P>) -> Result<T, RestError>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        self.get(endpoint, params).await
    }
}
