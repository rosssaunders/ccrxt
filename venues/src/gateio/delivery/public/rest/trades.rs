use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::gateio::delivery::RestResult;

const DELIVERY_TRADES_ENDPOINT: &str = "/delivery/{}/trades";

/// Request parameters for delivery trades
#[derive(Debug, Clone, Serialize, Default)]
pub struct DeliveryTradesRequest {
    /// Settlement currency
    pub settle: String,

    /// Contract name
    pub contract: String,

    /// Maximum number of records to return (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Specify list offset (default 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    /// Specify the starting point for this list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,

    /// Specify starting time in Unix seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// Specify ending time in Unix seconds  
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
}

/// Delivery trade entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryTrade {
    /// Trade ID
    pub id: i64,

    /// Trading time
    pub create_time: f64,

    /// Trading contract
    pub contract: String,

    /// Trading size
    pub size: i64,

    /// Trading price
    pub price: String,

    /// Whether internal trade
    pub is_internal: Option<bool>,
}

impl RestClient {
    /// Futures trading history
    ///
    /// Retrieve futures trading history
    ///
    /// [docs]: https://www.gate.io/docs/developers/apiv4/en/#futures-trading-history-2
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `params` - The delivery trades request parameters
    ///
    /// # Returns
    /// List of delivery trades
    pub async fn get_delivery_trades(
        &self,
        params: DeliveryTradesRequest,
    ) -> RestResult<Vec<DeliveryTrade>> {
        let endpoint = DELIVERY_TRADES_ENDPOINT.replace("{}", &params.settle);
        self.get_with_query(&endpoint, Some(&params)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delivery_trades_endpoint_constant() {
        assert_eq!(DELIVERY_TRADES_ENDPOINT, "/delivery/{}/trades");
    }

    #[test]
    fn test_delivery_trades_request_minimal() {
        let request = DeliveryTradesRequest {
            settle: "BTC".to_string(),
            contract: "BTC_USDT_20240315".to_string(),
            limit: None,
            offset: None,
            last_id: None,
            from: None,
            to: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "BTC");
        assert_eq!(json["contract"], "BTC_USDT_20240315");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 2); // Only settle and contract
        assert!(!obj.contains_key("limit"));
        assert!(!obj.contains_key("offset"));
        assert!(!obj.contains_key("last_id"));
        assert!(!obj.contains_key("from"));
        assert!(!obj.contains_key("to"));
    }

    #[test]
    fn test_delivery_trades_request_with_limit() {
        let request = DeliveryTradesRequest {
            settle: "BTC".to_string(),
            contract: "BTC_USDT_20240315".to_string(),
            limit: Some(500),
            offset: None,
            last_id: None,
            from: None,
            to: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["limit"], 500);

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 3);
    }

