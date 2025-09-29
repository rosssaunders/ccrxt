use std::{borrow::Cow, sync::Arc, time::Instant};

use rest::HttpClient;
use serde::Serialize;

use crate::binance::{
    coinm::{CoinmConfig, Errors, ResponseHeaders, RestResponse, RestResult},
    shared::{
        Errors as SharedErrors, client::PrivateBinanceClient, credentials::Credentials,
        rate_limiter_trait::BinanceRateLimiter, venue_trait::VenueConfig,
    },
};

pub struct CoinmRestClient(PrivateBinanceClient);

pub type RestClient = CoinmRestClient;

impl From<PrivateBinanceClient> for CoinmRestClient {
    fn from(client: PrivateBinanceClient) -> Self {
        CoinmRestClient(client)
    }
}

impl CoinmRestClient {
    /// Create a new CoinmRestClient with credentials, HTTP client, and rate limiter
    ///
    /// Creates a new private REST client for Binance Coin-M Futures using the provided credentials,
    /// HTTP client implementation, and rate limiter.
    ///
    /// # Arguments
    /// * `credentials` - API credentials containing key and secret
    /// * `http_client` - HTTP client implementation to use for requests
    /// * `rate_limiter` - Rate limiter implementation for request throttling
    ///
    /// # Returns
    /// A new `CoinmRestClient` instance configured for Coin-M Futures trading
    ///
    /// # Example
    /// ```no_run
    /// use std::sync::Arc;
    /// use rest::HttpClient;
    /// use secrets::SecretString;
    /// // Use public re-exports instead of private module paths
    /// use venues::binance::shared::{credentials::Credentials, rate_limiter::RateLimiter, venue_trait::VenueConfig};
    /// use venues::binance::coinm::{PrivateRestClient as RestClient, CoinmConfig};
    ///
    /// # #[derive(Debug)]
    /// # struct MyHttpClient;
    /// # #[async_trait::async_trait]
    /// # impl HttpClient for MyHttpClient {
    /// #     async fn execute(&self, _: rest::Request) -> Result<rest::Response, rest::HttpError> {
    /// #         unimplemented!()
    /// #     }
    /// # }
    ///
    /// let credentials = Credentials {
    ///     api_key: SecretString::new("your_api_key".to_string().into()),
    ///     api_secret: SecretString::new("your_api_secret".to_string().into()),
    /// };
    ///
    /// let http_client: Arc<dyn HttpClient> = Arc::new(MyHttpClient);
    /// let config = CoinmConfig;
    /// let rate_limiter = Arc::new(RateLimiter::new(config.rate_limits()));
    /// let client = RestClient::new(credentials, http_client, rate_limiter);
    /// ```
    pub fn new(
        credentials: Credentials,
        http_client: Arc<dyn HttpClient>,
        rate_limiter: Arc<dyn BinanceRateLimiter>,
    ) -> Self {
        let config = CoinmConfig;

        let private_client = PrivateBinanceClient::new(
            Cow::Owned(config.base_url().to_string()),
            http_client,
            rate_limiter,
            Box::new(credentials.api_key.clone()),
            Box::new(credentials.api_secret.clone()),
        );

        CoinmRestClient(private_client)
    }
    /// Send a signed GET request with coinm-specific response type (high-performance)
    pub async fn send_get_signed_request<T, R>(
        &self,
        endpoint: &str,
        params: R,
        weight: u32,
        is_order: bool,
    ) -> RestResult<T>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
        R: Serialize,
    {
        let start = Instant::now();

        // Call the shared client's high-performance GET function
        let shared_response = PrivateBinanceClient::send_get_signed_request::<T, R, SharedErrors>(
            &self.0, endpoint, params, weight, is_order,
        )
        .await
        .map_err(|e| match e {
            SharedErrors::Api(_) => Errors::Error("API error occurred".to_string()),
            SharedErrors::RateLimitExceeded { retry_after } => Errors::Error(format!(
                "Rate limit exceeded, retry after {:?}",
                retry_after
            )),
            SharedErrors::Http { message: msg } => Errors::Error(format!("HTTP error: {msg}")),
            SharedErrors::InvalidApiKey => Errors::Error("Invalid API key".to_string()),
            SharedErrors::Serialize { message: msg } => {
                Errors::Error(format!("Serialization error: {msg}"))
            }
            SharedErrors::Deserialize { message: msg } => {
                Errors::Error(format!("Deserialization error: {msg}"))
            }
            SharedErrors::Generic { message: msg } => Errors::Error(msg),
        })?;

        let duration = start.elapsed();
        tracing::debug!("Request to {endpoint} took {duration:?}");

        Ok(RestResponse {
            data: shared_response.data,
            request_duration: duration,
            headers: ResponseHeaders::default(), // TODO: Convert headers properly
        })
    }

