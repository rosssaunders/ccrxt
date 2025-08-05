use serde::{Deserialize, Serialize};

use super::RestClient;

const DELIVERY_SETTLEMENTS_ENDPOINT: &str = "/delivery/{}/settlements";

/// Delivery settlement record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliverySettlement {
    /// Settlement time
    pub time: i64,

    /// Contract
    pub contract: String,

    /// Profit in settlement currency
    pub profit: String,

    /// Settlement price
    pub settle_price: String,

    /// Position size at settlement
    pub size: i64,
}

impl RestClient {
    /// Get delivery settlements
    ///
    /// This endpoint returns settlement history for delivery contracts.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `settle` - Settlement currency
    /// * `contract` - Optional contract filter
    /// * `limit` - Optional limit for number of records
    ///
    /// # Returns
    /// List of delivery settlement records
    pub async fn get_delivery_settlements(
        &self,
        settle: &str,
        contract: Option<&str>,
        limit: Option<i32>,
    ) -> crate::gateio::delivery::Result<Vec<DeliverySettlement>> {
        let mut endpoint = DELIVERY_SETTLEMENTS_ENDPOINT.replace("{}", settle);
        let mut query_params = Vec::new();

        if let Some(contract) = contract {
            query_params.push(format!("contract={}", contract));
        }
        if let Some(limit) = limit {
            query_params.push(format!("limit={}", limit));
        }

        if !query_params.is_empty() {
            endpoint.push('?');
            endpoint.push_str(&query_params.join("&"));
        }

        self.get(&endpoint).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delivery_settlements_endpoint() {
        assert_eq!(DELIVERY_SETTLEMENTS_ENDPOINT, "/delivery/{}/settlements");
    }

    #[test]
    fn test_settlements_endpoint_construction() {
        let settle = "BTC";
        let endpoint = DELIVERY_SETTLEMENTS_ENDPOINT.replace("{}", settle);
        assert_eq!(endpoint, "/delivery/BTC/settlements");
    }

    #[test]
    fn test_settlements_endpoint_different_settlements() {
        let settlements = vec!["BTC", "USDT", "ETH"];
        for settle in settlements {
            let endpoint = DELIVERY_SETTLEMENTS_ENDPOINT.replace("{}", settle);
            assert_eq!(endpoint, format!("/delivery/{}/settlements", settle));
        }
    }

    #[test]
    fn test_delivery_settlement_deserialization() {
        let json = r#"{
            "time": 1640995200,
            "contract": "BTC_USDT_20240315",
            "profit": "150.25",
            "settle_price": "45000.0",
            "size": 100
        }"#;

        let settlement: DeliverySettlement = serde_json::from_str(json).unwrap();
        assert_eq!(settlement.time, 1640995200);
        assert_eq!(settlement.contract, "BTC_USDT_20240315");
        assert_eq!(settlement.profit, "150.25");
        assert_eq!(settlement.settle_price, "45000.0");
        assert_eq!(settlement.size, 100);
    }

    #[test]
    fn test_delivery_settlement_round_trip() {
        let original = DeliverySettlement {
            time: 1640995300,
            contract: "ETH_USDT_20240415".to_string(),
            profit: "-25.75".to_string(),
            settle_price: "3000.0".to_string(),
            size: 50,
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: DeliverySettlement = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.time, original.time);
        assert_eq!(deserialized.contract, original.contract);
        assert_eq!(deserialized.profit, original.profit);
        assert_eq!(deserialized.settle_price, original.settle_price);
        assert_eq!(deserialized.size, original.size);
    }

    #[test]
    fn test_settlement_positive_and_negative_profit() {
        let profitable = DeliverySettlement {
            time: 1640995200,
            contract: "BTC_USDT_20240315".to_string(),
            profit: "100.0".to_string(),
            settle_price: "45000.0".to_string(),
            size: 50,
        };

        let loss = DeliverySettlement {
            time: 1640995300,
            contract: "BTC_USDT_20240315".to_string(),
            profit: "-75.0".to_string(),
            settle_price: "45000.0".to_string(),
            size: 25,
        };

        assert!(profitable.profit.parse::<f64>().unwrap() > 0.0);
        assert!(loss.profit.parse::<f64>().unwrap() < 0.0);
    }
}
