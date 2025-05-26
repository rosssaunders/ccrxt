use super::types::OrderBookSnapshot;
use reqwest::Client;
use std::error::Error;

const BYBIT_PERP_REST_URL: &str = "https://api.bybit.com";

pub struct BybitPerpPublicRest {
    client: Client,
    base_url: String,
}

impl BybitPerpPublicRest {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: BYBIT_PERP_REST_URL.to_string(),
        }
    }

    pub async fn get_orderbook_snapshot(
        &self,
        symbol: &str,
        limit: Option<u32>,
    ) -> Result<OrderBookSnapshot, Box<dyn Error>> {
        let limit = limit.unwrap_or(100);
        let url = format!(
            "{}/v5/market/orderbook?category=linear&symbol={}&limit={}",
            self.base_url, symbol, limit
        );

        let response = self.client.get(&url).send().await?;
        let snapshot: OrderBookSnapshot = response.json().await?;

        if snapshot.ret_code != 0 {
            return Err(format!("Bybit API error: {}", snapshot.ret_msg).into());
        }

        Ok(snapshot)
    }
}

impl Default for BybitPerpPublicRest {
    fn default() -> Self {
        Self::new()
    }
}
