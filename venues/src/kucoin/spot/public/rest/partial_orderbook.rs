use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

const PARTIAL_ORDERBOOK_ENDPOINT: &str = "/api/v1/market/orderbook/level2_{level}";

/// Request for getting partial order book
#[derive(Debug, Clone, Serialize)]
pub struct GetPartOrderBookRequest {
    /// Trading symbol (e.g., "BTC-USDT")
    pub symbol: String,

    /// Depth level (20 or 100)
    #[serde(skip_serializing)]
    pub level: OrderBookLevel,
}

/// Query parameters for partial order book request (excludes level which goes in path)
#[derive(Debug, Clone, Serialize)]
struct PartOrderBookQueryParams {
    /// Trading symbol (e.g., "BTC-USDT")
    symbol: String,
}

/// Order book depth levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum OrderBookLevel {
    #[serde(rename = "20")]
    Twenty,
    #[serde(rename = "100")]
    OneHundred,
}

/// Partial order book response  
#[derive(Debug, Clone, Deserialize)]
pub struct PartOrderBookResponse {
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
    /// Get partial order book (20 or 100 levels)
    ///
    /// Reference: https://docs.kucoin.com/#get-part-order-book-aggregated
    pub async fn get_part_order_book(
        &self,
        request: GetPartOrderBookRequest,
    ) -> Result<(PartOrderBookResponse, ResponseHeaders)> {
        let level_str = match request.level {
            OrderBookLevel::Twenty => "20",
            OrderBookLevel::OneHundred => "100",
        };

        let endpoint = PARTIAL_ORDERBOOK_ENDPOINT.replace("{level}", level_str);

        let query_params = PartOrderBookQueryParams {
            symbol: request.symbol,
        };

        let (response, headers): (RestResponse<PartOrderBookResponse>, ResponseHeaders) =
            self.get_with_request(&endpoint, &query_params).await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_order_book_request_creation() {
        let request = GetPartOrderBookRequest {
            symbol: "BTC-USDT".to_string(),
            level: OrderBookLevel::Twenty,
        };
        assert_eq!(request.symbol, "BTC-USDT");
        assert_eq!(request.level, OrderBookLevel::Twenty);
    }
}
