use std::sync::Arc;

use rest::{
    HttpClient,
    http_client::{Method as HttpMethod, RequestBuilder},
};
use serde::{Serialize, de::DeserializeOwned};

use crate::gateio::{GateIoError, GateIoRateLimiter, RestResult};

const LIVE_URL: &str = "https://api.gateio.ws/api/v4";
const TESTNET_URL: &str = "https://api-testnet.gateapi.io/api/v4";

/// Centralized public REST API client for Gate.io (root)
#[derive(Clone)]
pub struct PublicRestClient {
    http_client: Arc<dyn HttpClient>,
    base_url: String,
    rate_limiter: Arc<dyn GateIoRateLimiter>,
}

impl PublicRestClient {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        rate_limiter: Arc<dyn GateIoRateLimiter>,
        testnet: bool,
    ) -> RestResult<Self> {
        Ok(Self {
            http_client,
            base_url: if testnet { TESTNET_URL } else { LIVE_URL }.to_string(),
            rate_limiter,
        })
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub async fn get<T>(&self, endpoint: &str) -> RestResult<T>
    where
        T: DeserializeOwned,
    {
        self.request(HttpMethod::Get, endpoint, None::<&()>).await
    }

    // Accept an optional query parameter to match existing wrapper call-sites that pass
    // `Some(&params)`.
    pub async fn get_with_query<T, Q>(&self, endpoint: &str, query: Option<&Q>) -> RestResult<T>
    where
        T: DeserializeOwned,
        Q: Serialize,
    {
        match query {
            Some(q) => self.request(HttpMethod::Get, endpoint, Some(q)).await,
            None => self.request(HttpMethod::Get, endpoint, None::<&()>).await,
        }
    }

    // Adapter helpers for existing wrappers
    pub async fn send_get_request<T, Q>(&self, endpoint: &str, query: Option<&Q>) -> RestResult<T>
    where
        T: DeserializeOwned,
        Q: Serialize,
    {
        match query {
            Some(q) => self.request(HttpMethod::Get, endpoint, Some(q)).await,
            None => self.request(HttpMethod::Get, endpoint, None::<&()>).await,
        }
    }

    pub async fn send_get_with_query<T, Q>(&self, endpoint: &str, query: &Q) -> RestResult<T>
    where
        T: DeserializeOwned,
        Q: Serialize,
    {
        self.request(HttpMethod::Get, endpoint, Some(query)).await
    }

    async fn request<T>(
        &self,
        method: HttpMethod,
        endpoint: &str,
        query: Option<&impl Serialize>,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
    {
        self.rate_limiter.get_permit(endpoint).await?;

        let url = format!("{}{}", self.base_url, endpoint);
        let query_string = if let Some(params) = query {
            serde_urlencoded::to_string(params).map_err(|e| GateIoError::Internal(e.to_string()))?
        } else {
            String::new()
        };

        let full_url = if !query_string.is_empty() {
            format!("{}?{}", url, query_string)
        } else {
            url
        };

        let request = RequestBuilder::new(method, full_url).build();
        let response = self
            .http_client
            .execute(request)
            .await
            .map_err(|e| GateIoError::Network(format!("HTTP request failed: {}", e)))?;
        let status = response.status;
        let response_text = response
            .text()
            .map_err(|e| GateIoError::Network(format!("Failed to read response: {}", e)))?;

        if (200..300).contains(&status) {
            let data: T = serde_json::from_str(&response_text).map_err(GateIoError::Json)?;
            Ok(data)
        } else {
            let err = serde_json::from_str::<crate::gateio::ErrorResponse>(&response_text)
                .map_err(GateIoError::Json)?;
            Err(GateIoError::Api(err.into()))
        }
    }
}
