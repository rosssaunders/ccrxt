//! KuCoin private REST API client
#![allow(clippy::needless_borrows_for_generic_args)]
#![allow(clippy::unwrap_used)]

use std::collections::HashMap;

use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use chrono::Utc;
use hmac::{Hmac, Mac};
use reqwest::Client;
use rest::secrets::ExposableSecret;
use serde::de::DeserializeOwned;
use sha2::Sha256;

use crate::kucoin::{ApiError, RateLimiter, ResponseHeaders, RestResponse, Result};

/// Private REST client for KuCoin spot market
pub struct RestClient {
    pub base_url: String,
    pub client: Client,
    pub rate_limiter: RateLimiter,
    api_key: Box<dyn ExposableSecret>,
    api_secret: Box<dyn ExposableSecret>,
    api_passphrase: Box<dyn ExposableSecret>,
    #[allow(dead_code)]
    is_sandbox: bool,
}

impl RestClient {
    /// Create a new private REST client
    pub fn new(
        base_url: impl Into<String>,
        rate_limiter: RateLimiter,
        client: Client,
        api_key: impl Into<Box<dyn ExposableSecret>>,
        api_secret: impl Into<Box<dyn ExposableSecret>>,
        api_passphrase: impl Into<Box<dyn ExposableSecret>>,
        is_sandbox: bool,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            client,
            rate_limiter,
            api_key: api_key.into(),
            api_secret: api_secret.into(),
            api_passphrase: api_passphrase.into(),
            is_sandbox,
        }
    }

    /// Create a new private REST client with default production settings
    pub fn new_with_credentials(
        api_key: impl Into<Box<dyn ExposableSecret>>,
        api_secret: impl Into<Box<dyn ExposableSecret>>,
        api_passphrase: impl Into<Box<dyn ExposableSecret>>,
    ) -> Self {
        Self::new(
            "https://api.kucoin.com",
            RateLimiter::new(),
            Client::new(),
            api_key,
            api_secret,
            api_passphrase,
            false,
        )
    }

    /// Create a new private REST client for sandbox environment
    pub fn new_sandbox(
        api_key: impl Into<Box<dyn ExposableSecret>>,
        api_secret: impl Into<Box<dyn ExposableSecret>>,
        api_passphrase: impl Into<Box<dyn ExposableSecret>>,
    ) -> Self {
        Self::new(
            "https://openapi-sandbox.kucoin.com",
            RateLimiter::new(),
            Client::new(),
            api_key,
            api_secret,
            api_passphrase,
            true,
        )
    }

    /// Generate authentication headers for KuCoin API
    fn create_auth_headers(
        &self,
        method: &str,
        endpoint: &str,
        body: &str,
        timestamp: i64,
    ) -> Result<HashMap<String, String>> {
        let api_key = self.api_key.expose_secret();
        let api_secret = self.api_secret.expose_secret();
        let api_passphrase = self.api_passphrase.expose_secret();

        // Create the string to sign: timestamp + method + endpoint + body
        let str_to_sign = format!("{}{}{}{}", timestamp, method, endpoint, body);

        // Create HMAC-SHA256 signature
        let mut mac =
            Hmac::<Sha256>::new_from_slice(api_secret.as_bytes()).map_err(|e| ApiError::Other {
                code: "AUTH_ERROR".to_string(),
                message: format!("Failed to create HMAC: {}", e),
            })?;

        mac.update(str_to_sign.as_bytes());
        let signature = BASE64.encode(&mac.finalize().into_bytes());

        // Create passphrase signature for KC-API-PASSPHRASE header
        let mut passphrase_mac =
            Hmac::<Sha256>::new_from_slice(api_secret.as_bytes()).map_err(|e| ApiError::Other {
                code: "AUTH_ERROR".to_string(),
                message: format!("Failed to create passphrase HMAC: {}", e),
            })?;

        passphrase_mac.update(api_passphrase.as_bytes());
        let passphrase_signature = BASE64.encode(&passphrase_mac.finalize().into_bytes());

        let mut headers = HashMap::new();
        headers.insert("KC-API-KEY".to_string(), api_key);
        headers.insert("KC-API-SIGN".to_string(), signature);
        headers.insert("KC-API-TIMESTAMP".to_string(), timestamp.to_string());
        headers.insert("KC-API-PASSPHRASE".to_string(), passphrase_signature);
        headers.insert("KC-API-KEY-VERSION".to_string(), "2".to_string());

        Ok(headers)
    }

    /// Make a GET request to the private API
    pub async fn get<T>(
        &self,
        endpoint: &str,
        params: Option<HashMap<String, String>>,
    ) -> Result<(RestResponse<T>, ResponseHeaders)>
    where
        T: DeserializeOwned,
    {
        // Check rate limiter
        if !self.rate_limiter.can_proceed().await {
            return Err(ApiError::RateLimitExceeded {
                code: "429000".to_string(),
                message: "Rate limit exceeded".to_string(),
            }
            .into());
        }

        let timestamp = Utc::now().timestamp_millis();
        let url = format!("{}{}", self.base_url, endpoint);

        let mut request = self.client.get(&url);

        if let Some(params) = params {
            request = request.query(&params);
        }

        // Create auth headers
        let auth_headers = self.create_auth_headers("GET", endpoint, "", timestamp)?;

        for (key, value) in auth_headers {
            request = request.header(&key, &value);
        }

        let response = request.send().await?;

        let status = response.status();
        let headers = response.headers().clone();

        let text = response.text().await?;

        if !status.is_success() {
            // Try to parse as error response
            if let Ok(error_response) =
                serde_json::from_str::<super::super::super::ErrorResponse>(&text)
            {
                return Err(ApiError::from(error_response).into());
            } else {
                return Err(ApiError::Http(format!("HTTP {}: {}", status, text)).into());
            }
        }

        // Parse successful response
        let response: RestResponse<T> = serde_json::from_str(&text)
            .map_err(|e| ApiError::JsonParsing(format!("Failed to parse response: {}", e)))?;

        // Check if KuCoin indicates success
        if !response.is_success() {
            return Err(ApiError::Other {
                code: response.code.clone(),
                message: "KuCoin API returned non-success code".to_string(),
            }
            .into());
        }

        let rate_limit_headers = ResponseHeaders::from_headers(&headers);

        Ok((response, rate_limit_headers))
    }

    /// Make a POST request to the private API
    pub async fn post<T>(
        &self,
        endpoint: &str,
        body: &str,
    ) -> Result<(RestResponse<T>, ResponseHeaders)>
    where
        T: DeserializeOwned,
    {
        // Check rate limiter
        if !self.rate_limiter.can_proceed().await {
            return Err(ApiError::RateLimitExceeded {
                code: "429000".to_string(),
                message: "Rate limit exceeded".to_string(),
            }
            .into());
        }

        let timestamp = Utc::now().timestamp_millis();
        let url = format!("{}{}", self.base_url, endpoint);

        let mut request = self.client.post(&url);

        // Create auth headers
        let auth_headers = self.create_auth_headers("POST", endpoint, body, timestamp)?;

        for (key, value) in auth_headers {
            request = request.header(&key, &value);
        }

        request = request
            .header("Content-Type", "application/json")
            .body(body.to_string());

        let response = request.send().await?;

        let status = response.status();
        let headers = response.headers().clone();

        let text = response.text().await?;

        if !status.is_success() {
            // Try to parse as error response
            if let Ok(error_response) =
                serde_json::from_str::<super::super::super::ErrorResponse>(&text)
            {
                return Err(ApiError::from(error_response).into());
            } else {
                return Err(ApiError::Http(format!("HTTP {}: {}", status, text)).into());
            }
        }

        // Parse successful response
        let response: RestResponse<T> = serde_json::from_str(&text)
            .map_err(|e| ApiError::JsonParsing(format!("Failed to parse response: {}", e)))?;

        // Check if KuCoin indicates success
        if !response.is_success() {
            return Err(ApiError::Other {
                code: response.code.clone(),
                message: "KuCoin API returned non-success code".to_string(),
            }
            .into());
        }

        let rate_limit_headers = ResponseHeaders::from_headers(&headers);

        Ok((response, rate_limit_headers))
    }

    /// Make a DELETE request to the private API
    pub async fn delete<T>(
        &self,
        endpoint: &str,
        params: Option<HashMap<String, String>>,
    ) -> Result<(RestResponse<T>, ResponseHeaders)>
    where
        T: DeserializeOwned,
    {
        // Check rate limiter
        if !self.rate_limiter.can_proceed().await {
            return Err(ApiError::RateLimitExceeded {
                code: "429000".to_string(),
                message: "Rate limit exceeded".to_string(),
            }
            .into());
        }

        let timestamp = Utc::now().timestamp_millis();
        let url = format!("{}{}", self.base_url, endpoint);

        let mut request = self.client.delete(&url);

        if let Some(params) = params {
            request = request.query(&params);
        }

        // Create auth headers
        let auth_headers = self.create_auth_headers("DELETE", endpoint, "", timestamp)?;

        for (key, value) in auth_headers {
            request = request.header(&key, &value);
        }

        let response = request.send().await?;

        let status = response.status();
        let headers = response.headers().clone();

        let text = response.text().await?;

        if !status.is_success() {
            // Try to parse as error response
            if let Ok(error_response) =
                serde_json::from_str::<super::super::super::ErrorResponse>(&text)
            {
                return Err(ApiError::from(error_response).into());
            } else {
                return Err(ApiError::Http(format!("HTTP {}: {}", status, text)).into());
            }
        }

        // Parse successful response
        let response: RestResponse<T> = serde_json::from_str(&text)
            .map_err(|e| ApiError::JsonParsing(format!("Failed to parse response: {}", e)))?;

        // Check if KuCoin indicates success
        if !response.is_success() {
            return Err(ApiError::Other {
                code: response.code.clone(),
                message: "KuCoin API returned non-success code".to_string(),
            }
            .into());
        }

        let rate_limit_headers = ResponseHeaders::from_headers(&headers);

        Ok((response, rate_limit_headers))
    }
}
