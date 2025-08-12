use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::RestResult;

const DEPTH_ENDPOINT: &str = "/api/v3/depth";

/// Request parameters for order book depth
#[derive(Debug, Clone, Serialize)]
pub struct DepthRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Number of entries to return. Default 100, Max 5000
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Order book depth response
#[derive(Debug, Clone, Deserialize)]
pub struct DepthResponse {
    /// Last update ID
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: u64,

    /// Bid orders (price, quantity)
    #[serde(rename = "bids")]
    pub bids: Vec<(Decimal, Decimal)>,

    /// Ask orders (price, quantity)
    #[serde(rename = "asks")]
    pub asks: Vec<(Decimal, Decimal)>,
}

impl RestClient {
    /// Get order book depth
    ///
    /// Returns the order book for a given symbol.
    ///
    /// [docs](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#order-book)
    /// Method: GET /api/v3/depth
    /// Weight: Variable based on limit (5-250)
    /// Security: None
    pub async fn get_depth(&self, params: DepthRequest) -> RestResult<DepthResponse> {
        // Calculate weight based on limit
        let weight = match params.limit.unwrap_or(100) {
            1..=100 => 5,
            101..=500 => 25,
            501..=1000 => 50,
            1001..=5000 => 250,
            _ => 250, // Use max weight for any value above 5000
        };

        self.send_get_request(DEPTH_ENDPOINT, Some(params), weight)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_depth_request_serialization() {
        let request = DepthRequest {
            symbol: "BTCUSDT".to_string(),
            limit: Some(100),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_depth_request_serialization_default_limit() {
        let request = DepthRequest {
            symbol: "ETHUSDT".to_string(),
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=ETHUSDT");
        assert!(!serialized.contains("limit"));
    }

    #[test]
    fn test_depth_request_serialization_max_limit() {
        let request = DepthRequest {
            symbol: "BTCUSDT".to_string(),
            limit: Some(5000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("limit=5000"));
    }

    #[test]
    fn test_depth_response_deserialization() {
        let json = r#"{
            "lastUpdateId": 1027024,
            "bids": [
                ["4.00000000", "431.00000000"],
                ["3.99000000", "123.45000000"],
                ["3.98000000", "50.00000000"]
            ],
            "asks": [
                ["4.00000200", "12.00000000"],
                ["4.01000000", "25.30000000"],
                ["4.02000000", "100.00000000"]
            ]
        }"#;

        let depth: DepthResponse = serde_json::from_str(json).unwrap();
        assert_eq!(depth.last_update_id, 1027024);

        // Check bids (should be sorted by price descending)
        assert_eq!(depth.bids.len(), 3);
        assert_eq!(depth.bids[0].0.to_string(), "4.00000000");
        assert_eq!(depth.bids[0].1.to_string(), "431.00000000");
        assert_eq!(depth.bids[1].0.to_string(), "3.99000000");
        assert_eq!(depth.bids[1].1.to_string(), "123.45000000");
        assert_eq!(depth.bids[2].0.to_string(), "3.98000000");
        assert_eq!(depth.bids[2].1.to_string(), "50.00000000");

        // Check asks (should be sorted by price ascending)
        assert_eq!(depth.asks.len(), 3);
        assert_eq!(depth.asks[0].0.to_string(), "4.00000200");
        assert_eq!(depth.asks[0].1.to_string(), "12.00000000");
        assert_eq!(depth.asks[1].0.to_string(), "4.01000000");
        assert_eq!(depth.asks[1].1.to_string(), "25.30000000");
        assert_eq!(depth.asks[2].0.to_string(), "4.02000000");
        assert_eq!(depth.asks[2].1.to_string(), "100.00000000");
    }

    #[test]
    fn test_depth_response_empty_orderbook() {
        let json = r#"{
            "lastUpdateId": 1027025,
            "bids": [],
            "asks": []
        }"#;

        let depth: DepthResponse = serde_json::from_str(json).unwrap();
        assert_eq!(depth.last_update_id, 1027025);
        assert_eq!(depth.bids.len(), 0);
        assert_eq!(depth.asks.len(), 0);
    }

    #[test]
    fn test_depth_response_high_precision() {
        let json = r#"{
            "lastUpdateId": 2000000,
            "bids": [
                ["45380.12345678", "0.00123456"],
                ["45379.87654321", "1.23456789"]
            ],
            "asks": [
                ["45381.11111111", "2.34567890"],
                ["45382.99999999", "0.99999999"]
            ]
        }"#;

        let depth: DepthResponse = serde_json::from_str(json).unwrap();
        assert_eq!(depth.last_update_id, 2000000);

        assert_eq!(depth.bids[0].0.to_string(), "45380.12345678");
        assert_eq!(depth.bids[0].1.to_string(), "0.00123456");
        assert_eq!(depth.bids[1].0.to_string(), "45379.87654321");
        assert_eq!(depth.bids[1].1.to_string(), "1.23456789");

        assert_eq!(depth.asks[0].0.to_string(), "45381.11111111");
        assert_eq!(depth.asks[0].1.to_string(), "2.34567890");
        assert_eq!(depth.asks[1].0.to_string(), "45382.99999999");
        assert_eq!(depth.asks[1].1.to_string(), "0.99999999");
    }

    #[test]
    fn test_depth_response_single_level() {
        let json = r#"{
            "lastUpdateId": 3000000,
            "bids": [
                ["50000.00", "1.50000000"]
            ],
            "asks": [
                ["50001.00", "0.75000000"]
            ]
        }"#;

        let depth: DepthResponse = serde_json::from_str(json).unwrap();
        assert_eq!(depth.bids.len(), 1);
        assert_eq!(depth.asks.len(), 1);
        assert_eq!(depth.bids[0].0.to_string(), "50000.00");
        assert_eq!(depth.bids[0].1.to_string(), "1.50000000");
        assert_eq!(depth.asks[0].0.to_string(), "50001.00");
        assert_eq!(depth.asks[0].1.to_string(), "0.75000000");
    }

    #[test]
    fn test_depth_response_large_orderbook() {
        // Simulate a large order book with multiple levels
        let mut bids = Vec::new();
        let mut asks = Vec::new();

        // Generate test data
        for i in 0..10 {
            bids.push(format!("[\"500{}.00\", \"1.0000\"]", 99 - i));
            asks.push(format!("[\"501{}.00\", \"1.0000\"]", i));
        }

        let json = format!(
            r#"{{
            "lastUpdateId": 4000000,
            "bids": [{}],
            "asks": [{}]
        }}"#,
            bids.join(","),
            asks.join(",")
        );

