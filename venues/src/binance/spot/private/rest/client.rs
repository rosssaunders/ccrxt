use std::{borrow::Cow, sync::Arc};

use rest::{HttpClient, http_client::Method as HttpMethod};
use serde::Serialize;

use crate::binance::{
    shared::{
        Errors as SharedErrors, client::PrivateBinanceClient, credentials::Credentials,
        rate_limiter_trait::BinanceRateLimiter, venue_trait::VenueConfig,
    },
    spot::{Errors, RestResponse, RestResult, SpotConfig},
};

pub struct SpotPrivateRestClient(PrivateBinanceClient);

pub type RestClient = SpotPrivateRestClient;

impl From<PrivateBinanceClient> for SpotPrivateRestClient {
    fn from(client: PrivateBinanceClient) -> Self {
        SpotPrivateRestClient(client)
    }
}

impl SpotPrivateRestClient {
    /// Create a new SpotPrivateRestClient with credentials, HTTP client, and rate limiter
    ///
    /// Creates a new private REST client for Binance Spot using the provided credentials,
    /// HTTP client implementation, and rate limiter.
    ///
    /// # Arguments
    /// * `credentials` - API credentials containing key and secret
    /// * `http_client` - HTTP client implementation to use for requests
    /// * `rate_limiter` - Rate limiter implementation for request throttling
    ///
    /// # Returns
    /// A new `SpotPrivateRestClient` instance configured for Spot trading
    ///
    /// # Example
    /// ```no_run
    /// use std::sync::Arc;
    /// use rest::{secrets::SecretString, HttpClient};
    /// use venues::binance::shared::{credentials::Credentials, rate_limiter::RateLimiter, venue_trait::VenueConfig};
    /// use venues::binance::spot::{private::rest::RestClient, SpotConfig};
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
    /// let config = SpotConfig;
    /// let rate_limiter = Box::new(RateLimiter::new(config.rate_limits()));
    /// let client = RestClient::new(credentials, http_client, rate_limiter);
    /// ```
    pub fn new(
        credentials: Credentials,
        http_client: Arc<dyn HttpClient>,
        rate_limiter: Arc<dyn BinanceRateLimiter>,
    ) -> Self {
        let config = SpotConfig;

        let private_client = PrivateBinanceClient::new(
            Cow::Owned(config.base_url().to_string()),
            http_client,
            rate_limiter,
            Box::new(credentials.api_key.clone()),
            Box::new(credentials.api_secret.clone()),
        );

        SpotPrivateRestClient(private_client)
    }

    /// Send a signed GET request with spot-specific response type (high-performance)
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
        // Call the shared client's high-performance GET function
        let shared_response = PrivateBinanceClient::send_get_signed_request::<T, R, SharedErrors>(
            &self.0, endpoint, params, weight, is_order,
        )
        .await
        .map_err(|e| match e {
            SharedErrors::ApiError(_) => Errors::Error("API error occurred".to_string()),
            SharedErrors::RateLimitExceeded { retry_after } => Errors::Error(format!(
                "Rate limit exceeded, retry after {:?}",
                retry_after
            )),
            SharedErrors::InvalidApiKey() => Errors::InvalidApiKey(),
            SharedErrors::HttpError(err) => Errors::HttpError(err),
            SharedErrors::SerializationError(msg) => {
                Errors::Error(format!("Serialization error: {}", msg))
            }
            SharedErrors::Error(msg) => Errors::Error(msg),
        })?;

