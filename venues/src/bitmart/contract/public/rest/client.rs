//! BitMart Futures Public REST Client
use crate::bitmart::spot::error::Result;
use reqwest::Client;
use std::sync::Arc;

#[derive(Debug, Clone, Default)]
pub struct RestClient {
    // For public endpoints, credentials are not required
    pub client: Arc<Client>,
    pub base_url: String,
}

impl RestClient {
    pub fn default() -> Self {
        Self {
            client: Arc::new(Client::new()),
            base_url: "https://api-cloud-v2.bitmart.com".to_string(),
        }
    }

    pub async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| crate::bitmart::error::BitmartError::Other(0, e.to_string().into()))?;
        let status = resp.status();
        let text = resp
            .text()
            .await
            .map_err(|e| crate::bitmart::error::BitmartError::Other(0, e.to_string().into()))?;
        if !status.is_success() {
            let err: crate::bitmart::error::ErrorResponse = serde_json::from_str(&text)
                .unwrap_or_else(|_| crate::bitmart::error::ErrorResponse {
                    code: status.as_u16() as i32,
                    message: text.into(),
                });
            return Err(crate::bitmart::error::BitmartError::from(err));
        }
        serde_json::from_str(&text)
            .map_err(|e| crate::bitmart::error::BitmartError::Other(0, e.to_string().into()))
    }
}