        let depth: DepthResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(depth.bids.len(), 10);
        assert_eq!(depth.asks.len(), 10);

        // Check first and last entries
        assert_eq!(depth.bids[0].0.to_string(), "50099.00");
        assert_eq!(depth.bids[9].0.to_string(), "50090.00");
        assert_eq!(depth.asks[0].0.to_string(), "5010.00");
        assert_eq!(depth.asks[9].0.to_string(), "5019.00");
    }

    #[test]
    fn test_depth_request_limit_values() {
        let limits = vec![1, 5, 10, 20, 50, 100, 500, 1000, 5000];

        for limit in limits {
            let request = DepthRequest {
                symbol: "BTCUSDT".to_string(),
                limit: Some(limit),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("limit={}", limit)));
        }
    }

    #[test]
    fn test_depth_response_zero_quantities() {
        let json = r#"{
            "lastUpdateId": 5000000,
            "bids": [
                ["45000.00", "0.00000000"],
                ["44999.00", "1.23456789"]
            ],
            "asks": [
                ["45001.00", "0.00000000"],
                ["45002.00", "2.34567890"]
            ]
        }"#;

        let depth: DepthResponse = serde_json::from_str(json).unwrap();
        assert_eq!(depth.bids[0].1.to_string(), "0.00000000");
        assert_eq!(depth.asks[0].1.to_string(), "0.00000000");
        // Non-zero quantities should parse correctly
        assert_eq!(depth.bids[1].1.to_string(), "1.23456789");
        assert_eq!(depth.asks[1].1.to_string(), "2.34567890");
    }

    #[test]
    fn test_depth_response_altcoin_pair() {
        let json = r#"{
            "lastUpdateId": 6000000,
            "bids": [
                ["0.00001234", "1000000.00000000"],
                ["0.00001233", "500000.00000000"]
            ],
            "asks": [
                ["0.00001235", "750000.00000000"],
                ["0.00001236", "250000.00000000"]
            ]
        }"#;

        let depth: DepthResponse = serde_json::from_str(json).unwrap();
        // Test small price values (like satoshi-level pricing)
        assert_eq!(depth.bids[0].0.to_string(), "0.00001234");
        assert_eq!(depth.bids[0].1.to_string(), "1000000.00000000");
        assert_eq!(depth.asks[0].0.to_string(), "0.00001235");
        assert_eq!(depth.asks[0].1.to_string(), "750000.00000000");
    }
}
