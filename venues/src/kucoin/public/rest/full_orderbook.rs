use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::kucoin::{ResponseHeaders, RestResponse, Result};

use super::RestClient;

/// Request for getting full order book
#[derive(Debug, Clone, Serialize)]
pub struct GetFullOrderBookRequest {
    /// Trading symbol (e.g., "BTC-USDT")
    pub symbol: String,
}

/// Full order book response
#[derive(Debug, Clone, Deserialize)]
pub struct FullOrderBookResponse {
    /// Server timestamp
    pub time: i64,

    /// Sequence number
    pub sequence: String,

    /// Bid orders (buy orders)
    pub bids: Vec<[String; 2]>, // [price, size]

    /// Ask orders (sell orders)
    pub asks: Vec<[String; 2]>, // [price, size]
}

impl RestClient {
    /// Get full order book (all levels)
    ///
    /// Reference: https://docs.kucoin.com/#get-full-order-book-aggregated
    pub async fn get_full_order_book(
        &self,
        request: GetFullOrderBookRequest,
    ) -> Result<(FullOrderBookResponse, ResponseHeaders)> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), request.symbol);

        let (response, headers): (RestResponse<FullOrderBookResponse>, ResponseHeaders) =
            self.get("/api/v1/market/orderbook/level2", Some(params)).await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_order_book_request_creation() {
        let request = GetFullOrderBookRequest {
            symbol: "BTC-USDT".to_string(),
        };
        assert_eq!(request.symbol, "BTC-USDT");
    }
}
