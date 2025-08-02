use serde::Serialize;

use crate::binance::{
    shared::{Errors as SharedErrors, client::PrivateBinanceClient},
    spot::{Errors, RestResponse, RestResult},
};

pub struct SpotPrivateRestClient(PrivateBinanceClient);

pub type RestClient = SpotPrivateRestClient;

impl From<PrivateBinanceClient> for SpotPrivateRestClient {
    fn from(client: PrivateBinanceClient) -> Self {
        SpotPrivateRestClient(client)
    }
}

impl SpotPrivateRestClient {
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
        method: reqwest::Method,
        params: R,
        weight: u32,
        is_order: bool,
    ) -> RestResult<T>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
        R: Serialize,
    {
        // Call the shared client's send_signed_request
        #[allow(deprecated)]
        let shared_response = PrivateBinanceClient::send_signed_request::<T, R, SharedErrors>(
            &self.0, endpoint, method, params, weight, is_order,
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
}
