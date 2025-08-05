use serde::Serialize;

use super::RestClient;
use crate::gateio::spot::private::rest::create_order::Order;

const AMEND_ORDER_ENDPOINT: &str = "/spot/orders";

/// Request parameters for amending an order.
#[derive(Debug, Clone, Serialize)]
pub struct AmendOrderRequest {
    /// New order amount to be set for this order.
    /// Must be greater than zero if provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,

    /// New order price to be set for this order.
    /// Must be greater than zero if provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Custom amendment text for tracking purposes.
    /// Free text field for client use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amend_text: Option<String>,
}

impl RestClient {
    /// Amend an order
    ///
    /// Modifies the price and/or amount of an existing order.
    /// At least one of amount or price must be provided in the amendment.
    ///
    /// [docs]: https://www.gate.com/docs/developers/apiv4/#amend-an-order
    ///
    /// Rate limit: 100 requests per 2 seconds
    ///
    /// # Arguments
    /// * `order_id` - The order ID to be amended
    /// * `currency_pair` - Trading pair symbol (e.g., "BTC_USDT")
    /// * `amendment` - The order amendment parameters
    ///
    /// # Returns
    /// Updated order information after amendment
    pub async fn amend_order(
        &self,
        order_id: &str,
        currency_pair: &str,
        amendment: AmendOrderRequest,
    ) -> crate::gateio::spot::RestResult<Order> {
        let endpoint = format!("{}/{}", AMEND_ORDER_ENDPOINT, order_id);
        #[allow(clippy::unwrap_used)]
        let mut body = serde_json::to_value(&amendment).unwrap();
        #[allow(clippy::indexing_slicing)]
        {
            body["currency_pair"] = serde_json::Value::String(currency_pair.to_string());
        }

        self.patch(&endpoint, &body).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amend_order_request_minimal_serialization() {
        let request = AmendOrderRequest {
            amount: None,
            price: None,
            amend_text: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_amend_order_request_with_amount() {
        let request = AmendOrderRequest {
            amount: Some("0.5".to_string()),
            price: None,
            amend_text: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "amount=0.5");
    }

    #[test]
    fn test_amend_order_request_with_price() {
        let request = AmendOrderRequest {
            amount: None,
            price: Some("30000".to_string()),
            amend_text: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "price=30000");
    }

    #[test]
    fn test_amend_order_request_with_amend_text() {
        let request = AmendOrderRequest {
            amount: None,
            price: None,
            amend_text: Some("update_amount".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "amend_text=update_amount");
    }

    #[test]
    fn test_amend_order_request_amount_and_price() {
        let request = AmendOrderRequest {
            amount: Some("1.0".to_string()),
            price: Some("25000".to_string()),
            amend_text: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("amount=1.0"));
        assert!(serialized.contains("price=25000"));
    }

    #[test]
    fn test_amend_order_request_full_parameters() {
        let request = AmendOrderRequest {
            amount: Some("2.5".to_string()),
            price: Some("28000".to_string()),
            amend_text: Some("price_amount_update".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("amount=2.5"));
        assert!(serialized.contains("price=28000"));
        assert!(serialized.contains("amend_text=price_amount_update"));
    }

    #[test]
    fn test_amend_order_request_json_serialization() {
        let request = AmendOrderRequest {
            amount: Some("1.5".to_string()),
            price: Some("32000".to_string()),
            amend_text: Some("update_order".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["amount"], "1.5");
        assert_eq!(json["price"], "32000");
        assert_eq!(json["amend_text"], "update_order");
    }

    #[test]
    fn test_amend_order_request_json_serialization_defaults() {
        let request = AmendOrderRequest {
            amount: None,
            price: None,
            amend_text: None,
        };

        let json = serde_json::to_value(&request).unwrap();

        // Optional fields should be omitted when None
        let obj = json.as_object().unwrap();
        assert!(!obj.contains_key("amount"));
        assert!(!obj.contains_key("price"));
        assert!(!obj.contains_key("amend_text"));
    }

    #[test]
    fn test_amend_order_request_different_amounts() {
        let amounts = vec!["0.001", "0.1", "1.0", "10.5", "100.123456"];

        for amount in amounts {
            let request = AmendOrderRequest {
                amount: Some(amount.to_string()),
                price: None,
                amend_text: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("amount={}", amount));
        }
    }

    #[test]
    fn test_amend_order_request_different_prices() {
        let prices = vec!["100", "1000.5", "30000", "50000.123", "999999.9999"];

        for price in prices {
            let request = AmendOrderRequest {
                amount: None,
                price: Some(price.to_string()),
                amend_text: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("price={}", price));
        }
    }

    #[test]
    fn test_amend_order_request_different_amend_texts() {
        let texts = vec![
            "increase_amount",
            "decrease_price",
            "urgent_update",
            "client_modification",
            "algo_adjustment",
        ];

        for text in texts {
            let request = AmendOrderRequest {
                amount: None,
                price: None,
                amend_text: Some(text.to_string()),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("amend_text={}", text));
        }
    }

    #[test]
    fn test_amend_order_request_precision_amounts() {
        let precision_amounts = vec![
            "0.00000001", // 8 decimal places
            "1.12345678", // Max precision BTC
            "0.123456",   // 6 decimal places
            "1000.00001", // Large amount with precision
        ];

        for amount in precision_amounts {
            let request = AmendOrderRequest {
                amount: Some(amount.to_string()),
                price: Some("30000".to_string()),
                amend_text: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["amount"], amount);
            assert_eq!(json["price"], "30000");
        }
    }

    #[test]
    fn test_amend_order_request_precision_prices() {
        let precision_prices = vec![
            "30000.01",     // 2 decimal places
            "30000.12345",  // 5 decimal places
            "1.00000001",   // Small price with precision
            "99999.999999", // High precision
        ];

        for price in precision_prices {
            let request = AmendOrderRequest {
                amount: Some("1.0".to_string()),
                price: Some(price.to_string()),
                amend_text: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["amount"], "1.0");
            assert_eq!(json["price"], price);
        }
    }

    #[test]
    fn test_amend_order_request_zero_values() {
        // Test zero amount
        let zero_amount_request = AmendOrderRequest {
            amount: Some("0".to_string()),
            price: Some("30000".to_string()),
            amend_text: None,
        };

        let serialized = serde_urlencoded::to_string(&zero_amount_request).unwrap();
        assert!(serialized.contains("amount=0"));
        assert!(serialized.contains("price=30000"));

        // Test zero price (unlikely but valid for testing)
        let zero_price_request = AmendOrderRequest {
            amount: Some("1.0".to_string()),
            price: Some("0".to_string()),
            amend_text: None,
        };

        let serialized2 = serde_urlencoded::to_string(&zero_price_request).unwrap();
        assert!(serialized2.contains("amount=1.0"));
        assert!(serialized2.contains("price=0"));
    }

    #[test]
    fn test_amend_order_request_large_values() {
        let request = AmendOrderRequest {
            amount: Some("999999999.99999999".to_string()),
            price: Some("999999999.99".to_string()),
            amend_text: Some("large_value_update".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["amount"], "999999999.99999999");
        assert_eq!(json["price"], "999999999.99");
        assert_eq!(json["amend_text"], "large_value_update");
    }

    #[test]
    fn test_amend_order_request_empty_strings() {
        let request = AmendOrderRequest {
            amount: Some("".to_string()),
            price: Some("".to_string()),
            amend_text: Some("".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("amount="));
        assert!(serialized.contains("price="));
        assert!(serialized.contains("amend_text="));
    }

    #[test]
    fn test_amend_order_request_special_characters_in_text() {
        let special_texts = vec![
            "update-order",
            "update_order_2024",
            "order@modification",
            "urgent!update",
            "order#123",
        ];

        for text in special_texts {
            let request = AmendOrderRequest {
                amount: Some("1.0".to_string()),
                price: None,
                amend_text: Some(text.to_string()),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["amend_text"], text);
        }
    }

    #[test]
    fn test_amend_order_request_realistic_scenarios() {
        // Scenario 1: Increase amount only
        let increase_amount = AmendOrderRequest {
            amount: Some("2.0".to_string()),
            price: None,
            amend_text: Some("increase_position".to_string()),
        };

        let serialized1 = serde_urlencoded::to_string(&increase_amount).unwrap();
        assert!(serialized1.contains("amount=2.0"));
        assert!(serialized1.contains("amend_text=increase_position"));
        assert!(!serialized1.contains("price="));

        // Scenario 2: Reduce price only
        let reduce_price = AmendOrderRequest {
            amount: None,
            price: Some("29000".to_string()),
            amend_text: Some("better_price".to_string()),
        };

        let serialized2 = serde_urlencoded::to_string(&reduce_price).unwrap();
        assert!(serialized2.contains("price=29000"));
        assert!(serialized2.contains("amend_text=better_price"));
        assert!(!serialized2.contains("amount="));

        // Scenario 3: Update both amount and price
        let update_both = AmendOrderRequest {
            amount: Some("0.5".to_string()),
            price: Some("31000".to_string()),
            amend_text: Some("full_update".to_string()),
        };

        let serialized3 = serde_urlencoded::to_string(&update_both).unwrap();
        assert!(serialized3.contains("amount=0.5"));
        assert!(serialized3.contains("price=31000"));
        assert!(serialized3.contains("amend_text=full_update"));
    }

    #[test]
    fn test_amend_order_endpoint_path_construction() {
        let order_ids = vec!["12345678", "87654321", "11111111", "order_abc123"];

        for order_id in order_ids {
            let expected_endpoint = format!("/spot/orders/{}", order_id);

            // Verify the path construction logic
            assert!(expected_endpoint.starts_with("/spot/orders/"));
            assert!(expected_endpoint.ends_with(order_id));
            assert_eq!(
                expected_endpoint.len(),
                "/spot/orders/".len() + order_id.len()
            );
        }
    }

    #[test]
    fn test_amend_order_body_construction() {
        let amendment = AmendOrderRequest {
            amount: Some("1.5".to_string()),
            price: Some("32000".to_string()),
            amend_text: Some("test_update".to_string()),
        };

        let currency_pair = "BTC_USDT";

        // Simulate the body construction logic
        let mut body = serde_json::to_value(&amendment).unwrap();
        body["currency_pair"] = serde_json::Value::String(currency_pair.to_string());

        // Verify the body contains all expected fields
        assert_eq!(body["amount"], "1.5");
        assert_eq!(body["price"], "32000");
        assert_eq!(body["amend_text"], "test_update");
        assert_eq!(body["currency_pair"], "BTC_USDT");
    }

    #[test]
    fn test_amend_order_body_with_minimal_amendment() {
        let amendment = AmendOrderRequest {
            amount: Some("0.1".to_string()),
            price: None,
            amend_text: None,
        };

        let currency_pair = "ETH_USDT";

        // Simulate the body construction logic
        let mut body = serde_json::to_value(&amendment).unwrap();
        body["currency_pair"] = serde_json::Value::String(currency_pair.to_string());

        // Verify the body contains expected fields
        assert_eq!(body["amount"], "0.1");
        assert_eq!(body["currency_pair"], "ETH_USDT");

        // Verify optional fields are not present
        let obj = body.as_object().unwrap();
        assert!(!obj.contains_key("price"));
        assert!(!obj.contains_key("amend_text"));
    }

    #[test]
    fn test_amend_order_request_clone() {
        let original = AmendOrderRequest {
            amount: Some("1.0".to_string()),
            price: Some("30000".to_string()),
            amend_text: Some("test".to_string()),
        };

        let cloned = original.clone();
        assert_eq!(cloned.amount, original.amount);
        assert_eq!(cloned.price, original.price);
        assert_eq!(cloned.amend_text, original.amend_text);
    }

    #[test]
    fn test_amend_order_request_debug() {
        let request = AmendOrderRequest {
            amount: Some("2.0".to_string()),
            price: Some("35000".to_string()),
            amend_text: Some("debug_test".to_string()),
        };

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("AmendOrderRequest"));
        assert!(debug_str.contains("2.0"));
        assert!(debug_str.contains("35000"));
        assert!(debug_str.contains("debug_test"));
    }

    #[test]
    fn test_amend_order_request_stablecoin_scenarios() {
        // Stablecoin pairs typically have tighter spreads
        let stablecoin_amendment = AmendOrderRequest {
            amount: Some("1000.0".to_string()),
            price: Some("1.0001".to_string()), // Typical stablecoin spread
            amend_text: Some("stablecoin_arb".to_string()),
        };

        let json = serde_json::to_value(&stablecoin_amendment).unwrap();
        assert_eq!(json["amount"], "1000.0");
        assert_eq!(json["price"], "1.0001");

        let price: f64 = json["price"].as_str().unwrap().parse().unwrap();
        assert!(price > 0.999 && price < 1.001); // Should be close to 1.0
    }

    #[test]
    fn test_amend_order_request_altcoin_scenarios() {
        let altcoin_pairs = vec![
            ("SOL_USDT", "150.50", "5.0"),
            ("DOT_USDT", "25.123", "10.5"),
            ("MATIC_USDT", "0.8765", "100.0"),
            ("LINK_USDT", "18.99", "15.25"),
        ];

        for (pair, price, amount) in altcoin_pairs {
            let request = AmendOrderRequest {
                amount: Some(amount.to_string()),
                price: Some(price.to_string()),
                amend_text: Some(format!("update_{}", pair.split('_').next().unwrap())),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["amount"], amount);
            assert_eq!(json["price"], price);
        }
    }

    #[test]
    fn test_amend_order_request_serialization_order() {
        let request = AmendOrderRequest {
            amount: Some("1.0".to_string()),
            price: Some("30000".to_string()),
            amend_text: Some("test".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();

        // All fields should be present
        assert!(serialized.contains("amount=1.0"));
        assert!(serialized.contains("price=30000"));
        assert!(serialized.contains("amend_text=test"));

        // Should be connected with &
        let field_count = serialized.matches('&').count();
        assert_eq!(field_count, 2); // Two & symbols for three fields
    }

    #[test]
    fn test_amend_order_request_partial_updates() {
        // Test all possible combinations of partial updates
        let partial_tests = vec![
            (Some("1.0"), None, None),                         // Amount only
            (None, Some("30000"), None),                       // Price only
            (None, None, Some("text_only")),                   // Text only
            (Some("1.0"), Some("30000"), None),                // Amount + Price
            (Some("1.0"), None, Some("amount_text")),          // Amount + Text
            (None, Some("30000"), Some("price_text")),         // Price + Text
            (Some("1.0"), Some("30000"), Some("full_update")), // All fields
        ];

        for (amount, price, text) in partial_tests {
            let request = AmendOrderRequest {
                amount: amount.map(|s| s.to_string()),
                price: price.map(|s| s.to_string()),
                amend_text: text.map(|s| s.to_string()),
            };

            let json = serde_json::to_value(&request).unwrap();
            let obj = json.as_object().unwrap();

            // Verify only provided fields are present
            assert_eq!(obj.contains_key("amount"), amount.is_some());
            assert_eq!(obj.contains_key("price"), price.is_some());
            assert_eq!(obj.contains_key("amend_text"), text.is_some());
        }
    }

    #[test]
    fn test_amend_order_request_currency_pair_scenarios() {
        let currency_pairs = vec![
            "BTC_USDT",
            "ETH_BTC",
            "BNB_USDT",
            "SOL_USDC",
            "USDC_USDT",
            "BTC3L_USDT",
            "ETH3S_USDT",
        ];

        for pair in currency_pairs {
            let amendment = AmendOrderRequest {
                amount: Some("1.0".to_string()),
                price: Some("100".to_string()),
                amend_text: Some("test".to_string()),
            };

            // Simulate body construction with different currency pairs
            let mut body = serde_json::to_value(&amendment).unwrap();
            body["currency_pair"] = serde_json::Value::String(pair.to_string());

            assert_eq!(body["currency_pair"], pair);
            assert_eq!(body["amount"], "1.0");
        }
    }
}
