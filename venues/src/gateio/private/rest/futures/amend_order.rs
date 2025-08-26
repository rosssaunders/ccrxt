use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult, order::FuturesOrder};

const ENDPOINT_FUTURES_PREFIX: &str = "/futures";

/// Request to amend a futures order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmendFuturesOrderRequest {
    /// Settlement currency
    pub settle: String,

    /// Order ID to amend
    pub order_id: String,

    /// New order size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,

    /// New order price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Amendment text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amend_text: Option<String>,
}

impl RestClient {
    /// Amend a futures order
    ///
    /// Modifies the price and/or size of an existing order.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#amend-an-order)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The order amendment request parameters
    ///
    /// # Returns
    /// Amended order details
    pub async fn amend_futures_order(
        &self,
        request: AmendFuturesOrderRequest,
    ) -> RestResult<FuturesOrder> {
        let endpoint = format!(
            "{}/{}/orders/{}",
            ENDPOINT_FUTURES_PREFIX, request.settle, request.order_id
        );
        self.put(&endpoint, &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amend_order_request_price_only() {
        let request = AmendFuturesOrderRequest {
            settle: "USDT".to_string(),
            order_id: "12345".to_string(),
            size: None,
            price: Some("43500.0".to_string()),
            amend_text: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["order_id"], "12345");
        assert_eq!(json["price"], "43500.0");
        assert!(!json.as_object().unwrap().contains_key("size"));
        assert!(!json.as_object().unwrap().contains_key("amend_text"));
    }

    #[test]
    fn test_amend_order_request_size_only() {
        let request = AmendFuturesOrderRequest {
            settle: "USDT".to_string(),
            order_id: "67890".to_string(),
            size: Some(2000),
            price: None,
            amend_text: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["order_id"], "67890");
        assert_eq!(json["size"], 2000);
        assert!(!json.as_object().unwrap().contains_key("price"));
        assert!(!json.as_object().unwrap().contains_key("amend_text"));
    }

    #[test]
    fn test_amend_order_request_both_price_and_size() {
        let request = AmendFuturesOrderRequest {
            settle: "USDT".to_string(),
            order_id: "11111".to_string(),
            size: Some(-3000),
            price: Some("2700.50".to_string()),
            amend_text: Some("Update both price and size".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["order_id"], "11111");
        assert_eq!(json["size"], -3000);
        assert_eq!(json["price"], "2700.50");
        assert_eq!(json["amend_text"], "Update both price and size");
    }

    #[test]
    fn test_amend_order_various_sizes() {
        let size_scenarios = vec![
            (1000, "Increase long position"),
            (-1000, "Increase short position"),
            (500, "Reduce long position"),
            (-500, "Reduce short position"),
            (10000, "Large position"),
        ];

        for (size, _description) in size_scenarios {
            let request = AmendFuturesOrderRequest {
                settle: "USDT".to_string(),
                order_id: "test123".to_string(),
                size: Some(size),
                price: None,
                amend_text: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["size"], size);
        }
    }

    #[test]
    fn test_amend_order_various_prices() {
        let price_scenarios = vec!["43000.0", "43000.50", "43000.25", "0.001", "999999.99"];

        for price in price_scenarios {
            let request = AmendFuturesOrderRequest {
                settle: "USDT".to_string(),
                order_id: "test456".to_string(),
                size: None,
                price: Some(price.to_string()),
                amend_text: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["price"], price);
        }
    }

    #[test]
    fn test_amend_order_with_text() {
        let text_scenarios = vec![
            "Adjust to market conditions",
            "Update TP level",
            "Reduce exposure",
            "Improve entry price",
            "Risk management adjustment",
        ];

        for text in text_scenarios {
            let request = AmendFuturesOrderRequest {
                settle: "USDT".to_string(),
                order_id: "test789".to_string(),
                size: Some(1000),
                price: Some("43000.0".to_string()),
                amend_text: Some(text.to_string()),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["amend_text"], text);
        }
    }

    #[test]
    fn test_amend_order_realistic_scenarios() {
        // Scenario 1: Move stop loss order
        let stop_loss_update = AmendFuturesOrderRequest {
            settle: "USDT".to_string(),
            order_id: "sl_12345".to_string(),
            size: None,
            price: Some("42500.0".to_string()),
            amend_text: Some("Move stop loss higher".to_string()),
        };

        let json = serde_json::to_value(&stop_loss_update).unwrap();
        assert_eq!(json["price"], "42500.0");
        assert_eq!(json["amend_text"], "Move stop loss higher");

        // Scenario 2: Reduce position size
        let reduce_position = AmendFuturesOrderRequest {
            settle: "USDT".to_string(),
            order_id: "pos_67890".to_string(),
            size: Some(500), // Reduce from 1000 to 500
            price: None,
            amend_text: Some("Reduce exposure".to_string()),
        };

        let json = serde_json::to_value(&reduce_position).unwrap();
        assert_eq!(json["size"], 500);
        assert!(!json.as_object().unwrap().contains_key("price"));

        // Scenario 3: Improve limit order
        let improve_limit = AmendFuturesOrderRequest {
            settle: "USDT".to_string(),
            order_id: "lmt_11111".to_string(),
            size: Some(2000),                   // Increase size
            price: Some("43100.0".to_string()), // Better price
            amend_text: Some("Improve order".to_string()),
        };

        let json = serde_json::to_value(&improve_limit).unwrap();
        assert_eq!(json["size"], 2000);
        assert_eq!(json["price"], "43100.0");
    }

    #[test]
    fn test_amend_order_serialization_omits_null() {
        let request = AmendFuturesOrderRequest {
            settle: "USDT".to_string(),
            order_id: "12345".to_string(),
            size: None,
            price: None,
            amend_text: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();

        // Only settle and order_id should be present
        assert_eq!(obj.len(), 2);
        assert!(obj.contains_key("settle"));
        assert!(obj.contains_key("order_id"));
        assert!(!obj.contains_key("size"));
        assert!(!obj.contains_key("price"));
        assert!(!obj.contains_key("amend_text"));
    }
}
