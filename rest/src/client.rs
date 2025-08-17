use std::sync::Arc;

use serde::{Serialize, de::DeserializeOwned};

#[cfg(feature = "native")]
use crate::native::NativeHttpClient;
use crate::{
    error::RestError,
    http_client::{HttpClient, HttpError, Method, RequestBuilder},
};

/// A generic REST client for making HTTP requests
#[derive(Clone)]
pub struct Client {
    base_url: String,
    http_client: Arc<dyn HttpClient>,
}

impl Client {
    /// Create a new REST client with a custom HTTP client
    pub fn with_http_client(
        base_url: impl Into<String>,
        http_client: impl HttpClient + 'static,
        _api_key: Option<String>,
        _secret: Option<String>,
        _passphrase: Option<String>,
    ) -> Result<Self, RestError> {
        Ok(Self {
            base_url: base_url.into(),
            http_client: Arc::new(http_client),
        })
    }

    /// Create a new REST client with the default HTTP client for the platform
    #[cfg(feature = "native")]
    pub fn new(
        base_url: impl Into<String>,
        _api_key: Option<String>,
        _secret: Option<String>,
        _passphrase: Option<String>,
    ) -> Result<Self, RestError> {
        let http_client = NativeHttpClient::new()
            .map_err(|e| RestError::Unknown(format!("Failed to create HTTP client: {}", e)))?;

        Self::with_http_client(base_url, http_client, _api_key, _secret, _passphrase)
    }

    /// Make a GET request
    pub async fn get<T, P>(&self, endpoint: &str, params: Option<&P>) -> Result<T, RestError>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        let mut builder = RequestBuilder::new(Method::Get, url);

        if let Some(params) = params {
            builder = builder.query(params).map_err(RestError::from)?;
        }

        let request = builder.build();
        let response = self
            .http_client
            .execute(request)
            .await
            .map_err(RestError::from)?;

        if !response.is_success() {
            return Err(RestError::HttpError(format!(
                "HTTP {}: {}",
                response.status,
                response.text().unwrap_or_default()
            )));
        }

        let result = response
            .json()
            .map_err(|e| RestError::Unknown(format!("Failed to parse response: {}", e)))?;
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
        let mut builder = RequestBuilder::new(Method::Post, url);

        if let Some(params) = params {
            builder = builder.json(params).map_err(RestError::from)?;
        }

        let request = builder.build();
        let response = self
            .http_client
            .execute(request)
            .await
            .map_err(RestError::from)?;

        if !response.is_success() {
            return Err(RestError::HttpError(format!(
                "HTTP {}: {}",
                response.status,
                response.text().unwrap_or_default()
            )));
        }

        let result = response
            .json()
            .map_err(|e| RestError::Unknown(format!("Failed to parse response: {}", e)))?;
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

impl From<HttpError> for RestError {
    fn from(err: HttpError) -> Self {
        match err {
            HttpError::Network(msg) => RestError::HttpError(msg),
            HttpError::Timeout => RestError::HttpError("Request timeout".to_string()),
            HttpError::InvalidUrl(msg) => RestError::ValidationError(msg),
            HttpError::Decode(msg) => RestError::Unknown(msg),
            HttpError::Http { status, body } => {
                RestError::HttpError(format!("HTTP {}: {}", status, body))
            }
            HttpError::Unknown(msg) => RestError::Unknown(msg),
        }
    }
}
