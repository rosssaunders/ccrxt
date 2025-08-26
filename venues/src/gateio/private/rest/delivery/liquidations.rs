use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const DELIVERY_LIQUIDATES_ENDPOINT: &str = "/delivery/{}/liquidates";

/// Request parameters for delivery liquidation history
#[derive(Debug, Clone, Serialize, Default)]
pub struct DeliveryLiquidationHistoryRequest {
    /// Settlement currency
    pub settle: String,

    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,

    /// Start time filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Maximum number of records (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Delivery liquidation history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryLiquidation {
    /// Liquidation time
    pub time: f64,

    /// Contract name
    pub contract: String,

    /// Liquidation size
    pub size: i64,

    /// Liquidation price
    pub price: String,

    /// Left position size after liquidation
    pub left: i64,

    /// Leverage
    pub leverage: String,

    /// Margin
    pub margin: String,

    /// Entry price
    pub entry_price: String,

    /// Liquidation fee
    pub liq_price: String,

    /// Mark price
    pub mark_price: String,
}

impl RestClient {
    /// List delivery liquidation history
    ///
    /// Retrieves the user's liquidation history for delivery contracts.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#query-liquidation-history-2)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `params` - The liquidation history request parameters
    ///
    /// # Returns
    /// List of liquidation history entries
    pub async fn get_delivery_liquidation_history(
        &self,
        params: DeliveryLiquidationHistoryRequest,
    ) -> RestResult<Vec<DeliveryLiquidation>> {
        let endpoint = DELIVERY_LIQUIDATES_ENDPOINT.replace("{}", &params.settle);
        self.get_with_query(&endpoint, &params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delivery_liquidates_endpoint() {
        assert_eq!(DELIVERY_LIQUIDATES_ENDPOINT, "/delivery/{}/liquidates");
    }

    #[test]
    fn test_liquidation_history_request_minimal() {
        let request = DeliveryLiquidationHistoryRequest {
            settle: "BTC".to_string(),
            ..Default::default()
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "BTC");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 1);
        assert!(!obj.contains_key("contract"));
        assert!(!obj.contains_key("from"));
        assert!(!obj.contains_key("to"));
        assert!(!obj.contains_key("limit"));
    }

    #[test]
    fn test_liquidation_history_request_full() {
        let request = DeliveryLiquidationHistoryRequest {
            settle: "USDT".to_string(),
            contract: Some("BTC_USDT_20240315".to_string()),
            from: Some(1640995200),
            to: Some(1640995800),
            limit: Some(50),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT_20240315");
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["to"], 1640995800);
        assert_eq!(json["limit"], 50);

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 5);
    }

    #[test]
    fn test_delivery_liquidation_deserialization() {
        let json = r#"{
            "time": 1640995200.123,
            "contract": "BTC_USDT_20240315",
            "size": 100,
            "price": "44000.0",
            "left": 0,
            "leverage": "10",
            "margin": "1000.0",
            "entry_price": "45000.0",
            "liq_price": "4050.0",
            "mark_price": "44000.0"
        }"#;

        let liquidation: DeliveryLiquidation = serde_json::from_str(json).unwrap();
        assert_eq!(liquidation.time, 1640995200.123);
        assert_eq!(liquidation.contract, "BTC_USDT_20240315");
        assert_eq!(liquidation.size, 100);
        assert_eq!(liquidation.price, "44000.0");
        assert_eq!(liquidation.left, 0);
        assert_eq!(liquidation.leverage, "10");
        assert_eq!(liquidation.margin, "1000.0");
        assert_eq!(liquidation.entry_price, "45000.0");
        assert_eq!(liquidation.liq_price, "4050.0");
        assert_eq!(liquidation.mark_price, "44000.0");
    }

    #[test]
    fn test_delivery_liquidation_round_trip() {
        let original = DeliveryLiquidation {
            time: 1640995300.456,
            contract: "ETH_USDT_20240415".to_string(),
            size: 50,
            price: "2800.0".to_string(),
            left: 25,
            leverage: "5".to_string(),
            margin: "500.0".to_string(),
            entry_price: "3000.0".to_string(),
            liq_price: "2700.0".to_string(),
            mark_price: "2800.0".to_string(),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: DeliveryLiquidation = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.time, original.time);
        assert_eq!(deserialized.contract, original.contract);
        assert_eq!(deserialized.size, original.size);
        assert_eq!(deserialized.price, original.price);
        assert_eq!(deserialized.left, original.left);
        assert_eq!(deserialized.leverage, original.leverage);
        assert_eq!(deserialized.margin, original.margin);
        assert_eq!(deserialized.entry_price, original.entry_price);
        assert_eq!(deserialized.liq_price, original.liq_price);
        assert_eq!(deserialized.mark_price, original.mark_price);
    }

    #[test]
    fn test_liquidation_endpoint_construction() {
        let settle = "BTC";
        let endpoint = DELIVERY_LIQUIDATES_ENDPOINT.replace("{}", settle);
        assert_eq!(endpoint, "/delivery/BTC/liquidates");
    }

    #[test]
    fn test_liquidation_different_settlements() {
        let settlements = vec!["BTC", "USDT", "ETH"];
        for settle in settlements {
            let endpoint = DELIVERY_LIQUIDATES_ENDPOINT.replace("{}", settle);
            assert_eq!(endpoint, format!("/delivery/{}/liquidates", settle));
        }
    }

    #[test]
    fn test_liquidation_price_validation() {
        let liquidation = DeliveryLiquidation {
            time: 1640995200.0,
            contract: "BTC_USDT_20240315".to_string(),
            size: 100,
            price: "44000.0".to_string(),
            left: 0,
            leverage: "10".to_string(),
            margin: "1000.0".to_string(),
            entry_price: "45000.0".to_string(),
            liq_price: "4050.0".to_string(),
            mark_price: "44000.0".to_string(),
        };

        // All price fields should be parseable as numbers
        assert!(liquidation.price.parse::<f64>().is_ok());
        assert!(liquidation.leverage.parse::<f64>().is_ok());
        assert!(liquidation.margin.parse::<f64>().is_ok());
        assert!(liquidation.entry_price.parse::<f64>().is_ok());
        assert!(liquidation.liq_price.parse::<f64>().is_ok());
        assert!(liquidation.mark_price.parse::<f64>().is_ok());
    }
}
