use reqwest::Client;
use std::error::Error;
use super::types::OrderBookSnapshot;

const BINANCE_COINM_REST_URL: &str = "https://dapi.binance.com";

pub struct BinanceCoinMPublicRest {
    client: Client,
}

impl BinanceCoinMPublicRest {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn get_orderbook_snapshot(&self, symbol: &str, limit: Option<u32>) -> Result<OrderBookSnapshot, Box<dyn Error>> {
        let limit = limit.unwrap_or(1000);
        let url = format!("{}/dapi/v1/depth?symbol={}&limit={}", BINANCE_COINM_REST_URL, symbol, limit);
        
        let response = self.client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;
            
        let snapshot: OrderBookSnapshot = response.json().await?;
        Ok(snapshot)
    }
} 