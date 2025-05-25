//! Binance Coin-M Futures API request handling module.
//! 
//! This module provides functionality for making HTTP requests to the Binance Coin-M Futures API.
//! It handles authentication, rate limiting headers, error responses, and request/response timing.
//! 
//! ## Binance Exchange Behavior
//! 
//! The Binance API has specific behaviors that this module handles:
//! 
//! - **Dual Error Format**: Binance can return errors in two ways:
//!   1. HTTP error status codes with error JSON in the body
//!   2. HTTP 200 OK with error details in the response JSON (disguised errors)
//! 
//! - **Rate Limiting Headers**: Binance includes rate limiting information in response headers:
//!   - `X-MBX-USED-WEIGHT-1M`: API weight used in the last minute
//!   - `X-MBX-ORDER-COUNT-1M`: Orders placed in the last minute  
//!   - `X-MBX-ORDER-COUNT-1D`: Orders placed in the last day
//!   - `X-MBX-ORDER-COUNT-1S`: Orders placed in the last second
//! 
//! - **Authentication**: Requires API key in `X-MBX-APIKEY` header for authenticated endpoints
//! 
//! - **Timestamp Requirements**: Signed requests must include a timestamp parameter and signature
//!   based on the current UTC timestamp in milliseconds
//! 
//! - **Request Signing**: For private endpoints, query parameters (including timestamp) must be
//!   signed using HMAC-SHA256 with the API secret
use reqwest::Client;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use hex;
use secrecy::{ExposeSecret, SecretString};
use serde::Serialize;
use super::types::{BinanceCoinMResponse, BinanceHeaders, BinanceCoinMError, BinanceCoinMResult, ErrorResponse, OrderRequest, OrderResponse};
use super::api_errors::BinanceCoinMAPIError;
use std::time::Instant;
use reqwest::StatusCode;
use serde::{Deserialize};

/// Represents a successful or error response from the Binance API.
/// This enum is used to handle both successful responses and error responses
/// in a unified way, allowing for easier error handling and response parsing.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ApiResponse<T> {
    Ok(T),
    Err(ErrorResponse),
}

/// A client for interacting with the Binance Coin-M Futures private REST API
/// 
/// This client handles encrypted API keys and secrets for enhanced security.
/// The API key and secret are stored in encrypted form and only decrypted when needed.
pub struct BinanceCoinMPrivateRest {
    pub(crate) client: Client,
    //pub(crate) rate_limiter: BinanceCoinMRateLimiter,
    pub(crate) encrypted_api_key: SecretString,
    pub(crate) encrypted_api_secret: SecretString,
    pub(crate) base_url: String,
}

impl BinanceCoinMPrivateRest {
    /// Creates a new BinanceCoinMPrivateRest client with encrypted API credentials
    /// 
    /// # Arguments
    /// * `encrypted_api_key` - The encrypted API key
    /// * `encrypted_api_secret` - The encrypted API secret
    /// * `base_url` - The base URL for the API
    /// * `encryption_key` - The key used for decrypting the API credentials
    /// 
    /// # Returns
    /// A new BinanceCoinMPrivateRest client instance
    pub fn new(
        encrypted_api_key: impl Into<SecretString>,
        encrypted_api_secret: impl Into<SecretString>,
        base_url: String
    ) -> Self {
        Self {
            client: Client::new(),
            //rate_limiter: BinanceCoinMRateLimiter::new(),
            encrypted_api_key: encrypted_api_key.into(),
            encrypted_api_secret: encrypted_api_secret.into(),
            base_url,
            //encryption_key: Secret::new(encryption_key),
        }
    }

    /// Signs a request using the decrypted API secret
    pub fn sign_request(&self, query_string: &str) -> Result<String, BinanceCoinMError> {
        let api_secret = self.encrypted_api_secret.expose_secret();
        let mut mac = Hmac::<Sha256>::new_from_slice(api_secret.as_bytes())
            .map_err(|e| BinanceCoinMError::Error(format!("SigningFailed: {}", e)))?;
        mac.update(query_string.as_bytes());
        Ok(hex::encode(mac.finalize().into_bytes()))
    }

