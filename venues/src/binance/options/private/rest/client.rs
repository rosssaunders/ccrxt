use std::time::Instant;

use serde::Serialize;

use crate::binance::{
    options::{Errors, RestResponse, RestResult},
    shared::{Errors as SharedErrors, client::PrivateBinanceClient},
};

pub struct OptionsPrivateRestClient(PrivateBinanceClient);

pub type RestClient = OptionsPrivateRestClient;

impl From<PrivateBinanceClient> for OptionsPrivateRestClient {
    fn from(client: PrivateBinanceClient) -> Self {
        OptionsPrivateRestClient(client)
    }
}

impl OptionsPrivateRestClient {
    /// Send a signed GET request with options-specific response type (high-performance)
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

        // Convert shared RestResponse to options RestResponse
        Ok(RestResponse {
            data: shared_response.data,
            request_duration: start.elapsed(),
            headers: crate::binance::options::ResponseHeaders {
                values: std::collections::HashMap::new(), // TODO: Convert headers properly
            },
        })
    }

    /// Send a signed POST request with options-specific response type (high-performance)
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

        // Convert shared RestResponse to options RestResponse
        Ok(RestResponse {
            data: shared_response.data,
            request_duration: start.elapsed(),
            headers: crate::binance::options::ResponseHeaders {
                values: std::collections::HashMap::new(), // TODO: Convert headers properly
            },
        })
    }

    /// Send a signed PUT request with options-specific response type (high-performance)
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

        // Convert shared RestResponse to options RestResponse
        Ok(RestResponse {
            data: shared_response.data,
            request_duration: start.elapsed(),
            headers: crate::binance::options::ResponseHeaders {
                values: std::collections::HashMap::new(), // TODO: Convert headers properly
            },
        })
    }

    /// Send a signed DELETE request with options-specific response type (high-performance)
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

        // Convert shared RestResponse to options RestResponse
        Ok(RestResponse {
            data: shared_response.data,
            request_duration: start.elapsed(),
            headers: crate::binance::options::ResponseHeaders {
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
            rest::http_client::Method::Get => self.send_get_signed_request(endpoint, params, weight, is_order).await,
            rest::http_client::Method::Post => self.send_post_signed_request(endpoint, params, weight, is_order).await,
            rest::http_client::Method::Put => self.send_put_signed_request(endpoint, params, weight, is_order).await,
            rest::http_client::Method::Delete => self.send_delete_signed_request(endpoint, params, weight, is_order).await,
            _ => Err(Errors::Error(format!("Unsupported HTTP method: {:?}", method))),
        }
    }
}
