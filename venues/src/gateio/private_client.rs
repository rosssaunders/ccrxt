use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use hmac::{Hmac, Mac};
use serde::{Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha512};

type HmacSha512 = Hmac<Sha512>;

use crate::gateio::{Credentials, ErrorResponse, GateIoError, GateIoRateLimiter, RestResult};
use rest::secrets::ExposableSecret;
use rest::{
    HttpClient,
    http_client::{Method as HttpMethod, RequestBuilder},
};

const LIVE_URL: &str = "https://api.gateio.ws/api/v4";
const TESTNET_URL: &str = "https://api-testnet.gateapi.io/api/v4";

/// Centralized private REST API client for Gate.io (root)
#[derive(Clone)]
pub struct PrivateRestClient {
    http_client: Arc<dyn HttpClient>,
    base_url: String,
    credentials: Credentials,
    rate_limiter: Arc<dyn GateIoRateLimiter>,
}

impl PrivateRestClient {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        credentials: Credentials,
        rate_limiter: Arc<dyn GateIoRateLimiter>,
        testnet: bool,
    ) -> RestResult<Self> {
        Ok(Self {
            http_client,
            base_url: if testnet { TESTNET_URL } else { LIVE_URL }.to_string(),
            credentials,
            rate_limiter,
        })
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    fn generate_signature(
        &self,
        method: &str,
        path: &str,
        query: &str,
        body: &str,
        timestamp: &str,
    ) -> RestResult<String> {
        let mut hasher = Sha512::new();
        hasher.update(body.as_bytes());
        let body_hash = hex::encode(hasher.finalize());

        let signature_string = format!(
            "{}\n{}\n{}\n{}\n{}",
            method, path, query, body_hash, timestamp
        );

        let mut mac =
            HmacSha512::new_from_slice(self.credentials.api_secret.expose_secret().as_bytes())
                .map_err(|e| GateIoError::Internal(format!("failed to create HMAC: {}", e)))?;
        mac.update(signature_string.as_bytes());
        let signature = mac.finalize();

        Ok(hex::encode(signature.into_bytes()))
    }

    pub async fn get<T>(&self, endpoint: &str) -> RestResult<T>
    where
        T: DeserializeOwned,
    {
        self.request(HttpMethod::Get, endpoint, None::<&()>, None::<&()>)
            .await
    }

    // Adapter methods kept for backward compatibility with existing endpoint wrappers
    pub async fn send_get_request<T, Q>(&self, endpoint: &str, query: Option<&Q>) -> RestResult<T>
    where
        T: DeserializeOwned,
        Q: Serialize,
    {
        self.request(HttpMethod::Get, endpoint, query, None::<&()>)
            .await
    }

    pub async fn send_post_request<T, B>(&self, endpoint: &str, body: Option<&B>) -> RestResult<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        match body {
            Some(b) => {
                self.request(HttpMethod::Post, endpoint, None::<&()>, Some(b))
                    .await
            }
            None => {
                self.request(HttpMethod::Post, endpoint, None::<&()>, None::<&()>)
                    .await
            }
        }
    }

    pub async fn send_put_request<T, B>(&self, endpoint: &str, body: Option<&B>) -> RestResult<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        match body {
            Some(b) => {
                self.request(HttpMethod::Put, endpoint, None::<&()>, Some(b))
                    .await
            }
            None => {
                self.request(HttpMethod::Put, endpoint, None::<&()>, None::<&()>)
                    .await
            }
        }
    }

    pub async fn send_delete_request<T, Q>(
        &self,
        endpoint: &str,
        query: Option<&Q>,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        Q: Serialize,
    {
        self.request(HttpMethod::Delete, endpoint, query, None::<&()>)
            .await
    }

    pub async fn send_patch_request<T, B>(&self, endpoint: &str, body: Option<&B>) -> RestResult<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        match body {
            Some(b) => {
                self.request(HttpMethod::Patch, endpoint, None::<&()>, Some(b))
                    .await
            }
            None => {
                self.request(HttpMethod::Patch, endpoint, None::<&()>, None::<&()>)
                    .await
            }
        }
    }

    pub async fn get_with_query<T, Q>(&self, endpoint: &str, query: &Q) -> RestResult<T>
    where
        T: DeserializeOwned,
        Q: Serialize,
    {
        self.request(HttpMethod::Get, endpoint, Some(query), None::<&()>)
            .await
    }

    pub async fn post<T>(&self, endpoint: &str, body: &impl Serialize) -> RestResult<T>
    where
        T: DeserializeOwned,
    {
        self.request(HttpMethod::Post, endpoint, None::<&()>, Some(body))
            .await
    }

    pub async fn put<T>(&self, endpoint: &str, body: &impl Serialize) -> RestResult<T>
    where
        T: DeserializeOwned,
    {
        self.request(HttpMethod::Put, endpoint, None::<&()>, Some(body))
            .await
    }

    pub async fn delete<T>(&self, endpoint: &str) -> RestResult<T>
    where
        T: DeserializeOwned,
    {
        self.request(HttpMethod::Delete, endpoint, None::<&()>, None::<&()>)
            .await
    }

    pub async fn delete_with_query<T, Q>(&self, endpoint: &str, query: &Q) -> RestResult<T>
    where
        T: DeserializeOwned,
        Q: Serialize,
    {
        self.request(HttpMethod::Delete, endpoint, Some(query), None::<&()>)
            .await
    }

    pub async fn patch<T>(&self, endpoint: &str, body: &impl Serialize) -> RestResult<T>
    where
        T: DeserializeOwned,
    {
        self.request(HttpMethod::Patch, endpoint, None::<&()>, Some(body))
            .await
    }

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
        self.rate_limiter.get_permit(endpoint).await?;

        let url = format!("{}{}", self.base_url, endpoint);
        let method_str = match method {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Patch => "PATCH",
            _ => "GET",
        };

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| GateIoError::Internal(e.to_string()))?
            .as_secs()
            .to_string();

        let query_string = if let Some(params) = query {
            serde_urlencoded::to_string(params).map_err(|e| GateIoError::Internal(e.to_string()))?
        } else {
            String::new()
        };

        let body_str = if let Some(body_data) = body {
            serde_json::to_string(body_data).map_err(GateIoError::Json)?
        } else {
            String::new()
        };

        let signature =
            self.generate_signature(method_str, endpoint, &query_string, &body_str, &timestamp)?;

        let full_url = if !query_string.is_empty() {
            format!("{}?{}", url, query_string)
        } else {
            url
        };

        let mut request = RequestBuilder::new(method, full_url)
            .header("KEY", self.credentials.api_key.expose_secret())
            .header("Timestamp", &timestamp)
            .header("SIGN", signature);

        if !body_str.is_empty() {
            request = request
                .header("Content-Type", "application/json")
                .body(body_str.into_bytes());
        }

        let response = self
            .http_client
            .execute(request.build())
            .await
            .map_err(|e| GateIoError::Network(format!("HTTP request failed: {}", e)))?;
        let status = response.status;
        self.rate_limiter.update_from_headers(&response.headers, endpoint).await;

        let response_text = response
            .text()
            .map_err(|e| GateIoError::Network(format!("Failed to read response: {}", e)))?;

        if (200..300).contains(&status) {
            let data: T = serde_json::from_str(&response_text).map_err(GateIoError::Json)?;
            Ok(data)
        } else {
            let error: ErrorResponse =
                serde_json::from_str(&response_text).map_err(GateIoError::Json)?;
            Err(GateIoError::Api(error.into()))
        }
    }
}
