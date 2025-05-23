use reqwest::Client;
use std::error::Error;
use super::types::OrderBookSnapshot;
use super::rate_limit::BinanceSpotRateLimiter;

const BINANCE_SPOT_API_URL: &str = "https://api.binance.com/api/v3";

pub struct BinanceSpotPublicRest {
    client: Client,
    base_url: String,
    pub(crate) rate_limiter: BinanceSpotRateLimiter,
}

impl BinanceSpotPublicRest {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: BINANCE_SPOT_API_URL.to_string(),
            rate_limiter: BinanceSpotRateLimiter::new(),
        }
    }

    pub async fn get_orderbook_snapshot(&self, symbol: &str, limit: Option<u32>) -> Result<OrderBookSnapshot, Box<dyn Error>> {
        let limit = limit.unwrap_or(100);
        let url = format!("{}/depth?symbol={}&limit={}", self.base_url, symbol, limit);
        
        // Check rate limits before making the request
        if let Err(e) = self.rate_limiter.check_weight_limit("depth", 1).await {
            return Err(e);
        }
        
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