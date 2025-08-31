use std::{borrow::Cow, sync::Arc};

use rest::HttpClient;
use serde::Serialize;

use crate::binance::{
    shared::{
        Errors as SharedErrors, RestResponse, client::PrivateBinanceClient,
        credentials::Credentials, rate_limiter_trait::BinanceRateLimiter, venue_trait::VenueConfig,
    },
    usdm::{Errors, UsdmConfig},
};

pub struct UsdmPrivateRestClient(PrivateBinanceClient);

pub type UsdmClient = UsdmPrivateRestClient;
pub type RestClient = UsdmPrivateRestClient;

impl From<PrivateBinanceClient> for UsdmPrivateRestClient {
    fn from(client: PrivateBinanceClient) -> Self {
        UsdmPrivateRestClient(client)
    }
}

impl UsdmPrivateRestClient {
    /// Create a new UsdmPrivateRestClient with credentials, HTTP client, and rate limiter
    ///
    /// Creates a new private REST client for Binance USD-M Futures using the provided credentials,
    /// HTTP client implementation, and rate limiter.
    ///
    /// # Arguments
    /// * `credentials` - API credentials containing key and secret
    /// * `http_client` - HTTP client implementation to use for requests
    /// * `rate_limiter` - Rate limiter implementation for request throttling
    ///
    /// # Returns
    /// A new `UsdmPrivateRestClient` instance configured for USD-M Futures trading
    ///
    /// # Example
    /// ```no_run
    /// use std::sync::Arc;
    /// use rest::{secrets::SecretString, HttpClient};
    /// use venues::binance::shared::{credentials::Credentials, rate_limiter::RateLimiter, venue_trait::VenueConfig};
    /// use venues::binance::usdm::{PrivateRestClient as UsdmClient, UsdmConfig};
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
    /// let config = UsdmConfig;
    /// let rate_limiter = Arc::new(RateLimiter::new(config.rate_limits()));
    /// let client = UsdmClient::new(credentials, http_client, rate_limiter);
    /// ```
    pub fn new(
        credentials: Credentials,
        http_client: Arc<dyn HttpClient>,
        rate_limiter: Arc<dyn BinanceRateLimiter>,
    ) -> Self {
        let config = UsdmConfig;

        let private_client = PrivateBinanceClient::new(
            Cow::Owned(config.base_url().to_string()),
            http_client,
            rate_limiter,
            Box::new(credentials.api_key.clone()),
            Box::new(credentials.api_secret.clone()),
        );

        UsdmPrivateRestClient(private_client)
    }
    /// Send a signed GET request with usdm-specific response type (high-performance)
    pub async fn send_get_signed_request<T, R>(
        &self,
        endpoint: &str,
        params: R,
        weight: u32,
        is_order: bool,
    ) -> Result<RestResponse<T>, Errors>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
        R: Serialize,
    {
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
            SharedErrors::InvalidApiKey => Errors::InvalidApiKey(),
            SharedErrors::Http { message: err } => Errors::HttpError(err),
            SharedErrors::Serialize { message: msg } => {
                Errors::Error(format!("Serialization error: {}", msg))
            }
            SharedErrors::Deserialize { message: msg } => {
                Errors::Error(format!("Deserialization error: {}", msg))
            }
            SharedErrors::Generic { message: msg } => Errors::Error(msg),
        })?;

        // Return the shared RestResponse directly
        Ok(shared_response)
    }

    /// Send a signed POST request with usdm-specific response type (high-performance)
    pub async fn send_post_signed_request<T, R>(
        &self,
        endpoint: &str,
        params: R,
        weight: u32,
        is_order: bool,
    ) -> Result<RestResponse<T>, Errors>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
        R: Serialize,
    {
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
            SharedErrors::InvalidApiKey => Errors::InvalidApiKey(),
            SharedErrors::Http { message: err } => Errors::HttpError(err),
            SharedErrors::Serialize { message: msg } => {
                Errors::Error(format!("Serialization error: {}", msg))
            }
            SharedErrors::Deserialize { message: msg } => {
                Errors::Error(format!("Deserialization error: {}", msg))
            }
            SharedErrors::Generic { message: msg } => Errors::Error(msg),
        })?;

        // Return the shared RestResponse directly
        Ok(shared_response)
    }

    /// Send a signed PUT request with usdm-specific response type (high-performance)
    pub async fn send_put_signed_request<T, R>(
        &self,
        endpoint: &str,
        params: R,
        weight: u32,
        is_order: bool,
    ) -> Result<RestResponse<T>, Errors>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
        R: Serialize,
    {
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
            SharedErrors::InvalidApiKey => Errors::InvalidApiKey(),
            SharedErrors::Http { message: err } => Errors::HttpError(err),
            SharedErrors::Serialize { message: msg } => {
                Errors::Error(format!("Serialization error: {}", msg))
            }
            SharedErrors::Deserialize { message: msg } => {
                Errors::Error(format!("Deserialization error: {}", msg))
            }
            SharedErrors::Generic { message: msg } => Errors::Error(msg),
        })?;

        // Return the shared RestResponse directly
        Ok(shared_response)
    }

    /// Send a signed DELETE request with usdm-specific response type (high-performance)
    pub async fn send_delete_signed_request<T, R>(
        &self,
        endpoint: &str,
        params: R,
        weight: u32,
        is_order: bool,
    ) -> Result<RestResponse<T>, Errors>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
        R: Serialize,
    {
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
                SharedErrors::InvalidApiKey => Errors::InvalidApiKey(),
                SharedErrors::Http { message: err } => Errors::HttpError(err),
                SharedErrors::Serialize { message: msg } => {
                    Errors::Error(format!("Serialization error: {}", msg))
                }
                SharedErrors::Deserialize { message: msg } => {
                    Errors::Error(format!("Deserialization error: {}", msg))
                }
                SharedErrors::Generic { message: msg } => Errors::Error(msg),
            })?;

        // Return the shared RestResponse directly
        Ok(shared_response)
    }

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
    ) -> Result<RestResponse<T>, Errors>
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
