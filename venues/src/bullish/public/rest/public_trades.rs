//! Public trades endpoint for Bullish Exchange API

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bullish::{EndpointType, RestResult, enums::OrderSide};

/// Endpoint URL path for public trades
const PUBLIC_TRADES_ENDPOINT: &str = "/trading-api/v1/markets/{}/trades";

/// Public trade execution
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicTrade {
    /// Unique trade ID
    pub trade_id: String,

    /// Market symbol
    pub symbol: String,

    /// Trade price
    pub price: String,

    /// Trade quantity
    pub quantity: String,

    /// Trade side (from the taker's perspective)
    pub side: OrderSide,

    /// Whether this is a taker trade
    pub is_taker: bool,

    /// Trade creation timestamp
    pub created_at_timestamp: String,

    /// Trade creation datetime
    pub created_at_datetime: String,

    /// Published timestamp
    pub published_at_timestamp: String,
}

/// Request parameters for querying public trades
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PublicTradesRequest {
    /// Start time filter (timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time filter (timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Number of trades to return (default: 100, max: 1000)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl RestClient {
    /// Get recent public trades for a market
    ///
    /// Retrieve a list of recent public trades for a specific market.
    ///
    /// [docs]: https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#get-/v1/markets/-symbol-/trades
    ///
    /// # Arguments
    /// * `symbol` - Market symbol
    /// * `request` - Optional parameters for filtering trades
    ///
    /// # Returns
    /// List of recent public trades
    pub async fn get_public_trades(
        &self,
        symbol: &str,
        request: Option<PublicTradesRequest>,
    ) -> RestResult<Vec<PublicTrade>> {
        let mut url = PUBLIC_TRADES_ENDPOINT.replace("{}", symbol);

        if let Some(request) = request {
            let query_string = serde_urlencoded::to_string(&request).map_err(|e| {
                crate::bullish::Errors::Error(format!(
                    "Failed to serialize query parameters: {}",
                    e
                ))
            })?;

            if !query_string.is_empty() {
                url.push('?');
                url.push_str(&query_string);
            }
        }

        self.send_get_request(&url, EndpointType::PublicTrades)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_trade_deserialization() {
        let json = r#"{
            "tradeId": "123456789",
            "symbol": "BTCUSDC",
            "price": "30000.0",
            "quantity": "1.0",
            "side": "BUY",
            "isTaker": true,
            "createdAtTimestamp": "1640995200000",
            "createdAtDatetime": "2022-01-01T00:00:00Z",
            "publishedAtTimestamp": "1640995200100"
        }"#;

        let trade: PublicTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.trade_id, "123456789");
        assert_eq!(trade.symbol, "BTCUSDC");
        assert_eq!(trade.price, "30000.0");
        assert_eq!(trade.side, OrderSide::Buy);
        assert_eq!(trade.created_at_timestamp, "1640995200000");
        assert!(trade.is_taker);
    }

    #[test]
    fn test_public_trades_request_default() {
        let request = PublicTradesRequest::default();
        assert!(request.start_time.is_none());
        assert!(request.end_time.is_none());
        assert!(request.limit.is_none());
    }
}
