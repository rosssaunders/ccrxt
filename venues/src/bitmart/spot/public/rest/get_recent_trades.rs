use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bitmart::{RestResult, rate_limit::EndpointType};

const RECENT_TRADES_ENDPOINT: &str = "/spot/quotation/v3/trades";

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

impl RestClient {
    /// Get Recent Trades (V3)
    ///
    /// Get the latest trade records of the specified trading pair.
    /// Note that the interface is not real-time data, if you need real-time data,
    /// please use websocket to subscribe Trade channel
    ///
    /// [docs](https://developer-pro.bitmart.com/en/spot/#get-recent-trades-v3)
    ///
    /// Rate limit: 10 times/2sec per IP
    ///
    /// # Arguments
    /// * `request` - The request parameters including symbol and optional limit
    ///
    /// # Returns
    /// Latest trade records for the specified trading pair
    pub async fn get_recent_trades(
        &self,
        request: GetRecentTradesRequest,
    ) -> RestResult<GetRecentTradesResponse> {
        self.send_get_request(
            RECENT_TRADES_ENDPOINT,
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

        assert_eq!(trade_data.first().map(|s| s.as_str()), Some("BMX_ETH"));
        assert_eq!(trade_data.get(1).map(|s| s.as_str()), Some("1691743270994"));
        assert_eq!(trade_data.get(2).map(|s| s.as_str()), Some("1.00000000"));
        assert_eq!(trade_data.get(3).map(|s| s.as_str()), Some("1.0"));
        assert_eq!(trade_data.get(4).map(|s| s.as_str()), Some("sell"));
    }

    #[test]
    fn test_trade_data_incomplete() {
        let trade_data = [
            "BMX_ETH".to_string(),
            "1691743270994".to_string(),
            "1.00000000".to_string(),
        ];

        assert_eq!(trade_data.first().map(|s| s.as_str()), Some("BMX_ETH"));
        assert_eq!(trade_data.get(1).map(|s| s.as_str()), Some("1691743270994"));
        assert_eq!(trade_data.get(2).map(|s| s.as_str()), Some("1.00000000"));
        assert_eq!(trade_data.get(3).map(|s| s.as_str()), None);
        assert_eq!(trade_data.get(4).map(|s| s.as_str()), None);
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
        assert_eq!(response.0[0].first().map(|s| s.as_str()), Some("BMX_ETH"));
        assert_eq!(response.0[0].get(4).map(|s| s.as_str()), Some("sell"));
        assert_eq!(response.0[1].first().map(|s| s.as_str()), Some("BTC_USDT"));
        assert_eq!(response.0[1].get(4).map(|s| s.as_str()), Some("buy"));
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
        assert_eq!(response.0[0].first().map(|s| s.as_str()), Some("BMX_ETH"));
        assert_eq!(
            response.0[0].get(1).map(|s| s.as_str()),
            Some("1691743270994")
        );
        assert_eq!(response.0[0].get(2).map(|s| s.as_str()), Some("1.00000000"));
        assert_eq!(response.0[0].get(3).map(|s| s.as_str()), Some("1.0"));
        assert_eq!(response.0[0].get(4).map(|s| s.as_str()), Some("sell"));
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
        assert_eq!(response.0[0].first().map(|s| s.as_str()), Some("BMX_ETH"));
        assert_eq!(
            response.0[0].get(1).map(|s| s.as_str()),
            Some("1691743270994")
        );
        assert_eq!(response.0[0].get(2).map(|s| s.as_str()), Some("1.00000000"));
        assert_eq!(response.0[0].get(3).map(|s| s.as_str()), Some("1.0"));
        assert_eq!(response.0[0].get(4).map(|s| s.as_str()), Some("sell"));

        // Second trade
        assert_eq!(response.0[1].first().map(|s| s.as_str()), Some("BMX_ETH"));
        assert_eq!(
            response.0[1].get(1).map(|s| s.as_str()),
            Some("1691743271000")
        );
        assert_eq!(response.0[1].get(2).map(|s| s.as_str()), Some("1.00000001"));
        assert_eq!(response.0[1].get(3).map(|s| s.as_str()), Some("2.5"));
        assert_eq!(response.0[1].get(4).map(|s| s.as_str()), Some("buy"));
    }

    #[test]
    fn test_trade_sides() {
        let buy_trade = [
            "BTC_USDT".to_string(),
            "1691743270994".to_string(),
            "30000.00".to_string(),
            "0.1".to_string(),
            "buy".to_string(),
        ];

        let sell_trade = [
            "BTC_USDT".to_string(),
            "1691743270995".to_string(),
            "29999.99".to_string(),
            "0.2".to_string(),
            "sell".to_string(),
        ];

        assert_eq!(buy_trade.get(4).map(|s| s.as_str()), Some("buy"));
        assert_eq!(sell_trade.get(4).map(|s| s.as_str()), Some("sell"));
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
