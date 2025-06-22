use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bitmart::RestResult;
use crate::bitmart::rate_limit::EndpointType;

/// Request parameters for getting recent trades
#[derive(Debug, Serialize)]
pub struct GetRecentTradesRequest {
    /// Trading pair (e.g. BMX_USDT)
    pub symbol: String,
    /// Number of returned items, maximum is 50, default 50
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Trade data (array format)
/// [symbol, timestamp, price, size, side]
pub type TradeData = Vec<String>;

/// Response for recent trades endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRecentTradesResponse(pub Vec<TradeData>);

impl GetRecentTradesResponse {
    /// Get the trading pair from trade data
    pub fn symbol(trade: &TradeData) -> Option<&str> {
        trade.get(0).map(|s| s.as_str())
    }

    /// Get the trade time (in milliseconds) from trade data
    pub fn timestamp(trade: &TradeData) -> Option<&str> {
        trade.get(1).map(|s| s.as_str())
    }

    /// Get the trade price from trade data
    pub fn price(trade: &TradeData) -> Option<&str> {
        trade.get(2).map(|s| s.as_str())
    }

    /// Get the trade number from trade data
    pub fn size(trade: &TradeData) -> Option<&str> {
        trade.get(3).map(|s| s.as_str())
    }

    /// Get the order side from trade data
    /// - `buy`
    /// - `sell`
    pub fn side(trade: &TradeData) -> Option<&str> {
        trade.get(4).map(|s| s.as_str())
    }
}

impl RestClient {
    /// Get Recent Trades (V3)
    ///
    /// Get the latest trade records of the specified trading pair.
    /// Note that the interface is not real-time data, if you need real-time data,
    /// please use websocket to subscribe Trade channel
    ///
    /// See: https://raw.githubusercontent.com/rosssaunders/coincise/refs/heads/main/docs/bitmart/spot/public_market_data.md
    ///
    /// Rate limit: 10 times/2sec per IP
    ///
    /// # Arguments
    /// * `request` - The request parameters including symbol and optional limit
    ///
    /// # Returns
    /// Latest trade records for the specified trading pair
    pub async fn get_recent_trades(&self, request: GetRecentTradesRequest) -> RestResult<GetRecentTradesResponse> {
        self.send_request(
            "/spot/quotation/v3/trades",
            reqwest::Method::GET,
            Some(&request),
            EndpointType::SpotPublicMarket,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_recent_trades_request() {
        let request = GetRecentTradesRequest {
            symbol: "BMX_ETH".to_string(),
            limit: Some(10),
        };

        assert_eq!(request.symbol, "BMX_ETH");
        assert_eq!(request.limit, Some(10));
    }

    #[test]
    fn test_get_recent_trades_request_default_limit() {
        let request = GetRecentTradesRequest {
            symbol: "BTC_USDT".to_string(),
            limit: None,
        };

        assert_eq!(request.symbol, "BTC_USDT");
        assert_eq!(request.limit, None);
    }

    #[test]
    fn test_trade_data_parsing() {
        let trade_data = vec![
            "BMX_ETH".to_string(),       // symbol
            "1691743270994".to_string(), // ts
            "1.00000000".to_string(),    // price
            "1.0".to_string(),           // size
            "sell".to_string(),          // side
        ];

        assert_eq!(
            GetRecentTradesResponse::symbol(&trade_data),
            Some("BMX_ETH")
        );
        assert_eq!(
            GetRecentTradesResponse::timestamp(&trade_data),
            Some("1691743270994")
        );
        assert_eq!(
            GetRecentTradesResponse::price(&trade_data),
            Some("1.00000000")
        );
        assert_eq!(GetRecentTradesResponse::size(&trade_data), Some("1.0"));
        assert_eq!(GetRecentTradesResponse::side(&trade_data), Some("sell"));
    }

    #[test]
    fn test_trade_data_incomplete() {
        let trade_data = vec![
            "BMX_ETH".to_string(),
            "1691743270994".to_string(),
            "1.00000000".to_string(),
        ];

        assert_eq!(
            GetRecentTradesResponse::symbol(&trade_data),
            Some("BMX_ETH")
        );
        assert_eq!(
            GetRecentTradesResponse::timestamp(&trade_data),
            Some("1691743270994")
        );
        assert_eq!(
            GetRecentTradesResponse::price(&trade_data),
            Some("1.00000000")
        );
        assert_eq!(GetRecentTradesResponse::size(&trade_data), None);
        assert_eq!(GetRecentTradesResponse::side(&trade_data), None);
    }

    #[test]
    fn test_get_recent_trades_response_structure() {
        let response = GetRecentTradesResponse(vec![
            vec![
                "BMX_ETH".to_string(),
                "1691743270994".to_string(),
                "1.00000000".to_string(),
                "1.0".to_string(),
                "sell".to_string(),
            ],
            vec![
                "BTC_USDT".to_string(),
                "1691743271000".to_string(),
                "30000.00".to_string(),
                "0.1".to_string(),
                "buy".to_string(),
            ],
        ]);

        assert_eq!(response.0.len(), 2);
        assert_eq!(
            GetRecentTradesResponse::symbol(&response.0[0]),
            Some("BMX_ETH")
        );
        assert_eq!(GetRecentTradesResponse::side(&response.0[0]), Some("sell"));
        assert_eq!(
            GetRecentTradesResponse::symbol(&response.0[1]),
            Some("BTC_USDT")
        );
        assert_eq!(GetRecentTradesResponse::side(&response.0[1]), Some("buy"));
    }

    #[test]
    fn test_request_serialization() {
        let request = GetRecentTradesRequest {
            symbol: "BMX_ETH".to_string(),
            limit: Some(10),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BMX_ETH"));
        assert!(serialized.contains("limit=10"));
    }

    #[test]
    fn test_request_serialization_no_limit() {
        let request = GetRecentTradesRequest {
            symbol: "BMX_ETH".to_string(),
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BMX_ETH"));
        assert!(!serialized.contains("limit"));
    }

    #[test]
    fn test_response_json_parsing() {
        let json = r#"[
            [
                "BMX_ETH",
                "1691743270994",
                "1.00000000",
                "1.0",
                "sell"
            ]
        ]"#;

        let response: GetRecentTradesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.0.len(), 1);
        assert_eq!(
            GetRecentTradesResponse::symbol(&response.0[0]),
            Some("BMX_ETH")
        );
        assert_eq!(
            GetRecentTradesResponse::timestamp(&response.0[0]),
            Some("1691743270994")
        );
        assert_eq!(
            GetRecentTradesResponse::price(&response.0[0]),
            Some("1.00000000")
        );
        assert_eq!(GetRecentTradesResponse::size(&response.0[0]), Some("1.0"));
        assert_eq!(GetRecentTradesResponse::side(&response.0[0]), Some("sell"));
    }

    #[test]
    fn test_empty_response() {
        let json = r#"[]"#;

        let response: GetRecentTradesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.0.len(), 0);
    }

    #[test]
    fn test_multiple_trades() {
        let json = r#"[
            [
                "BMX_ETH",
                "1691743270994",
                "1.00000000",
                "1.0",
                "sell"
            ],
            [
                "BMX_ETH",
                "1691743271000",
                "1.00000001",
                "2.5",
                "buy"
            ]
        ]"#;

        let response: GetRecentTradesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.0.len(), 2);

