use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use reqwest::{Client, Method};
use ring::hmac;
use serde::{Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha512};

use crate::gateio::options::{Result, rate_limit::RateLimiter};

const LIVE_URL: &str = "https://api.gateio.ws/api/v4";
const TESTNET_URL: &str = "https://api-testnet.gateapi.io/api/v4";

/// Private REST API client for Gate.io
#[derive(Clone)]
pub struct RestClient {
    client: Client,
    base_url: String,
    api_key: String,
    api_secret: String,
    rate_limiter: Arc<RateLimiter>,
}

impl RestClient {
    /// Create a new private REST client
    pub fn new(api_key: String, api_secret: String, testnet: bool) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(crate::gateio::options::GateIoError::Http)?;

        Ok(Self {
            client,
            base_url: if testnet { TESTNET_URL } else { LIVE_URL }.to_string(),
            api_key,
            api_secret,
            rate_limiter: Arc::new(RateLimiter::new()),
        })
    }

    /// Get the base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Generate signature for a request
    fn generate_signature(
        &self,
        method: &str,
        path: &str,
        query: &str,
        body: &str,
        timestamp: &str,
    ) -> String {
        // Calculate body hash
        let mut hasher = Sha512::new();
        hasher.update(body.as_bytes());
        let body_hash = hex::encode(hasher.finalize());

        // Build signature string
        let signature_string = format!(
            "{}\n{}\n{}\n{}\n{}",
            method, path, query, body_hash, timestamp
        );

        // Generate HMAC-SHA512 signature
        let key = hmac::Key::new(hmac::HMAC_SHA512, self.api_secret.as_bytes());
        let signature = hmac::sign(&key, signature_string.as_bytes());

        hex::encode(signature.as_ref())
    }

    /// Make a GET request to the API without query parameters
    pub async fn get<T>(&self, endpoint: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.request(Method::GET, endpoint, None::<&()>, None::<&()>)
            .await
    }

    /// Make a GET request to the API with query parameters
    pub async fn get_with_query<T, Q>(&self, endpoint: &str, query: &Q) -> Result<T>
    where
        T: DeserializeOwned,
        Q: Serialize,
    {
        self.request(Method::GET, endpoint, Some(query), None::<&()>)
            .await
    }

    /// Make a POST request to the API
    pub async fn post<T>(&self, endpoint: &str, body: &impl Serialize) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.request(Method::POST, endpoint, None::<&()>, Some(body))
            .await
    }

    /// Make a PUT request to the API
    pub async fn put<T>(&self, endpoint: &str, body: &impl Serialize) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.request(Method::PUT, endpoint, None::<&()>, Some(body))
            .await
    }

    /// Make a DELETE request to the API without query parameters
    pub async fn delete<T>(&self, endpoint: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.request(Method::DELETE, endpoint, None::<&()>, None::<&()>)
            .await
    }

    /// Make a DELETE request to the API with query parameters
    pub async fn delete_with_query<T, Q>(&self, endpoint: &str, query: &Q) -> Result<T>
    where
        T: DeserializeOwned,
        Q: Serialize,
    {
        self.request(Method::DELETE, endpoint, Some(query), None::<&()>)
            .await
    }

    /// Make a PATCH request to the API
    pub async fn patch<T>(&self, endpoint: &str, body: &impl Serialize) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.request(Method::PATCH, endpoint, None::<&()>, Some(body))
            .await
    }

    /// Make a request to the API with authentication
    async fn request<T>(
        &self,
        method: Method,
        endpoint: &str,
        query: Option<&impl Serialize>,
        body: Option<&impl Serialize>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        // Apply rate limiting
        let _permit = self.rate_limiter.get_permit(endpoint).await.map_err(|_| {
            crate::gateio::options::GateIoError::RateLimitExceeded {
                message: "Rate limit exceeded".to_string(),
            }
        })?;

        let url = format!("{}{}", self.base_url, endpoint);
        let method_str = method.as_str();

        // Generate timestamp
        #[allow(clippy::unwrap_used)]
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();

        // Build query string
        let query_string = if let Some(params) = query {
            serde_urlencoded::to_string(params)
                .map_err(|e| crate::gateio::options::GateIoError::Internal(e.to_string()))?
        } else {
            String::new()
        };

        // Get body string
        let body_str = if let Some(body_data) = body {
            serde_json::to_string(body_data).map_err(crate::gateio::options::GateIoError::Json)?
        } else {
            String::new()
        };

        // Generate signature
        let signature =
            self.generate_signature(method_str, endpoint, &query_string, &body_str, &timestamp);

        let mut request_builder = self
            .client
            .request(method, &url)
            .header("KEY", &self.api_key)
            .header("Timestamp", &timestamp)
            .header("SIGN", signature);

        // Add query parameters
        if !query_string.is_empty() {
            if let Some(params) = query {
                request_builder = request_builder.query(params);
            }
        }

        // Add body if present
        if !body_str.is_empty() {
            request_builder = request_builder
                .header("Content-Type", "application/json")
                .body(body_str);
        }

        let response = request_builder
            .send()
            .await
            .map_err(crate::gateio::options::GateIoError::Http)?;

        let status = response.status();
        let headers =
            crate::gateio::options::rate_limit::RateLimitHeader::from_headers(response.headers());

        // Update rate limiter with response headers
        if let Some(rate_status) = self.rate_limiter.update_from_headers(&headers, endpoint) {
            tracing::debug!("Rate limit status for {}: {:?}", endpoint, rate_status);
        }

        let response_text = response
            .text()
            .await
            .map_err(crate::gateio::options::GateIoError::Http)?;

        if status.is_success() {
            let data: T = serde_json::from_str(&response_text)
                .map_err(crate::gateio::options::GateIoError::Json)?;
            Ok(data)
        } else {
            let error: crate::gateio::options::errors::ErrorResponse =
                serde_json::from_str(&response_text)
                    .map_err(crate::gateio::options::GateIoError::Json)?;
            Err(crate::gateio::options::GateIoError::Api(
                crate::gateio::options::ApiError {
                    label: error.label,
                    message: error.message,
                    detail: error.detail,
                },
            ))
        }
    }
}