    /// Send a signed POST request with coinm-specific response type (high-performance)
    pub async fn send_post_signed_request<T, R>(
        &self,
        endpoint: &str,
        params: R,
        weight: u32,
        is_order: bool,
    ) -> RestResult<T>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
        R: Serialize,
    {
        let start = Instant::now();

        // Call the shared client's high-performance POST function
        let shared_response = PrivateBinanceClient::send_post_signed_request::<T, R, SharedErrors>(
            &self.0, endpoint, params, weight, is_order,
        )
        .await
        .map_err(|e| match e {
            SharedErrors::Api(_) => Errors::Error("API error occurred".to_string()),
            SharedErrors::RateLimitExceeded { retry_after } => Errors::Error(format!(
                "Rate limit exceeded, retry after {:?}",
                retry_after
            )),
            SharedErrors::Http { message: msg } => Errors::Error(format!("HTTP error: {msg}")),
            SharedErrors::InvalidApiKey => Errors::Error("Invalid API key".to_string()),
            SharedErrors::Serialize { message: msg } => {
                Errors::Error(format!("Serialization error: {msg}"))
            }
            SharedErrors::Deserialize { message: msg } => {
                Errors::Error(format!("Deserialization error: {msg}"))
            }
            SharedErrors::Generic { message: msg } => Errors::Error(msg),
        })?;

        let duration = start.elapsed();
        tracing::debug!("Request to {endpoint} took {duration:?}");

        Ok(RestResponse {
            data: shared_response.data,
            request_duration: duration,
            headers: ResponseHeaders::default(), // TODO: Convert headers properly
        })
    }

    /// Send a signed PUT request with coinm-specific response type (high-performance)
    pub async fn send_put_signed_request<T, R>(
        &self,
        endpoint: &str,
        params: R,
        weight: u32,
        is_order: bool,
    ) -> RestResult<T>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
        R: Serialize,
    {
        let start = Instant::now();

        // Call the shared client's high-performance PUT function
        let shared_response = PrivateBinanceClient::send_put_signed_request::<T, R, SharedErrors>(
            &self.0, endpoint, params, weight, is_order,
        )
        .await
        .map_err(|e| match e {
            SharedErrors::Api(_) => Errors::Error("API error occurred".to_string()),
            SharedErrors::RateLimitExceeded { retry_after } => Errors::Error(format!(
                "Rate limit exceeded, retry after {:?}",
                retry_after
            )),
            SharedErrors::Http { message: msg } => Errors::Error(format!("HTTP error: {msg}")),
            SharedErrors::InvalidApiKey => Errors::Error("Invalid API key".to_string()),
            SharedErrors::Serialize { message: msg } => {
                Errors::Error(format!("Serialization error: {msg}"))
            }
            SharedErrors::Deserialize { message: msg } => {
                Errors::Error(format!("Deserialization error: {msg}"))
            }
            SharedErrors::Generic { message: msg } => Errors::Error(msg),
        })?;

        let duration = start.elapsed();
        tracing::debug!("Request to {endpoint} took {duration:?}");

        Ok(RestResponse {
            data: shared_response.data,
            request_duration: duration,
            headers: ResponseHeaders::default(), // TODO: Convert headers properly
        })
    }

    /// Send a signed DELETE request with coinm-specific response type (high-performance)
    pub async fn send_delete_signed_request<T, R>(
        &self,
        endpoint: &str,
        params: R,
        weight: u32,
        is_order: bool,
    ) -> RestResult<T>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
        R: Serialize,
    {
        let start = Instant::now();

        // Call the shared client's high-performance DELETE function
        let shared_response =
            PrivateBinanceClient::send_delete_signed_request::<T, R, SharedErrors>(
                &self.0, endpoint, params, weight, is_order,
            )
            .await
            .map_err(|e| match e {
                SharedErrors::Api(_) => Errors::Error("API error occurred".to_string()),
                SharedErrors::RateLimitExceeded { retry_after } => Errors::Error(format!(
                    "Rate limit exceeded, retry after {:?}",
                    retry_after
                )),
                SharedErrors::Http { message: msg } => Errors::Error(format!("HTTP error: {msg}")),
                SharedErrors::InvalidApiKey => Errors::Error("Invalid API key".to_string()),
                SharedErrors::Serialize { message: msg } => {
                    Errors::Error(format!("Serialization error: {msg}"))
                }
                SharedErrors::Deserialize { message: msg } => {
                    Errors::Error(format!("Deserialization error: {msg}"))
                }
                SharedErrors::Generic { message: msg } => Errors::Error(msg),
            })?;

        let duration = start.elapsed();
        tracing::debug!("Request to {endpoint} took {duration:?}");

        Ok(RestResponse {
            data: shared_response.data,
            request_duration: duration,
            headers: ResponseHeaders::default(), // TODO: Convert headers properly
        })
    }

    /// ⚠️ DEPRECATED: Use verb-specific functions instead for better performance
    ///
    /// This function remains for backward compatibility but creates branch prediction penalties.
    /// Use send_get_signed_request, send_post_signed_request, etc. instead.
    #[deprecated(
        note = "Use verb-specific functions (send_get_signed_request, send_post_signed_request, etc.) for better performance"
    )]
    pub async fn send_signed_request<T, R>(
        &self,
        endpoint: &str,
        method: rest::http_client::Method,
        params: R,
        weight: u32,
        is_order: bool,
    ) -> RestResult<T>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
        R: Serialize,
    {
        // Route to appropriate verb-specific method based on HTTP method
        match method {
            rest::http_client::Method::Get => {
                self.send_get_signed_request(endpoint, params, weight, is_order)
                    .await
            }
            rest::http_client::Method::Post => {
                self.send_post_signed_request(endpoint, params, weight, is_order)
                    .await
            }
            rest::http_client::Method::Put => {
                self.send_put_signed_request(endpoint, params, weight, is_order)
                    .await
            }
            rest::http_client::Method::Delete => {
                self.send_delete_signed_request(endpoint, params, weight, is_order)
                    .await
            }
            _ => Err(Errors::Error(format!(
                "Unsupported HTTP method: {:?}",
                method
            ))),
        }
    }
}
