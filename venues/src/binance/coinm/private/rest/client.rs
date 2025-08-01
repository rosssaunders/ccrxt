use crate::binance::{
    coinm::{Errors, RestResponse, RestResult, ResponseHeaders},
    shared::{client::PrivateBinanceClient, Errors as SharedErrors},
};
use serde::Serialize;
use std::time::Instant;

pub struct CoinmRestClient(PrivateBinanceClient);

pub type RestClient = CoinmRestClient;


impl From<PrivateBinanceClient> for CoinmRestClient {
    fn from(client: PrivateBinanceClient) -> Self {
        CoinmRestClient(client)
    }
}


impl CoinmRestClient {
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
            &self.0,
            endpoint,
            params,
            weight,
            is_order
        )
        .await
        .map_err(|e| match e {
            SharedErrors::ApiError(_) => Errors::Error("API error occurred".to_string()),
            SharedErrors::RateLimitExceeded { retry_after } => {
                Errors::Error(format!("Rate limit exceeded, retry after {:?}", retry_after))
            },
            SharedErrors::HttpError(msg) => Errors::Error(format!("HTTP error: {msg}")),
            SharedErrors::InvalidApiKey() => Errors::Error("Invalid API key".to_string()),
            SharedErrors::SerializationError(msg) => Errors::Error(format!("Serialization error: {msg}")),
            SharedErrors::Error(msg) => Errors::Error(msg),
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
            &self.0,
            endpoint,
            params,
            weight,
            is_order
        )
        .await
        .map_err(|e| match e {
            SharedErrors::ApiError(_) => Errors::Error("API error occurred".to_string()),
            SharedErrors::RateLimitExceeded { retry_after } => {
                Errors::Error(format!("Rate limit exceeded, retry after {:?}", retry_after))
            },
            SharedErrors::HttpError(msg) => Errors::Error(format!("HTTP error: {msg}")),
            SharedErrors::InvalidApiKey() => Errors::Error("Invalid API key".to_string()),
            SharedErrors::SerializationError(msg) => Errors::Error(format!("Serialization error: {msg}")),
            SharedErrors::Error(msg) => Errors::Error(msg),
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
            &self.0,
            endpoint,
            params,
            weight,
            is_order
        )
        .await
        .map_err(|e| match e {
            SharedErrors::ApiError(_) => Errors::Error("API error occurred".to_string()),
            SharedErrors::RateLimitExceeded { retry_after } => {
                Errors::Error(format!("Rate limit exceeded, retry after {:?}", retry_after))
            },
            SharedErrors::HttpError(msg) => Errors::Error(format!("HTTP error: {msg}")),
            SharedErrors::InvalidApiKey() => Errors::Error("Invalid API key".to_string()),
            SharedErrors::SerializationError(msg) => Errors::Error(format!("Serialization error: {msg}")),
            SharedErrors::Error(msg) => Errors::Error(msg),
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
        let shared_response = PrivateBinanceClient::send_delete_signed_request::<T, R, SharedErrors>(
            &self.0,
            endpoint,
            params,
            weight,
            is_order
        )
        .await
        .map_err(|e| match e {
            SharedErrors::ApiError(_) => Errors::Error("API error occurred".to_string()),
            SharedErrors::RateLimitExceeded { retry_after } => {
                Errors::Error(format!("Rate limit exceeded, retry after {:?}", retry_after))
            },
            SharedErrors::HttpError(msg) => Errors::Error(format!("HTTP error: {msg}")),
            SharedErrors::InvalidApiKey() => Errors::Error("Invalid API key".to_string()),
            SharedErrors::SerializationError(msg) => Errors::Error(format!("Serialization error: {msg}")),
            SharedErrors::Error(msg) => Errors::Error(msg),
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
    #[deprecated(note = "Use verb-specific functions (send_get_signed_request, send_post_signed_request, etc.) for better performance")]
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
        let start = Instant::now();
        
        // Call the shared client's send_signed_request
        #[allow(deprecated)]
        let shared_response = PrivateBinanceClient::send_signed_request::<T, R, SharedErrors>(
            &self.0,
            endpoint,
            method,
            params,
            weight,
            is_order
        )
        .await
        .map_err(|e| match e {
            SharedErrors::ApiError(_) => Errors::Error("API error occurred".to_string()),
            SharedErrors::RateLimitExceeded { retry_after } => {
                Errors::Error(format!("Rate limit exceeded, retry after {:?}", retry_after))
            },
            SharedErrors::HttpError(msg) => Errors::Error(format!("HTTP error: {msg}")),
            SharedErrors::InvalidApiKey() => Errors::Error("Invalid API key".to_string()),
            SharedErrors::SerializationError(msg) => Errors::Error(format!("Serialization error: {msg}")),
            SharedErrors::Error(msg) => Errors::Error(msg),
        })?;

        let duration = start.elapsed();
        tracing::debug!("Request to {endpoint} took {duration:?}");

        Ok(RestResponse {
            data: shared_response.data,
            request_duration: duration,
            headers: ResponseHeaders::default(), // TODO: Convert headers properly
        })
    }
}
