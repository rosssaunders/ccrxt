use serde::{Deserialize, Serialize};

use super::RestClient;

const DELIVERY_POSITION_CLOSE_ENDPOINT: &str = "/delivery/{}/position_close";

/// Request parameters for delivery position close history
#[derive(Debug, Clone, Serialize, Default)]
pub struct DeliveryPositionCloseHistoryRequest {
    /// Settlement currency
    pub settle: String,

    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,

    /// Maximum number of records (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Page offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    /// Start time filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Order side filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
}

/// Delivery position close history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryPositionClose {
    /// Position close time
    pub time: f64,

    /// PnL
    pub pnl: String,

    /// Position side
    pub side: String,

    /// Contract name
    pub contract: String,

    /// Text
    pub text: String,

    /// Maximum position size during the period
    pub max_size: i64,
}

impl RestClient {
    /// List delivery position close history
    ///
    /// Retrieves history of closed delivery positions.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `params` - The position close history request parameters
    ///
    /// # Returns
    /// List of position close history entries
    pub async fn get_delivery_position_close_history(
        &self,
        params: DeliveryPositionCloseHistoryRequest,
    ) -> crate::gateio::delivery::Result<Vec<DeliveryPositionClose>> {
        let endpoint = DELIVERY_POSITION_CLOSE_ENDPOINT.replace("{}", &params.settle);
        self.get_with_query(&endpoint, &params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delivery_position_close_endpoint() {
        assert_eq!(DELIVERY_POSITION_CLOSE_ENDPOINT, "/delivery/{}/position_close");
    }

    #[test]
    fn test_position_close_history_request_minimal() {
        let request = DeliveryPositionCloseHistoryRequest {
            settle: "BTC".to_string(),
            ..Default::default()
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "BTC");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 1);
        assert!(!obj.contains_key("contract"));
        assert!(!obj.contains_key("limit"));
        assert!(!obj.contains_key("offset"));
        assert!(!obj.contains_key("from"));
        assert!(!obj.contains_key("to"));
        assert!(!obj.contains_key("side"));
    }

    #[test]
    fn test_position_close_history_request_full() {
        let request = DeliveryPositionCloseHistoryRequest {
            settle: "USDT".to_string(),
            contract: Some("BTC_USDT_20240315".to_string()),
            limit: Some(50),
            offset: Some(10),
            from: Some(1640995200),
            to: Some(1640995800),
            side: Some("long".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT_20240315");
        assert_eq!(json["limit"], 50);
        assert_eq!(json["offset"], 10);
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["to"], 1640995800);
        assert_eq!(json["side"], "long");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 7);
    }

    #[test]
    fn test_position_close_history_request_with_contract_filter() {
        let request = DeliveryPositionCloseHistoryRequest {
            settle: "BTC".to_string(),
            contract: Some("ETH_BTC_20240415".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["contract"], "ETH_BTC_20240415");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 2);
    }

    #[test]
    fn test_position_close_history_request_with_pagination() {
        let request = DeliveryPositionCloseHistoryRequest {
            settle: "USDT".to_string(),
            limit: Some(100),
            offset: Some(25),
            ..Default::default()
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["limit"], 100);
        assert_eq!(json["offset"], 25);

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 3);
    }

    #[test]
    fn test_position_close_history_request_with_time_range() {
        let request = DeliveryPositionCloseHistoryRequest {
            settle: "BTC".to_string(),
            from: Some(1640995200),
            to: Some(1640995800),
            ..Default::default()
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["to"], 1640995800);

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 3);
    }

    #[test]
    fn test_position_close_deserialization() {
        let json = r#"{
            "time": 1640995200.123,
            "pnl": "150.25",
            "side": "long",
            "contract": "BTC_USDT_20240315",
            "text": "Position closed at profit",
            "max_size": 100
        }"#;

        let position_close: DeliveryPositionClose = serde_json::from_str(json).unwrap();
        assert_eq!(position_close.time, 1640995200.123);
        assert_eq!(position_close.pnl, "150.25");
        assert_eq!(position_close.side, "long");
        assert_eq!(position_close.contract, "BTC_USDT_20240315");
        assert_eq!(position_close.text, "Position closed at profit");
        assert_eq!(position_close.max_size, 100);
    }

    #[test]
    fn test_position_close_round_trip() {
        let original = DeliveryPositionClose {
            time: 1640995300.456,
            pnl: "-25.75".to_string(),
            side: "short".to_string(),
            contract: "ETH_USDT_20240415".to_string(),
            text: "Position closed at loss".to_string(),
            max_size: 50,
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: DeliveryPositionClose = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.time, original.time);
        assert_eq!(deserialized.pnl, original.pnl);
        assert_eq!(deserialized.side, original.side);
        assert_eq!(deserialized.contract, original.contract);
        assert_eq!(deserialized.text, original.text);
        assert_eq!(deserialized.max_size, original.max_size);
    }

    #[test]
    fn test_position_close_different_sides() {
        let long_close = DeliveryPositionClose {
            time: 1640995200.0,
            pnl: "100.0".to_string(),
            side: "long".to_string(),
            contract: "BTC_USDT_20240315".to_string(),
            text: "Long position closed".to_string(),
            max_size: 75,
        };

        let short_close = DeliveryPositionClose {
            time: 1640995300.0,
            pnl: "-50.0".to_string(),
            side: "short".to_string(),
            contract: "BTC_USDT_20240315".to_string(),
            text: "Short position closed".to_string(),
            max_size: 25,
        };

        assert_eq!(long_close.side, "long");
        assert_eq!(short_close.side, "short");
        assert_ne!(long_close.pnl, short_close.pnl);
        assert_ne!(long_close.max_size, short_close.max_size);
    }

    #[test]
    fn test_position_close_endpoint_construction() {
        let settle = "BTC";
        let endpoint = DELIVERY_POSITION_CLOSE_ENDPOINT.replace("{}", settle);
        assert_eq!(endpoint, "/delivery/BTC/position_close");
    }

    #[test]
    fn test_position_close_different_settlements() {
        let settlements = vec!["BTC", "USDT", "ETH"];
        for settle in settlements {
            let endpoint = DELIVERY_POSITION_CLOSE_ENDPOINT.replace("{}", settle);
            assert_eq!(endpoint, format!("/delivery/{}/position_close", settle));
        }
    }
}