        // Convert shared RestResponse to spot RestResponse
        Ok(RestResponse {
            data: shared_response.data,
            headers: crate::binance::spot::ResponseHeaders {
                values: std::collections::HashMap::new(), // TODO: Convert headers properly
            },
        })
    }

    /// Send a signed POST request with spot-specific response type (high-performance)
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
        // Call the shared client's high-performance POST function
        let shared_response = PrivateBinanceClient::send_post_signed_request::<T, R, SharedErrors>(
            &self.0, endpoint, params, weight, is_order,
        )
        .await
        .map_err(|e| match e {
            SharedErrors::ApiError(_) => Errors::Error("API error occurred".to_string()),
            SharedErrors::RateLimitExceeded { retry_after } => Errors::Error(format!(
                "Rate limit exceeded, retry after {:?}",
                retry_after
            )),
            SharedErrors::InvalidApiKey() => Errors::InvalidApiKey(),
            SharedErrors::HttpError(err) => Errors::HttpError(err),
            SharedErrors::SerializationError(msg) => {
                Errors::Error(format!("Serialization error: {}", msg))
            }
            SharedErrors::Error(msg) => Errors::Error(msg),
        })?;

        // Convert shared RestResponse to spot RestResponse
        Ok(RestResponse {
            data: shared_response.data,
            headers: crate::binance::spot::ResponseHeaders {
                values: std::collections::HashMap::new(), // TODO: Convert headers properly
            },
        })
    }

    /// Send a signed PUT request with spot-specific response type (high-performance)
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
        // Call the shared client's high-performance PUT function
        let shared_response = PrivateBinanceClient::send_put_signed_request::<T, R, SharedErrors>(
            &self.0, endpoint, params, weight, is_order,
        )
        .await
        .map_err(|e| match e {
            SharedErrors::ApiError(_) => Errors::Error("API error occurred".to_string()),
            SharedErrors::RateLimitExceeded { retry_after } => Errors::Error(format!(
                "Rate limit exceeded, retry after {:?}",
                retry_after
            )),
            SharedErrors::InvalidApiKey() => Errors::InvalidApiKey(),
            SharedErrors::HttpError(err) => Errors::HttpError(err),
            SharedErrors::SerializationError(msg) => {
                Errors::Error(format!("Serialization error: {}", msg))
            }
            SharedErrors::Error(msg) => Errors::Error(msg),
        })?;

        // Convert shared RestResponse to spot RestResponse
        Ok(RestResponse {
            data: shared_response.data,
            headers: crate::binance::spot::ResponseHeaders {
                values: std::collections::HashMap::new(), // TODO: Convert headers properly
            },
        })
    }

    /// Send a signed DELETE request with spot-specific response type (high-performance)
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
        // Call the shared client's high-performance DELETE function
        let shared_response =
            PrivateBinanceClient::send_delete_signed_request::<T, R, SharedErrors>(
                &self.0, endpoint, params, weight, is_order,
            )
            .await
            .map_err(|e| match e {
                SharedErrors::ApiError(_) => Errors::Error("API error occurred".to_string()),
                SharedErrors::RateLimitExceeded { retry_after } => Errors::Error(format!(
                    "Rate limit exceeded, retry after {:?}",
                    retry_after
                )),
                SharedErrors::InvalidApiKey() => Errors::InvalidApiKey(),
                SharedErrors::HttpError(err) => Errors::HttpError(err),
                SharedErrors::SerializationError(msg) => {
                    Errors::Error(format!("Serialization error: {}", msg))
                }
                SharedErrors::Error(msg) => Errors::Error(msg),
            })?;

        // Convert shared RestResponse to spot RestResponse
        Ok(RestResponse {
            data: shared_response.data,
            headers: crate::binance::spot::ResponseHeaders {
                values: std::collections::HashMap::new(), // TODO: Convert headers properly
            },
        })
    }

    /// Use send_get_signed_request, send_post_signed_request, etc. instead.
    #[deprecated(
        note = "Use verb-specific functions (send_get_signed_request, send_post_signed_request, etc.) for better performance"
    )]
    pub async fn send_signed_request<T, R>(
        &self,
        endpoint: &str,
        method: HttpMethod,
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
            HttpMethod::Get => {
                self.send_get_signed_request(endpoint, params, weight, is_order)
                    .await
            }
            HttpMethod::Post => {
                self.send_post_signed_request(endpoint, params, weight, is_order)
                    .await
            }
            HttpMethod::Put => {
                self.send_put_signed_request(endpoint, params, weight, is_order)
                    .await
            }
            HttpMethod::Delete => {
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
