//! BitMart Futures Private REST Client
use std::sync::Arc;

use reqwest::Client;
use secrecy::{ExposeSecret, SecretString};

use crate::bitmart::{Errors, RestResult};

#[derive(Debug, Clone)]
pub struct RestClient {
    pub api_key: SecretString,
    pub api_secret: SecretString,
    pub client: Arc<Client>,
    pub base_url: String,
}

impl RestClient {
    pub fn new(api_key: impl Into<SecretString>, api_secret: impl Into<SecretString>) -> Self {
        Self {
            api_key: api_key.into(),
            api_secret: api_secret.into(),
            client: Arc::new(Client::new()),
            base_url: "https://api-cloud-v2.bitmart.com".to_string(),
        }
    }

    pub(super) async fn send_signed_request<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
    ) -> RestResult<T> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .client
            .get(&url)
            .header("X-BM-KEY", self.api_key.expose_secret().to_string())
            .send()
            .await
            .map_err(Errors::HttpError)?;
        let status = resp.status();
        let text = resp.text().await.map_err(Errors::HttpError)?;
        if !status.is_success() {
            let err: crate::bitmart::error::ErrorResponse = serde_json::from_str(&text)
                .unwrap_or_else(|_| crate::bitmart::error::ErrorResponse {
                    code: status.as_u16() as i32,
                    message: text.into(),
                });
            return Err(Errors::Error(
                crate::bitmart::error::BitmartError::from(err).to_string(),
            ));
        }
        serde_json::from_str(&text)
            .map_err(|e| Errors::Error(format!("JSON serialization error: {e}")))
    }
}

// Convert BitmartError to Errors for RestResult
fn bitmart_error_to_errors(e: crate::bitmart::error::BitmartError) -> Errors {
    Errors::Error(e.to_string())
}
