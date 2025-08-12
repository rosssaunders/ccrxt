use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use rest::{HttpClient, http_client::{Method as HttpMethod, RequestBuilder}};
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha512};
use serde::{Serialize, de::DeserializeOwned};

type HmacSha512 = Hmac<Sha512>;

use crate::gateio::delivery::{RestResult, rate_limit::RateLimiter};
use crate::gateio::shared::credentials::Credentials;
use rest::secrets::ExposableSecret;

const LIVE_URL: &str = "https://api.gateio.ws/api/v4";
const TESTNET_URL: &str = "https://api-testnet.gateapi.io/api/v4";

/// Private REST API client for Gate.io
#[derive(Clone)]
pub struct RestClient {
    http_client: Arc<dyn HttpClient>,
    base_url: String,
    credentials: Credentials,
    rate_limiter: Arc<RateLimiter>,
}

impl RestClient {
    /// Create a new private REST client
    pub fn new(http_client: Arc<dyn HttpClient>, credentials: Credentials, testnet: bool) -> RestResult<Self> {
        Ok(Self {
            http_client,
            base_url: if testnet { TESTNET_URL } else { LIVE_URL }.to_string(),
            credentials,
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
        let mut mac = HmacSha512::new_from_slice(self.credentials.api_secret.expose_secret().as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(signature_string.as_bytes());
        let signature = mac.finalize();

        hex::encode(signature.into_bytes())
    }

    /// Make a GET request to the API without query parameters
    pub async fn get<T>(&self, endpoint: &str) -> RestResult<T>
    where
        T: DeserializeOwned,
    {
        self.request(HttpMethod::Get, endpoint, None::<&()>, None::<&()>)
            .await
    }

    /// Make a GET request to the API with query parameters
    pub async fn get_with_query<T, Q>(&self, endpoint: &str, query: &Q) -> RestResult<T>
    where
        T: DeserializeOwned,
        Q: Serialize,
    {
        self.request(HttpMethod::Get, endpoint, Some(query), None::<&()>)
            .await
    }

    /// Make a POST request to the API
    pub async fn post<T>(&self, endpoint: &str, body: &impl Serialize) -> RestResult<T>
    where
        T: DeserializeOwned,
    {
        self.request(HttpMethod::Post, endpoint, None::<&()>, Some(body))
            .await
    }

    /// Make a PUT request to the API
    pub async fn put<T>(&self, endpoint: &str, body: &impl Serialize) -> RestResult<T>
    where
        T: DeserializeOwned,
    {
        self.request(HttpMethod::Put, endpoint, None::<&()>, Some(body))
            .await
    }

    /// Make a DELETE request to the API without query parameters
    pub async fn delete<T>(&self, endpoint: &str) -> RestResult<T>
    where
        T: DeserializeOwned,
    {
        self.request(HttpMethod::Delete, endpoint, None::<&()>, None::<&()>)
            .await
    }

    /// Make a DELETE request to the API with query parameters
    pub async fn delete_with_query<T, Q>(&self, endpoint: &str, query: &Q) -> RestResult<T>
    where
        T: DeserializeOwned,
        Q: Serialize,
    {
        self.request(HttpMethod::Delete, endpoint, Some(query), None::<&()>)
            .await
    }

    /// Make a PATCH request to the API
    pub async fn patch<T>(&self, endpoint: &str, body: &impl Serialize) -> RestResult<T>
    where
        T: DeserializeOwned,
    {
        self.request(HttpMethod::Patch, endpoint, None::<&()>, Some(body))
            .await
    }

    /// Make a request to the API with authentication
    async fn request<T>(
        &self,
        method: HttpMethod,
        endpoint: &str,
        query: Option<&impl Serialize>,
        body: Option<&impl Serialize>,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
    {
        // Apply rate limiting
        let _permit = self.rate_limiter.get_permit(endpoint).await.map_err(|_| {
            crate::gateio::delivery::GateIoError::RateLimitExceeded {
                message: "Rate limit exceeded".to_string(),
            }
        })?;

        let url = format!("{}{}", self.base_url, endpoint);
        let method_str = match method {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Patch => "PATCH",
            _ => "GET",
        };

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
                .map_err(|e| crate::gateio::delivery::GateIoError::Internal(e.to_string()))?
        } else {
            String::new()
        };

        // Get body string
        let body_str = if let Some(body_data) = body {
            serde_json::to_string(body_data).map_err(crate::gateio::delivery::GateIoError::Json)?
        } else {
            String::new()
        };

        // Generate signature
        let signature =
            self.generate_signature(method_str, endpoint, &query_string, &body_str, &timestamp);

        // Build URL with query parameters
        let full_url = if !query_string.is_empty() {
            format!("{}?{}", url, query_string)
        } else {
            url
        };

        // Build request
        let mut request = RequestBuilder::new(method, full_url)
            .header("KEY", self.credentials.api_key.expose_secret())
            .header("Timestamp", &timestamp)
            .header("SIGN", signature);

        // Add body if present
        if !body_str.is_empty() {
            request = request
                .header("Content-Type", "application/json")
                .body(body_str.into_bytes());
        }

        let response = self.http_client
            .execute(request.build())
            .await
            .map_err(|e| crate::gateio::delivery::GateIoError::Network(format!("HTTP request failed: {}", e)))?;

        let status = response.status;
        let headers =
            crate::gateio::delivery::rate_limit::RateLimitHeader::from_headers(&response.headers);

        // Update rate limiter with response headers
        if let Some(rate_status) = self.rate_limiter.update_from_headers(&headers, endpoint) {
            tracing::debug!("Rate limit status for {}: {:?}", endpoint, rate_status);
        }

        let response_text = response
            .text()
            .map_err(|e| crate::gateio::delivery::GateIoError::Network(format!("Failed to read response: {}", e)))?;

        if status >= 200 && status < 300 {
            let data: T = serde_json::from_str(&response_text)
                .map_err(crate::gateio::delivery::GateIoError::Json)?;
            Ok(data)
        } else {
            let error: crate::gateio::delivery::errors::ErrorResponse =
                serde_json::from_str(&response_text)
                    .map_err(crate::gateio::delivery::GateIoError::Json)?;
            Err(crate::gateio::delivery::GateIoError::Api(
                crate::gateio::delivery::ApiError {
                    label: error.label,
                    message: error.message,
                    detail: error.detail,
                },
            ))
        }
    }
}