    /// Sends a request to the Binance API
    /// 
    /// This method encapsulates all the logic for making authenticated requests to the Binance API,
    /// including rate limiting, error handling, and response parsing.
    /// 
    /// # Arguments
    /// * `endpoint` - The API endpoint path (e.g., "/fapi/v1/order")
    /// * `method` - The HTTP method to use
    /// * `query_string` - Optional query string parameters
    /// 
    /// # Returns
    /// A result containing the parsed response data and metadata, or an error
    pub async fn send_request<T>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        query_string: Option<&str>,
    ) -> BinanceCoinMResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let rate_limit_start = Instant::now();
        // TODO: Add rate limiting check here
        // self.rate_limiter.check().await?;

        let url = match query_string {
            Some(qs) => format!("{}{}?{}", self.base_url, endpoint, qs),
            None => format!("{}{}", self.base_url, endpoint),
        };

        let api_key = self.encrypted_api_key.expose_secret();
        let mut request = self.client.request(method, &url);
        request = request.header("X-MBX-APIKEY", api_key);

        let request_start = Instant::now();
        let response = request.send().await.map_err(BinanceCoinMError::HttpError)?;
        let rate_limit_duration = rate_limit_start.elapsed();
        let request_duration = request_start.elapsed();

        let headers = BinanceHeaders {
            used_weight_1m: response.headers()
                .get("X-MBX-USED-WEIGHT-1M")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse().ok()),
            order_count_1m: response.headers()
                .get("X-MBX-ORDER-COUNT-1M")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse().ok()),
            order_count_1d: response.headers()
                .get("X-MBX-ORDER-COUNT-1D")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse().ok()),
            order_count_1s: response.headers()
                .get("X-MBX-ORDER-COUNT-1S")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse().ok()),
        };

        match response.status() {
            StatusCode::OK => {
                let text = response.text().await.map_err(BinanceCoinMError::HttpError)?;
                let data: T = serde_json::from_str(&text)
                    .map_err(|e| BinanceCoinMError::Error(format!("JSON decode error: {} | body: {}", e, text)))?;
                
                // Check if the response is actually an error disguised as success
                if let Ok(api_response) = serde_json::from_str::<ApiResponse<T>>(&text) {
                    match api_response {
                        ApiResponse::Err(err) => return Err(BinanceCoinMError::ApiError(BinanceCoinMAPIError::from(err))),
                        ApiResponse::Ok(_) => {} // Continue with normal flow
                    }
                }
                
                Ok(BinanceCoinMResponse {
                    data,
                    rate_limit_duration,
                    request_duration,
                    headers,
                })
            }
            _status => {
                let text = response.text().await.map_err(BinanceCoinMError::HttpError)?;
                let err: ErrorResponse = serde_json::from_str(&text)
                    .map_err(|e| BinanceCoinMError::Error(format!("JSON decode error: {} | body: {}", e, text)))?;
                Err(BinanceCoinMError::ApiError(BinanceCoinMAPIError::from(err)))
            }
        }
    }

    /// Sends a signed request to the Binance API
    /// 
    /// This method automatically handles timestamp generation and request signing for private endpoints.
    /// It appends the current timestamp and generates the required signature.
    /// 
    /// # Arguments
    /// * `endpoint` - The API endpoint path (e.g., "/fapi/v1/order")
    /// * `method` - The HTTP method to use
    /// * `query_params` - Optional query parameters (without timestamp or signature)
    /// 
    /// # Returns
    /// A result containing the parsed response data and metadata, or an error
    pub async fn send_signed_request<T>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        query_params: Option<String>,
    ) -> BinanceCoinMResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        use super::utils::append_timestamp_and_signature;
        
        let query_string = query_params.unwrap_or_default();
        let signed_query = append_timestamp_and_signature(query_string, |qs| self.sign_request(qs))?;
        
        self.send_request(endpoint, method, Some(&signed_query)).await
    }

    /// Places a new order on Binance COIN-M futures
    /// 
    /// # Arguments
    /// * `order` - The order parameters
    /// 
    /// # Returns
    /// A result containing the order response, or an error
    /// 
    /// # API Documentation
    /// See: https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api
    /// 
    /// # Endpoint Weight
    /// 1 on 1min order rate limit(X-MBX-ORDER-COUNT-1M)
    /// 0 on IP rate limit(x-mbx-used-weight-1m)
    /// 
    /// # Examples
    /// ```
    /// use venues::binance::coinm::{BinanceCoinMPrivateRest, OrderRequest, OrderSide, OrderType};
    /// use rust_decimal::Decimal;
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BinanceCoinMPrivateRest::new(
    ///     "your-api-key",
    ///     "your-api-secret",
    ///     "https://dapi.binance.com".to_string()
    /// );
    /// 
    /// let mut order = OrderRequest {
    ///     symbol: "BTCUSD_PERP".to_string(),
    ///     side: OrderSide::Buy,
    ///     order_type: OrderType::Market,
    ///     quantity: Some(Decimal::from(1)),
    ///     positionSide: None,
    ///     timeInForce: None,
    ///     reduceOnly: None,
    ///     price: None,
    ///     newClientOrderId: None,
    ///     stopPrice: None,
    ///     closePosition: None,
    ///     activationPrice: None,
    ///     callbackRate: None,
    ///     workingType: None,
    ///     priceProtect: None,
    ///     newOrderRespType: None,
    ///     priceMatch: None,
    ///     selfTradePreventionMode: None,
    ///     recvWindow: None,
    ///     timestamp: None,
    /// };
    /// 
    /// let result = client.place_order(order).await?;
    /// println!("Order placed: {:?}", result.data);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn place_order(&self, mut order: OrderRequest) -> BinanceCoinMResult<OrderResponse> {
        use serde_urlencoded::to_string;

        // Ensure order has a timestamp if not already set
        if order.timestamp.is_none() {
            order.timestamp = Some(std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64);
        }

        // Convert the order into a serializable form
        #[derive(Serialize)]
        struct OrderParams {
            #[serde(flatten)]
            params: OrderRequest,
            timestamp: Option<u64>,
        }

        let params = OrderParams {
            params: order,
            timestamp: None, // Will be added by send_signed_request
        };

        // Convert struct to URL-encoded query string
        let query = to_string(&params)
            .map_err(|e| BinanceCoinMError::Error(format!("Failed to encode order parameters: {}", e)))?;

        // Send the request
        self.send_signed_request(
            "/dapi/v1/order", 
            reqwest::Method::POST,
            Some(query),
        ).await
    }
}