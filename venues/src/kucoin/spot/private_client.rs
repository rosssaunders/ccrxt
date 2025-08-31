use std::{collections::HashMap, sync::Arc};

use base64::{Engine as _, engine::general_purpose};
use chrono::Utc;
use hmac::{Hmac, Mac};
use rest::{HttpClient, HttpError, Method, RequestBuilder, secrets::ExposableSecret};
use serde::{Serialize, de::DeserializeOwned};
use sha2::Sha256;

use super::{ApiError, ResponseHeaders, RestResponse, Result, rate_limit::RateLimiter};
use crate::kucoin::shared::credentials::Credentials;

#[derive(Clone)]
pub struct RestClient {
    pub base_url: String,
    pub http_client: Arc<dyn HttpClient>,
    pub rate_limiter: RateLimiter,
    pub credentials: Credentials,
    pub is_sandbox: bool,
}

impl RestClient {
    pub fn new(
        base_url: impl Into<String>,
        rate_limiter: RateLimiter,
        http_client: Arc<dyn HttpClient>,
        credentials: Credentials,
        is_sandbox: bool,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            http_client,
            rate_limiter,
            credentials,
            is_sandbox,
        }
    }
    pub fn new_with_credentials(
        credentials: Credentials,
        http_client: Arc<dyn HttpClient>,
    ) -> Self {
        Self::new(
            "https://api.kucoin.com",
            RateLimiter::new(),
            http_client,
            credentials,
            false,
        )
    }
    pub fn new_sandbox(credentials: Credentials, http_client: Arc<dyn HttpClient>) -> Self {
        Self::new(
            "https://openapi-sandbox.kucoin.com",
            RateLimiter::new(),
            http_client,
            credentials,
            true,
        )
    }

    pub fn create_auth_headers(
        &self,
        method: &str,
        endpoint: &str,
        body: &str,
        timestamp: i64,
    ) -> Result<HashMap<String, String>> {
        type HmacSha256 = Hmac<Sha256>;
        let prehash = format!("{}{}{}{}", timestamp, method.to_uppercase(), endpoint, body);
        let secret = self.credentials.api_secret.expose_secret();
        let passphrase = self.credentials.api_passphrase.expose_secret();
        let key = self.credentials.api_key.expose_secret();
        let mut mac =
            HmacSha256::new_from_slice(secret.as_bytes()).map_err(|e| ApiError::Other {
                code: "SIGNING".into(),
                message: format!("HMAC init failed: {e}"),
            })?;
        mac.update(prehash.as_bytes());
        let sig = general_purpose::STANDARD.encode(mac.finalize().into_bytes());
        let mut mac_pass =
            HmacSha256::new_from_slice(secret.as_bytes()).map_err(|e| ApiError::Other {
                code: "SIGNING".into(),
                message: format!("HMAC init failed: {e}"),
            })?;
        mac_pass.update(passphrase.as_bytes());
        let passphrase_signed = general_purpose::STANDARD.encode(mac_pass.finalize().into_bytes());
        let mut h = HashMap::new();
        h.insert("KC-API-KEY".into(), key);
        h.insert("KC-API-SIGN".into(), sig);
        h.insert("KC-API-TIMESTAMP".into(), timestamp.to_string());
        h.insert("KC-API-PASSPHRASE".into(), passphrase_signed);
        h.insert("KC-API-KEY-VERSION".into(), "2".into());
        Ok(h)
    }

    async fn execute<T>(&self, request: rest::Request) -> Result<(RestResponse<T>, ResponseHeaders)>
    where
        T: DeserializeOwned,
    {
        let resp = self
            .http_client
            .execute(request)
            .await
            .map_err(|e| match e {
                HttpError::Network(msg) => ApiError::Http(format!("Network error: {}", msg)),
                HttpError::Timeout => ApiError::Http("Request timeout".into()),
                HttpError::Http { status, body } => {
                    ApiError::Http(format!("HTTP {}: {}", status, body))
                }
                other => ApiError::Http(format!("HTTP error: {}", other)),
            })?;

        let status = resp.status;
        let text = resp
            .text()
            .map_err(|e| ApiError::Http(format!("Failed to decode response: {}", e)))?;

        if !(200..300).contains(&status) {
            if let Ok(err_resp) = serde_json::from_str::<super::ErrorResponse>(&text) {
                return Err(ApiError::from(err_resp).into());
            }
            return Err(ApiError::Http(format!("HTTP {}: {}", status, text)).into());
        }

        let parsed: RestResponse<T> = serde_json::from_str(&text)
            .map_err(|e| ApiError::JsonParsing(format!("Failed to parse response: {e}")))?;

        if !parsed.is_success() {
            return Err(ApiError::Other {
                code: parsed.code.clone(),
                message: "KuCoin API returned non-success code".into(),
            }
            .into());
        }

        let rl = ResponseHeaders::from_headers(&resp.headers);
        Ok((parsed, rl))
    }

    pub async fn get<T>(
        &self,
        endpoint: &str,
        params: Option<HashMap<String, String>>,
    ) -> Result<(RestResponse<T>, ResponseHeaders)>
    where
        T: DeserializeOwned,
    {
        if !self.rate_limiter.can_proceed().await {
            return Err(ApiError::RateLimitExceeded {
                code: "429000".into(),
                message: "Rate limit exceeded".into(),
            }
            .into());
        }

        let ts = Utc::now().timestamp_millis();
        let mut url = format!("{}{}", self.base_url, endpoint);
        let endpoint_sign = if let Some(map) = params.clone() {
            if !map.is_empty() {
                let mut v: Vec<(String, String)> = map.into_iter().collect();
                v.sort_by(|a, b| a.0.cmp(&b.0));
                let query_string = serde_urlencoded::to_string(&v).map_err(|e| {
                    ApiError::JsonParsing(format!("Failed to serialize query params: {}", e))
                })?;
                url = format!("{}?{}", url, query_string);
                format!("{}?{}", endpoint, query_string)
            } else {
                endpoint.to_string()
            }
        } else {
            endpoint.to_string()
        };

        let mut headers = HashMap::new();
        for (k, v) in self.create_auth_headers("GET", &endpoint_sign, "", ts)? {
            headers.insert(k, v);
        }

        let request = RequestBuilder::new(Method::Get, url)
            .headers(headers)
            .build();

        self.execute(request).await
    }

    pub async fn get_with_request<P, T>(
        &self,
        endpoint: &str,
        request: &P,
    ) -> Result<(RestResponse<T>, ResponseHeaders)>
    where
        P: Serialize,
        T: DeserializeOwned,
    {
        if !self.rate_limiter.can_proceed().await {
            return Err(ApiError::RateLimitExceeded {
                code: "429000".into(),
                message: "Rate limit exceeded".into(),
            }
            .into());
        }

        let ts = Utc::now().timestamp_millis();
        let mut url = format!("{}{}", self.base_url, endpoint);
        let params = serde_urlencoded::to_string(request)
            .map_err(|e| ApiError::JsonParsing(format!("Failed to serialize request: {e}")))?;
        let endpoint_sign = if !params.is_empty() {
            url = format!("{}?{}", url, params);
            format!("{}?{}", endpoint, params)
        } else {
            endpoint.to_string()
        };

        let mut headers = HashMap::new();
        for (k, v) in self.create_auth_headers("GET", &endpoint_sign, "", ts)? {
            headers.insert(k, v);
        }

        let request = RequestBuilder::new(Method::Get, url)
            .headers(headers)
            .build();

        self.execute(request).await
    }

    pub async fn post<T>(
        &self,
        endpoint: &str,
        body: &str,
    ) -> Result<(RestResponse<T>, ResponseHeaders)>
    where
        T: DeserializeOwned,
    {
        if !self.rate_limiter.can_proceed().await {
            return Err(ApiError::RateLimitExceeded {
                code: "429000".into(),
                message: "Rate limit exceeded".into(),
            }
            .into());
        }

        let ts = Utc::now().timestamp_millis();
        let url = format!("{}{}", self.base_url, endpoint);

        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        for (k, v) in self.create_auth_headers("POST", endpoint, body, ts)? {
            headers.insert(k, v);
        }

        let request = RequestBuilder::new(Method::Post, url)
            .headers(headers)
            .body(body.as_bytes().to_vec())
            .build();

        self.execute(request).await
    }

    pub async fn post_with_request<P, T>(
        &self,
        endpoint: &str,
        request: &P,
    ) -> Result<(RestResponse<T>, ResponseHeaders)>
    where
        P: Serialize,
        T: DeserializeOwned,
    {
        if !self.rate_limiter.can_proceed().await {
            return Err(ApiError::RateLimitExceeded {
                code: "429000".into(),
                message: "Rate limit exceeded".into(),
            }
            .into());
        }

        let ts = Utc::now().timestamp_millis();
        let body = serde_json::to_string(request)
            .map_err(|e| ApiError::JsonParsing(format!("Failed to serialize request: {e}")))?;
        let url = format!("{}{}", self.base_url, endpoint);

        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        for (k, v) in self.create_auth_headers("POST", endpoint, &body, ts)? {
            headers.insert(k, v);
        }

        let request = RequestBuilder::new(Method::Post, url)
            .headers(headers)
            .body(body.as_bytes().to_vec())
            .build();

        self.execute(request).await
    }

    pub async fn delete<T>(
        &self,
        endpoint: &str,
        params: Option<HashMap<String, String>>,
    ) -> Result<(RestResponse<T>, ResponseHeaders)>
    where
        T: DeserializeOwned,
    {
        if !self.rate_limiter.can_proceed().await {
            return Err(ApiError::RateLimitExceeded {
                code: "429000".into(),
                message: "Rate limit exceeded".into(),
            }
            .into());
        }

        let ts = Utc::now().timestamp_millis();
        let mut url = format!("{}{}", self.base_url, endpoint);
        let endpoint_sign = if let Some(map) = params.clone() {
            if !map.is_empty() {
                let mut v: Vec<(String, String)> = map.into_iter().collect();
                v.sort_by(|a, b| a.0.cmp(&b.0));
                let query_string = serde_urlencoded::to_string(&v).map_err(|e| {
                    ApiError::JsonParsing(format!("Failed to serialize query params: {}", e))
                })?;
                url = format!("{}?{}", url, query_string);
                format!("{}?{}", endpoint, query_string)
            } else {
                endpoint.to_string()
            }
        } else {
            endpoint.to_string()
        };

        let mut headers = HashMap::new();
        for (k, v) in self.create_auth_headers("DELETE", &endpoint_sign, "", ts)? {
            headers.insert(k, v);
        }

        let request = RequestBuilder::new(Method::Delete, url)
            .headers(headers)
            .build();

        self.execute(request).await
    }

    pub async fn delete_with_request<P, T>(
        &self,
        endpoint: &str,
        request: &P,
    ) -> Result<(RestResponse<T>, ResponseHeaders)>
    where
        P: Serialize,
        T: DeserializeOwned,
    {
        if !self.rate_limiter.can_proceed().await {
            return Err(ApiError::RateLimitExceeded {
                code: "429000".into(),
                message: "Rate limit exceeded".into(),
            }
            .into());
        }

        let ts = Utc::now().timestamp_millis();
        let mut url = format!("{}{}", self.base_url, endpoint);
        let params = serde_urlencoded::to_string(request)
            .map_err(|e| ApiError::JsonParsing(format!("Failed to serialize request: {e}")))?;
        let endpoint_sign = if !params.is_empty() {
            url = format!("{}?{}", url, params);
            format!("{}?{}", endpoint, params)
        } else {
            endpoint.to_string()
        };

        let mut headers = HashMap::new();
        for (k, v) in self.create_auth_headers("DELETE", &endpoint_sign, "", ts)? {
            headers.insert(k, v);
        }

        let request = RequestBuilder::new(Method::Delete, url)
            .headers(headers)
            .build();

        self.execute(request).await
    }
}

#[cfg(test)]
mod tests {
    use rest::{NativeHttpClient, secrets::SecretString};

    use super::*;
    fn creds() -> Credentials {
        Credentials {
            api_key: SecretString::new("test_key".into()),
            api_secret: SecretString::new("test_secret".into()),
            api_passphrase: SecretString::new("passphrase123".into()),
        }
    }

    fn mock_http_client() -> Arc<dyn HttpClient> {
        Arc::new(NativeHttpClient::new().unwrap())
    }

    #[test]
    fn construction() {
        let c = RestClient::new_with_credentials(creds(), mock_http_client());
        assert_eq!(c.base_url, "https://api.kucoin.com");
    }

    #[test]
    fn sandbox() {
        let c = RestClient::new_sandbox(creds(), mock_http_client());
        assert!(c.is_sandbox);
    }

    #[test]
    fn auth_headers_basic() {
        let c = RestClient::new_with_credentials(creds(), mock_http_client());
        let ts = 1234567890123i64;
        let h = c
            .create_auth_headers("GET", "/api/v1/accounts", "", ts)
            .unwrap();
        assert_eq!(h.get("KC-API-KEY").unwrap(), "test_key");
        assert!(h.contains_key("KC-API-SIGN"));
    }
}