        // First trade
        assert_eq!(
            GetRecentTradesResponse::symbol(&response.0[0]),
            Some("BMX_ETH")
        );
        assert_eq!(
            GetRecentTradesResponse::timestamp(&response.0[0]),
            Some("1691743270994")
        );
        assert_eq!(
            GetRecentTradesResponse::price(&response.0[0]),
            Some("1.00000000")
        );
        assert_eq!(GetRecentTradesResponse::size(&response.0[0]), Some("1.0"));
        assert_eq!(GetRecentTradesResponse::side(&response.0[0]), Some("sell"));

        // Second trade
        assert_eq!(
            GetRecentTradesResponse::symbol(&response.0[1]),
            Some("BMX_ETH")
        );
        assert_eq!(
            GetRecentTradesResponse::timestamp(&response.0[1]),
            Some("1691743271000")
        );
        assert_eq!(
            GetRecentTradesResponse::price(&response.0[1]),
            Some("1.00000001")
        );
        assert_eq!(GetRecentTradesResponse::size(&response.0[1]), Some("2.5"));
        assert_eq!(GetRecentTradesResponse::side(&response.0[1]), Some("buy"));
    }

    #[test]
    fn test_trade_sides() {
        let buy_trade = vec![
            "BTC_USDT".to_string(),
            "1691743270994".to_string(),
            "30000.00".to_string(),
            "0.1".to_string(),
            "buy".to_string(),
        ];

        let sell_trade = vec![
            "BTC_USDT".to_string(),
            "1691743270995".to_string(),
            "29999.99".to_string(),
            "0.2".to_string(),
            "sell".to_string(),
        ];

        assert_eq!(GetRecentTradesResponse::side(&buy_trade), Some("buy"));
        assert_eq!(GetRecentTradesResponse::side(&sell_trade), Some("sell"));
    }

    #[test]
    fn test_max_limit() {
        let request = GetRecentTradesRequest {
            symbol: "BTC_USDT".to_string(),
            limit: Some(50), // Maximum allowed
        };

        assert_eq!(request.limit, Some(50));
    }
}
