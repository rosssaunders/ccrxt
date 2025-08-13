//! BitMart Futures Private REST Client
use std::sync::Arc;

use rest::{
    HttpClient,
    http_client::{Method as HttpMethod, RequestBuilder},
    secrets::ExposableSecret,
};

use crate::bitmart::{Errors, RestResult, shared::Credentials};

#[derive(Clone)]
pub struct RestClient {
    pub credentials: Credentials,
    pub http_client: Arc<dyn HttpClient>,
    pub base_url: String,
}

impl RestClient {
    pub fn new(
        credentials: Credentials,
        http_client: Arc<dyn HttpClient>,
    ) -> Self {
        Self {
            credentials,
            http_client,
            base_url: "https://api-cloud-v2.bitmart.com".to_string(),
        }
    }

    pub(super) async fn send_signed_request<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
    ) -> RestResult<T> {
        let url = format!("{}{}", self.base_url, path);
        let request = RequestBuilder::new(HttpMethod::Get, url)
            .header("X-BM-KEY", self.credentials.api_key.expose_secret())
            .build();

        let response = self
            .http_client
            .execute(request)
            .await
            .map_err(|e| Errors::NetworkError(format!("HTTP request failed: {e}")))?;

        let status = response.status;
        let text = response
            .text()
            .map_err(|e| Errors::NetworkError(format!("Failed to read response: {e}")))?;

        if status != 200 && status != 201 {
            let err: crate::bitmart::spot::error::ErrorResponse = serde_json::from_str(&text)
                .unwrap_or_else(|_| crate::bitmart::spot::error::ErrorResponse {
                    code: status as i32,
                    message: text.into(),
                });
            return Err(Errors::Error(
                crate::bitmart::spot::error::BitmartError::from(err).to_string(),
            ));
        }
        serde_json::from_str(&text)
            .map_err(|e| Errors::Error(format!("JSON serialization error: {e}")))
    }
}
