//! BitMart Futures Public REST Client
use std::sync::Arc;

use rest::{
    HttpClient,
    http_client::{Method as HttpMethod, RequestBuilder},
};

use crate::bitmart::spot::error::Result;

#[derive(Clone)]
pub struct RestClient {
    // For public endpoints, credentials are not required
    pub http_client: Arc<dyn HttpClient>,
    pub base_url: String,
}

impl RestClient {
    pub fn new(http_client: Arc<dyn HttpClient>) -> Self {
        Self {
            http_client,
            base_url: "https://api-cloud-v2.bitmart.com".to_string(),
        }
    }

    pub async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let request = RequestBuilder::new(HttpMethod::Get, url).build();
        
        let response = self.http_client.execute(request).await
            .map_err(|e| crate::bitmart::spot::error::BitmartError::Other(0, format!("HTTP request failed: {e}").into()))?;
        
        let status = response.status;
        let text = response.text()
            .map_err(|e| crate::bitmart::spot::error::BitmartError::Other(0, format!("Failed to read response: {e}").into()))?;
        
        if status != 200 && status != 201 {
            let err: crate::bitmart::spot::error::ErrorResponse = serde_json::from_str(&text)
                .unwrap_or_else(|_| crate::bitmart::spot::error::ErrorResponse {
                    code: status as i32,
                    message: text.into(),
                });
            return Err(crate::bitmart::spot::error::BitmartError::from(err));
        }
        serde_json::from_str(&text)
            .map_err(|e| crate::bitmart::spot::error::BitmartError::Other(0, e.to_string().into()))
    }
}
