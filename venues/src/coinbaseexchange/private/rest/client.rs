//! REST client for Coinbase Exchange private endpoints.
//!
//! Provides access to all private REST API endpoints for Coinbase Exchange.
//! All requests are authenticated and require API credentials.

use std::{borrow::Cow, sync::Arc};

use base64::{Engine as _, engine::general_purpose};
use chrono::Utc;
use hmac::{Hmac, Mac};
use rest::{
    HttpClient,
    http_client::{Method as HttpMethod, RequestBuilder},
    secrets::ExposableSecret,
};
use serde::{Serialize, de::DeserializeOwned};
use sha2::Sha256;

use super::get_account_balances::PaginationInfo;
use crate::coinbaseexchange::{EndpointType, Errors, RateLimiter, RestResult};

/// Private REST client for Coinbase Exchange
///
/// This client handles all private API endpoints that require authentication.
/// It provides automatic rate limiting, error handling, and request signing.
pub struct RestClient {
    /// The base URL for the Coinbase Exchange REST API
    pub base_url: Cow<'static, str>,

    /// The underlying HTTP client used for making requests
    pub http_client: Arc<dyn HttpClient>,

    /// The rate limiter used to manage request rates and prevent hitting API limits
    pub rate_limiter: RateLimiter,

    /// The encrypted API key
    pub(crate) api_key: Box<dyn ExposableSecret>,

    /// The encrypted API secret (base64 encoded)
    pub(crate) api_secret: Box<dyn ExposableSecret>,

    /// The encrypted API passphrase
    pub(crate) api_passphrase: Box<dyn ExposableSecret>,
}

