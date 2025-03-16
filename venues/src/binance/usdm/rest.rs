use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

const REST_BASE_URL: &str = "https://fapi.binance.com";

#[derive(Debug, Clone)]
pub struct BinanceUsdMPublicRest {
    client: Client,
}

#[derive(Debug, Deserialize)]
pub struct OrderBookSnapshot {
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: u64,
    pub bids: Vec<(String, String)>,
    pub asks: Vec<(String, String)>,
}

impl BinanceUsdMPublicRest {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn get_orderbook_snapshot(
        &self,
        symbol: &str,
        limit: Option<u32>,
    ) -> Result<OrderBookSnapshot, Box<dyn Error>> {
        let limit = limit.unwrap_or(100);
        let url = format!(
            "{}/fapi/v1/depth?symbol={}&limit={}",
            REST_BASE_URL, symbol, limit
        );

        let response = self.client.get(&url).send().await?;
        let snapshot: OrderBookSnapshot = response.json().await?;
        Ok(snapshot)
    }
}

impl Default for BinanceUsdMPublicRest {
    fn default() -> Self {
        Self::new()
    }
} 