    #[test]
    fn test_delivery_trades_request_with_pagination() {
        let request = DeliveryTradesRequest {
            settle: "USDT".to_string(),
            contract: "ETH_USDT_20240415".to_string(),
            limit: Some(100),
            offset: Some(50),
            last_id: None,
            from: None,
            to: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "ETH_USDT_20240415");
        assert_eq!(json["limit"], 100);
        assert_eq!(json["offset"], 50);

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 4);
    }

    #[test]
    fn test_delivery_trades_request_with_time_range() {
        let request = DeliveryTradesRequest {
            settle: "BTC".to_string(),
            contract: "BTC_USDT_20240315".to_string(),
            limit: None,
            offset: None,
            last_id: None,
            from: Some(1640995200),
            to: Some(1641081600),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["to"], 1641081600);

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 4);
    }

    #[test]
    fn test_delivery_trades_request_full_parameters() {
        let request = DeliveryTradesRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT_20240315".to_string(),
            limit: Some(1000),
            offset: Some(100),
            last_id: Some("trade_12345".to_string()),
            from: Some(1640995200),
            to: Some(1641081600),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT_20240315");
        assert_eq!(json["limit"], 1000);
        assert_eq!(json["offset"], 100);
        assert_eq!(json["last_id"], "trade_12345");
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["to"], 1641081600);

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 7);
    }

    #[test]
    fn test_delivery_trades_request_different_settlements() {
        let settlements = vec!["BTC", "USDT", "ETH"];

        for settle in settlements {
            let request = DeliveryTradesRequest {
                settle: settle.to_string(),
                contract: format!("{}_USDT_20240315", settle),
                limit: None,
                offset: None,
                last_id: None,
                from: None,
                to: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["settle"], settle);
            assert_eq!(json["contract"], format!("{}_USDT_20240315", settle));
        }
    }

    #[test]
    fn test_delivery_trades_request_default() {
        let request = DeliveryTradesRequest::default();
        assert_eq!(request.settle, "");
        assert_eq!(request.contract, "");
        assert_eq!(request.limit, None);
        assert_eq!(request.offset, None);
        assert_eq!(request.last_id, None);
        assert_eq!(request.from, None);
        assert_eq!(request.to, None);
    }

    #[test]
    fn test_delivery_trade_deserialization() {
        let json = r#"{
            "id": 123456789,
            "create_time": 1640995200.123,
            "contract": "BTC_USDT_20240315",
            "size": 1000,
            "price": "45000.50",
            "is_internal": false
        }"#;

        let trade: DeliveryTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, 123456789);
        assert_eq!(trade.create_time, 1640995200.123);
        assert_eq!(trade.contract, "BTC_USDT_20240315");
        assert_eq!(trade.size, 1000);
        assert_eq!(trade.price, "45000.50");
        assert_eq!(trade.is_internal, Some(false));
    }

    #[test]
    fn test_delivery_trade_deserialization_minimal() {
        let json = r#"{
            "id": 987654321,
            "create_time": 1640995200.0,
            "contract": "ETH_USDT_20240415",
            "size": 500,
            "price": "3500.00"
        }"#;

        let trade: DeliveryTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, 987654321);
        assert_eq!(trade.create_time, 1640995200.0);
        assert_eq!(trade.contract, "ETH_USDT_20240415");
        assert_eq!(trade.size, 500);
        assert_eq!(trade.price, "3500.00");
        assert_eq!(trade.is_internal, None);
    }

    #[test]
    fn test_endpoint_path_construction() {
        let settlements = vec!["BTC", "USDT", "ETH"];

        for settle in settlements {
            let endpoint = DELIVERY_TRADES_ENDPOINT.replace("{}", settle);
            assert_eq!(endpoint, format!("/delivery/{}/trades", settle));
        }
    }

    #[test]
    fn test_delivery_trades_request_limit_edge_cases() {
        // Test maximum limit
        let request = DeliveryTradesRequest {
            settle: "BTC".to_string(),
            contract: "BTC_USDT_20240315".to_string(),
            limit: Some(1000),
            offset: None,
            last_id: None,
            from: None,
            to: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["limit"], 1000);

        // Test minimum limit
        let request = DeliveryTradesRequest {
            settle: "BTC".to_string(),
            contract: "BTC_USDT_20240315".to_string(),
            limit: Some(1),
            offset: None,
            last_id: None,
            from: None,
            to: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["limit"], 1);
    }

    #[test]
    fn test_delivery_trades_request_extreme_values() {
        let request = DeliveryTradesRequest {
            settle: "BTC".to_string(),
            contract: "BTC_USDT_20240315".to_string(),
            limit: Some(i32::MAX),
            offset: Some(i32::MAX),
            last_id: Some("extreme_trade_id".to_string()),
            from: Some(i64::MIN),
            to: Some(i64::MAX),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["limit"], i32::MAX);
        assert_eq!(json["offset"], i32::MAX);
        assert_eq!(json["from"], i64::MIN);
        assert_eq!(json["to"], i64::MAX);
    }

    #[test]
    fn test_delivery_trades_request_serialization_skips_none() {
        let request = DeliveryTradesRequest {
            settle: "BTC".to_string(),
            contract: "BTC_USDT_20240315".to_string(),
            limit: None,
            offset: Some(50),
            last_id: None,
            from: Some(1640995200),
            to: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();

        assert!(obj.contains_key("settle"));
        assert!(obj.contains_key("contract"));
        assert!(!obj.contains_key("limit"));
        assert!(obj.contains_key("offset"));
        assert!(!obj.contains_key("last_id"));
        assert!(obj.contains_key("from"));
        assert!(!obj.contains_key("to"));
    }
}
