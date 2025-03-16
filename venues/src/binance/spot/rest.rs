use reqwest::Client;
use std::error::Error;
use super::types::OrderBookSnapshot;

const BINANCE_SPOT_API_URL: &str = "https://api.binance.com/api/v3";

pub struct BinanceSpotPublicRest {
    client: Client,
    base_url: String,
}

impl BinanceSpotPublicRest {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: BINANCE_SPOT_API_URL.to_string(),
        }
    }

    pub async fn get_orderbook_snapshot(&self, symbol: &str, limit: Option<u32>) -> Result<OrderBookSnapshot, Box<dyn Error>> {
        let limit = limit.unwrap_or(100);
        let url = format!("{}/depth?symbol={}&limit={}", self.base_url, symbol, limit);
        
        let response = self.client.get(&url).send().await?;
        let snapshot: OrderBookSnapshot = response.json().await?;
        
        Ok(snapshot)
    }
}

impl Default for BinanceSpotPublicRest {
    fn default() -> Self {
        Self::new()
    }
} 