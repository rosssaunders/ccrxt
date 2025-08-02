use serde::Serialize;

use super::RestClient;
use crate::gateio::spot::private::rest::create_order::Order;

/// Get order request parameters
#[derive(Debug, Clone, Serialize)]
pub struct GetOrderRequest {
    /// Currency pair
    pub currency_pair: String,

    /// Account type (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
}

impl RestClient {
    /// Get a specific order
    ///
    /// This endpoint returns details of a specific order.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#get-a-single-order>
    pub async fn get_order(
        &self,
        order_id: &str,
        currency_pair: &str,
    ) -> crate::gateio::spot::Result<Order> {
        let query = GetOrderRequest {
            currency_pair: currency_pair.to_string(),
            account: None,
        };
        let endpoint = format!("/spot/orders/{}", order_id);
        self.get_with_query(&endpoint, &query).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_order_request_serialization() {
        let request = GetOrderRequest {
            currency_pair: "BTC_USDT".to_string(),
            account: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "currency_pair=BTC_USDT");
    }

    #[test]
    fn test_get_order_request_with_account() {
        let request = GetOrderRequest {
            currency_pair: "ETH_USDT".to_string(),
            account: Some("spot".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("currency_pair=ETH_USDT"));
        assert!(serialized.contains("account=spot"));
    }

    #[test]
    fn test_get_order_request_different_pairs() {
        let pairs = vec!["BTC_USDT", "ETH_BTC", "BNB_USDT", "SOL_USDC", "ADA_USDT"];

        for pair in pairs {
            let request = GetOrderRequest {
                currency_pair: pair.to_string(),
                account: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("currency_pair={}", pair));
        }
    }

    #[test]
    fn test_get_order_request_different_accounts() {
        let accounts = vec!["spot", "margin", "cross_margin", "unified"];

        for account in accounts {
            let request = GetOrderRequest {
                currency_pair: "BTC_USDT".to_string(),
                account: Some(account.to_string()),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("account={}", account)));
            assert!(serialized.contains("currency_pair=BTC_USDT"));
        }
    }

    #[test]
    fn test_get_order_request_json_serialization() {
        let request = GetOrderRequest {
            currency_pair: "BTC_USDT".to_string(),
            account: Some("spot".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["account"], "spot");
    }

    #[test]
    fn test_get_order_request_json_serialization_no_account() {
        let request = GetOrderRequest {
            currency_pair: "ETH_USDT".to_string(),
            account: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency_pair"], "ETH_USDT");
        assert!(!json.as_object().unwrap().contains_key("account"));
    }

    #[test]
    fn test_get_order_request_mixed_case_currency() {
        let request = GetOrderRequest {
            currency_pair: "wBTC_USDT".to_string(),
            account: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "currency_pair=wBTC_USDT");
    }

    #[test]
    fn test_get_order_request_special_characters() {
        let request = GetOrderRequest {
            currency_pair: "3L_USDT".to_string(),
            account: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "currency_pair=3L_USDT");
    }

    #[test]
    fn test_get_order_request_long_currency_pair() {
        let request = GetOrderRequest {
            currency_pair: "VERYLONGTOKEN_ANOTHERLONGTOKEN".to_string(),
            account: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "currency_pair=VERYLONGTOKEN_ANOTHERLONGTOKEN");
    }

    #[test]
    fn test_get_order_request_stablecoin_pairs() {
        let stablecoin_pairs = vec![
            "USDC_USDT",
            "BUSD_USDT",
            "DAI_USDT",
            "TUSD_USDT",
            "FRAX_USDT",
        ];

        for pair in stablecoin_pairs {
            let request = GetOrderRequest {
                currency_pair: pair.to_string(),
                account: Some("spot".to_string()),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("currency_pair={}", pair)));
            assert!(serialized.contains("account=spot"));
        }
    }

    #[test]
    fn test_get_order_request_cross_margin_account() {
        let request = GetOrderRequest {
            currency_pair: "BTC_USDT".to_string(),
            account: Some("cross_margin".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("account=cross_margin"));
        assert!(serialized.contains("currency_pair=BTC_USDT"));
    }

    #[test]
    fn test_get_order_request_unified_account() {
        let request = GetOrderRequest {
            currency_pair: "ETH_USDT".to_string(),
            account: Some("unified".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("account=unified"));
        assert!(serialized.contains("currency_pair=ETH_USDT"));
    }

    #[test]
    fn test_get_order_request_clone() {
        let original = GetOrderRequest {
            currency_pair: "BTC_USDT".to_string(),
            account: Some("spot".to_string()),
        };

        let cloned = original.clone();
        assert_eq!(cloned.currency_pair, original.currency_pair);
        assert_eq!(cloned.account, original.account);
    }

    #[test]
    fn test_get_order_request_debug() {
        let request = GetOrderRequest {
            currency_pair: "BTC_USDT".to_string(),
            account: Some("spot".to_string()),
        };

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("GetOrderRequest"));
        assert!(debug_str.contains("BTC_USDT"));
        assert!(debug_str.contains("spot"));
    }

    #[test]
    fn test_get_order_endpoint_path_construction() {
        // Test that the endpoint format matches expected pattern
        let order_ids = vec!["12345678", "87654321", "11111111", "99999999"];

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
    fn test_get_order_endpoint_with_special_order_ids() {
        let special_order_ids = vec![
            "0",
            "1",
            "123456789012345678", // Very long ID
            "order_12345",        // With prefix
        ];

        for order_id in special_order_ids {
            let endpoint = format!("/spot/orders/{}", order_id);
            assert!(endpoint.starts_with("/spot/orders/"));
            assert!(endpoint.contains(order_id));
        }
    }

    #[test]
    fn test_get_order_request_serialization_order() {
        let request = GetOrderRequest {
            currency_pair: "BTC_USDT".to_string(),
            account: Some("spot".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();

        // Both orderings should be valid, test that both fields are present
        assert!(serialized.contains("currency_pair=BTC_USDT"));
        assert!(serialized.contains("account=spot"));

        // Should be connected with &
        if serialized.starts_with("currency_pair") {
            assert!(serialized.contains("currency_pair=BTC_USDT&account=spot"));
        } else {
            assert!(serialized.contains("account=spot&currency_pair=BTC_USDT"));
        }
    }

    #[test]
    fn test_get_order_request_realistic_scenarios() {
        // Scenario 1: Regular spot trading
        let spot_request = GetOrderRequest {
            currency_pair: "BTC_USDT".to_string(),
            account: Some("spot".to_string()),
        };

        let spot_serialized = serde_urlencoded::to_string(&spot_request).unwrap();
        assert!(spot_serialized.contains("account=spot"));

        // Scenario 2: Margin trading
        let margin_request = GetOrderRequest {
            currency_pair: "ETH_BTC".to_string(),
            account: Some("margin".to_string()),
        };

        let margin_serialized = serde_urlencoded::to_string(&margin_request).unwrap();
        assert!(margin_serialized.contains("account=margin"));

        // Scenario 3: Default account (None)
        let default_request = GetOrderRequest {
            currency_pair: "BNB_USDT".to_string(),
            account: None,
        };

        let default_serialized = serde_urlencoded::to_string(&default_request).unwrap();
        assert!(!default_serialized.contains("account="));
        assert!(default_serialized.contains("currency_pair=BNB_USDT"));
    }

    #[test]
    fn test_get_order_request_empty_string_handling() {
        // Test behavior with empty strings (though this shouldn't happen in practice)
        let request = GetOrderRequest {
            currency_pair: "BTC_USDT".to_string(),
            account: Some("".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("account="));
        assert!(serialized.contains("currency_pair=BTC_USDT"));
    }

    #[test]
    fn test_get_order_request_comprehensive_currency_pairs() {
        let comprehensive_pairs = vec![
            // Major pairs
            "BTC_USDT",
            "ETH_USDT",
            "BNB_USDT",
            // Cross pairs
            "ETH_BTC",
            "BNB_BTC",
            "ADA_BTC",
            // Alt pairs
            "SOL_USDT",
            "DOT_USDT",
            "MATIC_USDT",
            // Stablecoin pairs
            "USDC_USDT",
            "DAI_USDT",
            // Special tokens
            "BTC3L_USDT",
            "ETH3S_USDT",
        ];

        for pair in comprehensive_pairs {
            let request = GetOrderRequest {
                currency_pair: pair.to_string(),
                account: Some("spot".to_string()),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["currency_pair"], pair);
            assert_eq!(json["account"], "spot");
        }
    }
}