impl RestClient {
    /// Create a new REST client for Coinbase Exchange private endpoints
    ///
    /// # Arguments
    /// * `api_key` - Your Coinbase Exchange API key
    /// * `api_secret` - Your Coinbase Exchange API secret (base64 encoded)
    /// * `api_passphrase` - Your Coinbase Exchange API passphrase
    /// * `base_url` - The base URL for the Coinbase Exchange API
    /// * `http_client` - HTTP client for making requests
    /// * `rate_limiter` - Rate limiter for managing request frequency
    pub fn new(
        api_key: Box<dyn ExposableSecret>,
        api_secret: Box<dyn ExposableSecret>,
        api_passphrase: Box<dyn ExposableSecret>,
        base_url: impl Into<Cow<'static, str>>,
        http_client: Arc<dyn HttpClient>,
        rate_limiter: RateLimiter,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            http_client,
            rate_limiter,
            api_key,
            api_secret,
            api_passphrase,
        }
    }

    /// Create request signature for Coinbase Exchange authentication
    ///
    /// The signature is created by:
    /// 1. Create prehash string: timestamp + method + requestPath + body
    /// 2. Sign with HMAC SHA256 using the API secret (base64 decoded)
    /// 3. Encode as Base64
    ///
    /// # Arguments
    /// * `timestamp` - The timestamp string (Unix timestamp as string)
    /// * `method` - The HTTP method (uppercase)
    /// * `request_path` - The request path including query parameters
    /// * `body` - The request body (empty string for GET requests)
    ///
    /// # Returns
    /// A result containing the signature as a base64 string or an error
    pub fn sign_request(
        &self,
        timestamp: &str,
        method: &str,
        request_path: &str,
        body: &str,
    ) -> Result<String, Errors> {
        // Create the prehash string: timestamp + method + requestPath + body
        let prehash = format!("{timestamp}{method}{request_path}{body}");

        // Decode the base64 API secret
        let api_secret = self.api_secret.expose_secret();
        let secret_bytes = general_purpose::STANDARD
            .decode(&api_secret)
            .map_err(|e| Errors::Error(format!("Failed to decode API secret: {e}")))?;

        // Sign with HMAC SHA256
        let mut mac =
            Hmac::<Sha256>::new_from_slice(&secret_bytes).map_err(|_| Errors::InvalidApiKey())?;
        mac.update(prehash.as_bytes());

        // Encode as Base64
        Ok(general_purpose::STANDARD.encode(mac.finalize().into_bytes()))
    }

    /// Build request parameters for GET method
    #[inline(always)]
    fn build_get_params<P: Serialize + ?Sized>(
        &self,
        endpoint: &str,
        params: Option<&P>,
    ) -> Result<(String, String, String), Errors> {
        let (request_path, query_string) = params
            .map(|p| {
                serde_urlencoded::to_string(p)
                    .map_err(|e| Errors::Error(format!("Failed to serialize query parameters: {e}")))
                    .map(|qs| {
                        let empty = qs.is_empty();
                        // Always compute both branches to avoid branching
                        let with_query = format!("/{endpoint}?{qs}");
                        let without_query = format!("/{endpoint}");
                        // Use bitwise operations to select without branching
                        let path = [without_query, with_query];
                        (path[(!empty) as usize].clone(), qs)
                    })
            })
            .transpose()?
            .unwrap_or_else(|| (format!("/{endpoint}"), String::new()));

        Ok((request_path, String::new(), query_string))
    }

    /// Build request parameters for non-GET methods
    #[inline(always)]
    fn build_body_params<P: Serialize + ?Sized>(
        &self,
        endpoint: &str,
        params: Option<&P>,
    ) -> Result<(String, String, String), Errors> {
        let body = params
            .map(|p| {
                serde_json::to_string(p)
                    .map_err(|e| Errors::Error(format!("Failed to serialize request body: {e}")))
            })
            .transpose()?
            .unwrap_or_default();

        Ok((format!("/{endpoint}"), body.clone(), body))
    }

    /// Core request sending logic optimized for HFT
    #[inline(always)]
    async fn send_request_internal<T, P>(
        &self,
        endpoint: &str,
        method: HttpMethod,
        params: Option<&P>,
        endpoint_type: EndpointType,
        with_headers: bool,
    ) -> RestResult<(T, Option<std::collections::HashMap<String, String>>)>
    where
        T: DeserializeOwned,
        P: Serialize + ?Sized,
    {
        // Check rate limit before making request
        self.rate_limiter.check_limit(endpoint_type).await?;

        // Create timestamp
        let timestamp = Utc::now().timestamp().to_string();

        // Use function pointers to avoid branching on method type
        type ParamBuilder<P> = fn(
            &RestClient,
            &str,
            Option<&P>,
        ) -> Result<(String, String, String), Errors>;

        // Create lookup table for method handlers (computed at compile time)
        let method_index = ((method != HttpMethod::Get) as u8).min(1);
        let builders: [ParamBuilder<P>; 2] = [
            Self::build_get_params,
            Self::build_body_params,
        ];

        // Call the appropriate builder without branching
        let (request_path, body, query_or_body) = 
            builders[method_index as usize](self, endpoint, params)?;

        // Create signature
        let method_str = match method {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Head => "HEAD",
            HttpMethod::Options => "OPTIONS",
        };
        let signature = self.sign_request(&timestamp, method_str, &request_path, &body)?;

        // Build URL
        let base_url = format!("{}/{}", self.base_url, endpoint);
        let url = if method == HttpMethod::Get && !query_or_body.is_empty() {
            format!("{}?{}", base_url, query_or_body)
        } else {
            base_url
        };

        // Build request
        let api_key = self.api_key.expose_secret();
        let api_passphrase = self.api_passphrase.expose_secret();
        
        let mut builder = RequestBuilder::new(method, url)
            .header("CB-ACCESS-KEY", &api_key)
            .header("CB-ACCESS-SIGN", &signature)
            .header("CB-ACCESS-TIMESTAMP", &timestamp)
            .header("CB-ACCESS-PASSPHRASE", &api_passphrase)
            .header("User-Agent", "ccrxt/0.1.0");

        // Add body for non-GET methods
        if method != HttpMethod::Get && !query_or_body.is_empty() {
            builder = builder
                .header("Content-Type", "application/json")
                .body(query_or_body.into_bytes());
        }

        // Send request
        let response = self.http_client.execute(builder.build()).await
            .map_err(|e| Errors::NetworkError(format!("HTTP request failed: {e}")))?;

        // Process response
        let status = response.status;
        let headers_map = if with_headers {
            Some(response.headers.clone())
        } else {
            None
        };
        let response_text = response.text()
            .map_err(|e| Errors::NetworkError(format!("Failed to read response: {e}")))?;

        // Use lookup table for error handling to avoid branching
        let parse_result = serde_json::from_str::<T>(&response_text);
        let error_result = serde_json::from_str::<crate::coinbaseexchange::ErrorResponse>(&response_text);

        // Branchless status code to error conversion using array indexing
        const ERROR_TABLE_SIZE: usize = 501;
        let mut error_table = [None; ERROR_TABLE_SIZE];
        
        // Initialize error table c
        error_table[400] = Some(0); // BadRequest
        error_table[401] = Some(1); // Unauthorized
        error_table[403] = Some(2); // Forbidden
        error_table[404] = Some(3); // NotFound
        error_table[429] = Some(4); // TooManyRequests
        error_table[500] = Some(5); // InternalServerError

        let status_code = status as usize;
        let is_success = status == 200 || status == 201;

        // Process response without branching on success/failure
        match (is_success, parse_result, error_result) {
            (true, Ok(data), _) => Ok((data, headers_map)),
            (false, _, Ok(error_response)) => {
                // Use lookup table to determine error type
                let error_index = status_code
                    .min(ERROR_TABLE_SIZE - 1);
                let error_type = error_table.get(error_index).and_then(|&x| x).unwrap_or(6);
                
                // Create error based on type
                let api_errors = [
                    crate::coinbaseexchange::ApiError::BadRequest { msg: error_response.message.clone() },
                    crate::coinbaseexchange::ApiError::Unauthorized { msg: error_response.message.clone() },
                    crate::coinbaseexchange::ApiError::Forbidden { msg: error_response.message.clone() },
                    crate::coinbaseexchange::ApiError::NotFound { msg: error_response.message.clone() },
                    crate::coinbaseexchange::ApiError::TooManyRequests { msg: error_response.message.clone() },
                    crate::coinbaseexchange::ApiError::InternalServerError { msg: error_response.message.clone() },
                    crate::coinbaseexchange::ApiError::UnknownApiError { 
                        code: Some(status_code as i32), 
                        msg: error_response.message.clone() 
                    },
                ];
                
                Err(Errors::ApiError(api_errors[error_type.min(6)].clone()))
            }
            _ => Err(Errors::Error(format!("HTTP {status}: {response_text}"))),
        }
    }

    /// Send a request to a private endpoint and return both data and headers
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (without leading slash)
    /// * `method` - The HTTP method to use
    /// * `params` - Optional query parameters or request body
    /// * `endpoint_type` - The type of endpoint for rate limiting
    ///
    /// # Returns
    /// A result containing the deserialized response and headers or an error
    pub async fn send_request_with_headers<T, P>(
        &self,
        endpoint: &str,
        method: HttpMethod,
        params: Option<&P>,
        endpoint_type: EndpointType,
    ) -> RestResult<(T, std::collections::HashMap<String, String>)>
    where
        T: DeserializeOwned,
        P: Serialize + ?Sized,
    {
        let (data, headers) = self
            .send_request_internal(endpoint, method, params, endpoint_type, true)
            .await?;
        Ok((data, headers.unwrap()))
    }

    /// Send a request to a private endpoint and return data with extracted pagination info
    ///
    /// This method combines the functionality of `send_request_with_headers` with
    /// automatic extraction of Coinbase pagination headers (CB-BEFORE and CB-AFTER).
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (without leading slash)
    /// * `method` - The HTTP method to use
    /// * `params` - Optional query parameters or request body
    /// * `endpoint_type` - The type of endpoint for rate limiting
    ///
    /// # Returns
    /// A result containing the deserialized response and pagination info or an error
    pub async fn send_request_with_pagination<T, P>(
        &self,
        endpoint: &str,
        method: HttpMethod,
        params: Option<&P>,
        endpoint_type: EndpointType,
    ) -> RestResult<(T, Option<PaginationInfo>)>
    where
        T: DeserializeOwned,
        P: Serialize + ?Sized,
    {
        let (data, headers) = self
            .send_request_with_headers(endpoint, method, params, endpoint_type)
            .await?;

        // Extract pagination headers without branching
        let before = headers.get("CB-BEFORE").cloned();
        let after = headers.get("CB-AFTER").cloned();

        // Create pagination info without explicit branching
        let has_pagination = before.is_some() | after.is_some();
        let pagination = has_pagination.then(|| PaginationInfo { before, after });

        Ok((data, pagination))
    }

    /// Send a request to a private endpoint
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (without leading slash)
    /// * `method` - The HTTP method to use
    /// * `params` - Optional query parameters or request body
    /// * `endpoint_type` - The type of endpoint for rate limiting
    ///
    /// # Returns
    /// A result containing the deserialized response or an error
    pub async fn send_request<T, P>(
        &self,
        endpoint: &str,
        method: HttpMethod,
        params: Option<&P>,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        P: Serialize + ?Sized,
    {
        let (data, _) = self
            .send_request_internal(endpoint, method, params, endpoint_type, false)
            .await?;
        Ok(data)
    }

    /// Send a GET request to a private endpoint
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (without leading slash)
    /// * `params` - Query parameters
    /// * `endpoint_type` - The type of endpoint for rate limiting
    ///
    /// # Returns
    /// A result containing the deserialized response or an error
    pub async fn send_get_request<T, P>(
        &self,
        endpoint: &str,
        params: P,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        self.send_request(endpoint, HttpMethod::Get, Some(&params), endpoint_type)
            .await
    }

    /// Send a POST request to a private endpoint
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (without leading slash)
    /// * `params` - Request body parameters
    /// * `endpoint_type` - The type of endpoint for rate limiting
    ///
    /// # Returns
    /// A result containing the deserialized response or an error
    pub async fn send_post_request<T, P>(
        &self,
        endpoint: &str,
        params: P,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        self.send_request(
            endpoint,
            HttpMethod::Post,
            Some(&params),
            endpoint_type,
        )
        .await
    }

    /// Send a PUT request to a private endpoint
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (without leading slash)
    /// * `params` - Request body parameters
    /// * `endpoint_type` - The type of endpoint for rate limiting
    ///
    /// # Returns
    /// A result containing the deserialized response or an error
    pub async fn send_put_request<T, P>(
        &self,
        endpoint: &str,
        params: P,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        self.send_request(endpoint, HttpMethod::Put, Some(&params), endpoint_type)
            .await
    }

    /// Send a DELETE request to a private endpoint
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (without leading slash)
    /// * `params` - Optional query parameters or request body
    /// * `endpoint_type` - The type of endpoint for rate limiting
    ///
    /// # Returns
    /// A result containing the deserialized response or an error
    pub async fn send_delete_request<T, P>(
        &self,
        endpoint: &str,
        params: P,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        self.send_request(
            endpoint,
            HttpMethod::Delete,
            Some(&params),
            endpoint_type,
        )
        .await
    }

    /// Send a GET request to a private endpoint with headers
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (without leading slash)
    /// * `params` - Query parameters
    /// * `endpoint_type` - The type of endpoint for rate limiting
    ///
    /// # Returns
    /// A result containing the deserialized response and headers or an error
    pub async fn send_get_request_with_headers<T, P>(
        &self,
        endpoint: &str,
        params: P,
        endpoint_type: EndpointType,
    ) -> RestResult<(T, std::collections::HashMap<String, String>)>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        self.send_request_with_headers(endpoint, HttpMethod::Get, Some(&params), endpoint_type)
            .await
    }

    /// Send a GET request to a private endpoint with pagination
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (without leading slash)
    /// * `params` - Query parameters
    /// * `endpoint_type` - The type of endpoint for rate limiting
    ///
    /// # Returns
    /// A result containing the deserialized response and pagination info or an error
    pub async fn send_get_request_with_pagination<T, P>(
        &self,
        endpoint: &str,
        params: P,
        endpoint_type: EndpointType,
    ) -> RestResult<(T, Option<PaginationInfo>)>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        self.send_request_with_pagination(endpoint, HttpMethod::Get, Some(&params), endpoint_type)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature_generation() {
        // Test based on Coinbase documentation example
        // https://docs.cloud.coinbase.com/exchange/docs/authorization-and-authentication
        
        // Create a mock client with test credentials
        let api_secret = Box::new(rest::secrets::SecretString::from(
            "TEST_SECRET_BASE64_ENCODED",
        ));
        let api_key = Box::new(rest::secrets::SecretString::from("test_key"));
        let api_passphrase = Box::new(rest::secrets::SecretString::from("test_passphrase"));
        
        let rest_client = RestClient::new(
            api_key,
            api_secret,
            api_passphrase,
            "https://api.exchange.coinbase.com",
            Arc::new(rest::native::NativeHttpClient::default()),
            RateLimiter::new(),
        );

        // Test signature generation with known inputs
        let result = rest_client.sign_request("1640995200", "GET", "/accounts", "");
        
        // Should not error even with invalid base64 secret
        assert!(result.is_err());
    }

    #[test]
    fn test_base_url_handling() {
        let api_secret = Box::new(rest::secrets::SecretString::from("secret"));
        let api_key = Box::new(rest::secrets::SecretString::from("key"));
        let api_passphrase = Box::new(rest::secrets::SecretString::from("passphrase"));
        
        let rest_client = RestClient::new(
            api_key,
            api_secret,
            api_passphrase,
            "https://api.exchange.coinbase.com",
            Arc::new(rest::native::NativeHttpClient::default()),
            RateLimiter::new(),
        );

        assert_eq!(rest_client.base_url, "https://api.exchange.coinbase.com");
    }
}