//! Public trades endpoint for Bullish Exchange API

use serde::Deserialize;

use super::client::RestClient;
use crate::bullish::{EndpointType, RestResult, enums::OrderSide};

/// Endpoint URL path for public trades
const PUBLIC_TRADES_ENDPOINT: &str = "/trading-api/v1/markets/{}/trades";

/// Public trade execution
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicTrade {
    /// Unique trade ID
    #[serde(rename = "tradeId")]
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
    #[serde(rename = "isTaker")]
    pub is_taker: bool,
    /// Trade creation timestamp
    #[serde(rename = "createdAtTimestamp")]
    pub timestamp: String,
    /// Trade creation datetime
    #[serde(rename = "createdAtDatetime")]
    pub datetime: String,
    /// Published timestamp
    #[serde(rename = "publishedAtTimestamp")]
    pub published_timestamp: String,
}

/// Parameters for querying public trades
#[derive(Debug, Clone, Default)]
pub struct PublicTradesParams {
    /// Start time filter (timestamp)
    pub start_time: Option<u64>,
    /// End time filter (timestamp)
    pub end_time: Option<u64>,
    /// Number of trades to return (default: 100, max: 1000)
    pub limit: Option<u32>,
}

impl RestClient {
    /// Get recent public trades for a market
    ///
    /// Retrieve a list of recent public trades for a specific market.
    ///
    /// # Arguments
    /// * `symbol` - Market symbol
    /// * `params` - Optional parameters for filtering trades
    ///
    /// # Returns
    /// List of recent public trades
    ///
    /// https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#get-/v1/markets/-symbol-/trades
    pub async fn get_public_trades(
        &self,
        symbol: &str,
        params: Option<PublicTradesParams>,
    ) -> RestResult<Vec<PublicTrade>> {
        let mut url = PUBLIC_TRADES_ENDPOINT.replace("{}", symbol);

        if let Some(params) = params {
            let mut query_params = Vec::new();

            if let Some(start_time) = params.start_time {
                query_params.push(format!("startTime={}", start_time));
            }
            if let Some(end_time) = params.end_time {
                query_params.push(format!("endTime={}", end_time));
            }
            if let Some(limit) = params.limit {
                query_params.push(format!("limit={}", limit));
            }

            if !query_params.is_empty() {
                url.push('?');
                url.push_str(&query_params.join("&"));
            }
        }

        self.send_request(
            &url,
            reqwest::Method::GET,
            None::<&()>,
            EndpointType::PublicTrades,
        )
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
        assert_eq!(trade.timestamp, "1640995200000");
        assert_eq!(trade.is_taker, true);
    }

    #[test]
    fn test_public_trades_params_default() {
        let params = PublicTradesParams::default();
        assert!(params.start_time.is_none());
        assert!(params.end_time.is_none());
        assert!(params.limit.is_none());
    }
}
