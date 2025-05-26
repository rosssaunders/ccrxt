use super::types::OrderBookSnapshot;
use reqwest::Client;
use std::error::Error;

const OKX_REST_URL: &str = "https://www.okx.com";

pub struct OkxPublicRest {
    client: Client,
}

impl OkxPublicRest {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn get_orderbook_snapshot(
        &self,
        inst_id: &str,
        depth: Option<u32>,
    ) -> Result<OrderBookSnapshot, Box<dyn Error>> {
        let depth = depth.unwrap_or(400);
        let url = format!(
            "{}/api/v5/market/books?instId={}&sz={}",
            OKX_REST_URL, inst_id, depth
        );

        let response = self.client.get(&url).send().await?.error_for_status()?;

        let response_json: serde_json::Value = response.json().await?;

        // OKX returns data in a nested structure
        if let Some(data) = response_json["data"].as_array() {
            if let Some(first_book) = data.first() {
                let snapshot: OrderBookSnapshot = serde_json::from_value(first_book.clone())?;
                return Ok(snapshot);
            }
        }

        Err("Failed to parse orderbook data".into())
    }
}
