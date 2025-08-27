use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, public_client::RestClient};

const GET_SPREAD_PUBLIC_TRADES_ENDPOINT: &str = "/api/v5/sprd/public-trades";

/// Request parameters for getting public spread trades
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSpreadPublicTradesRequest {
    /// Spread ID, e.g. BTC-USDT_BTC-USDT-SWAP
    #[serde(rename = "sprdId", skip_serializing_if = "Option::is_none")]
    pub sprd_id: Option<String>,
}

/// Response data for getting public spread trades
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpreadPublicTradeData {
    /// Spread ID
    #[serde(rename = "sprdId")]
    pub sprd_id: String,

    /// Trade ID
    #[serde(rename = "tradeId")]
    pub trade_id: String,

    /// Trade price
    #[serde(rename = "px")]
    pub px: String,

    /// Trade quantity
    #[serde(rename = "sz")]
    pub sz: String,

    /// Trade side of the taker
    /// buy
    /// sell
    #[serde(rename = "side")]
    pub side: String,

    /// Trade time, Unix timestamp format in milliseconds
    #[serde(rename = "ts")]
    pub ts: String,
}

impl RestClient {
    /// Get public spread trades
    /// Retrieve the recent transactions of a spread (at most 500 records per request)
    /// [docs](https://www.okx.com/docs-v5/en/#spread-trading-rest-api-get-public-trades-public)
    pub async fn get_spread_public_trades(
        &self,
        request: Option<GetSpreadPublicTradesRequest>,
    ) -> RestResult<SpreadPublicTradeData> {
        self.send_get_request(
            GET_SPREAD_PUBLIC_TRADES_ENDPOINT,
            request.as_ref(),
            EndpointType::PublicMarketData,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_get_spread_public_trades_request_with_sprd_id() {
        let request = GetSpreadPublicTradesRequest {
            sprd_id: Some("BTC-USDT_BTC-USDT-SWAP".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: GetSpreadPublicTradesRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_get_spread_public_trades_request_without_sprd_id() {
        let request = GetSpreadPublicTradesRequest { sprd_id: None };

        let serialized = serde_json::to_string(&request).unwrap();
        assert_eq!(serialized, "{}");
    }

    #[test]
    fn test_get_spread_public_trades_request_none() {
        let request: Option<GetSpreadPublicTradesRequest> = None;
        assert!(request.is_none());
    }

    #[test]
    fn test_spread_public_trade_data_deserialization() {
        let json_response = r#"{
            "sprdId": "BTC-USDT_BTC-USDT-SWAP",
            "tradeId": "123456789",
            "px": "50.5",
            "sz": "1.2",
            "side": "buy",
            "ts": "1597026383085"
        }"#;

        let trade: SpreadPublicTradeData = serde_json::from_str(json_response).unwrap();
        assert_eq!(trade.sprd_id, "BTC-USDT_BTC-USDT-SWAP");
        assert_eq!(trade.trade_id, "123456789");
        assert_eq!(trade.px, "50.5");
        assert_eq!(trade.sz, "1.2");
        assert_eq!(trade.side, "buy");
        assert_eq!(trade.ts, "1597026383085");
    }

    #[test]
    fn test_spread_public_trade_data_serialization() {
        let trade = SpreadPublicTradeData {
            sprd_id: "BTC-USDT_BTC-USDT-SWAP".to_string(),
            trade_id: "123456789".to_string(),
            px: "50.5".to_string(),
            sz: "1.2".to_string(),
            side: "buy".to_string(),
            ts: "1597026383085".to_string(),
        };

        let serialized = serde_json::to_string(&trade).unwrap();
        let deserialized: SpreadPublicTradeData = serde_json::from_str(&serialized).unwrap();
        assert_eq!(trade, deserialized);
    }

    #[test]
    fn test_trade_sides() {
        let sides = vec!["buy", "sell"];

        for side in sides {
            let json = format!(
                r#"{{
                "sprdId": "BTC-USDT_BTC-USDT-SWAP",
                "tradeId": "123456789",
                "px": "50.5",
                "sz": "1.2",
                "side": "{}",
                "ts": "1597026383085"
            }}"#,
                side
            );

            let trade: SpreadPublicTradeData = serde_json::from_str(&json).unwrap();
            assert_eq!(trade.side, side);
        }
    }

    #[test]
    fn test_spread_public_trades_multiple_entries() {
        // Test deserialization of multiple trade entries (as would be returned by the API)
        let trades = vec![
            SpreadPublicTradeData {
                sprd_id: "BTC-USDT_BTC-USDT-SWAP".to_string(),
                trade_id: "123456789".to_string(),
                px: "50.5".to_string(),
                sz: "1.2".to_string(),
                side: "buy".to_string(),
                ts: "1597026383085".to_string(),
            },
            SpreadPublicTradeData {
                sprd_id: "BTC-USDT_BTC-USDT-SWAP".to_string(),
                trade_id: "123456790".to_string(),
                px: "50.3".to_string(),
                sz: "0.8".to_string(),
                side: "sell".to_string(),
                ts: "1597026384085".to_string(),
            },
        ];

        let serialized = serde_json::to_string(&trades).unwrap();
        let deserialized: Vec<SpreadPublicTradeData> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(trades, deserialized);
        assert_eq!(deserialized.len(), 2);
    }

    #[test]
    fn test_timestamp_format() {
        let trade = SpreadPublicTradeData {
            sprd_id: "BTC-USDT_BTC-USDT-SWAP".to_string(),
            trade_id: "123456789".to_string(),
            px: "50.5".to_string(),
            sz: "1.2".to_string(),
            side: "buy".to_string(),
            ts: "1597026383085".to_string(),
        };

        // The timestamp should be a Unix timestamp in milliseconds
        assert_eq!(trade.ts.len(), 13); // Typical length of millisecond timestamp
        assert!(trade.ts.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_spread_id_formats() {
        let spread_ids = vec![
            "BTC-USDT_BTC-USDT-SWAP",
            "ETH-USD_ETH-USD-SWAP",
            "SOL-USDT_SOL-USDT-SWAP",
        ];

        for sprd_id in spread_ids {
            let request = GetSpreadPublicTradesRequest {
                sprd_id: Some(sprd_id.to_string()),
            };

            let serialized = serde_json::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("\"sprdId\":\"{}\"", sprd_id)));
        }
    }

    #[test]
    fn test_numeric_string_fields() {
        let trade = SpreadPublicTradeData {
            sprd_id: "BTC-USDT_BTC-USDT-SWAP".to_string(),
            trade_id: "123456789".to_string(),
            px: "50.5".to_string(),
            sz: "1.2".to_string(),
            side: "buy".to_string(),
            ts: "1597026383085".to_string(),
        };

        // All numeric fields should be strings as per API specification
        assert_eq!(trade.px, "50.5");
        assert_eq!(trade.sz, "1.2");
        assert_eq!(trade.trade_id, "123456789");
        assert_eq!(trade.ts, "1597026383085");
    }
}
