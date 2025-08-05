use serde::Serialize;

use super::RestClient;
use crate::gateio::spot::private::rest::create_order::Order;

/// Cancel order request parameters
#[derive(Debug, Clone, Serialize)]
pub struct CancelOrderRequest {
    /// Currency pair
    pub currency_pair: String,

    /// Account type (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
}

impl RestClient {
    /// Cancel an order
    ///
    /// This endpoint cancels a specific order.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#cancel-a-single-order>
    pub async fn cancel_order(
        &self,
        order_id: &str,
        currency_pair: &str,
    ) -> crate::gateio::spot::RestResult<Order> {
        let query = CancelOrderRequest {
            currency_pair: currency_pair.to_string(),
            account: None,
        };
        let endpoint = format!("/spot/orders/{}", order_id);
        self.delete_with_query(&endpoint, &query).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_order_request_serialization() {
        let request = CancelOrderRequest {
            currency_pair: "BTC_USDT".to_string(),
            account: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "currency_pair=BTC_USDT");
    }

    #[test]
    fn test_cancel_order_request_with_account() {
        let request = CancelOrderRequest {
            currency_pair: "ETH_USDT".to_string(),
            account: Some("spot".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("currency_pair=ETH_USDT"));
        assert!(serialized.contains("account=spot"));
    }

    #[test]
    fn test_cancel_order_request_different_pairs() {
        let pairs = vec!["BTC_USDT", "ETH_BTC", "BNB_USDT", "SOL_USDC", "ADA_USDT"];

        for pair in pairs {
            let request = CancelOrderRequest {
                currency_pair: pair.to_string(),
                account: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("currency_pair={}", pair));
        }
    }

    #[test]
    fn test_cancel_order_request_different_accounts() {
        let accounts = vec!["spot", "margin", "cross_margin", "unified"];

        for account in accounts {
            let request = CancelOrderRequest {
                currency_pair: "BTC_USDT".to_string(),
                account: Some(account.to_string()),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("account={}", account)));
            assert!(serialized.contains("currency_pair=BTC_USDT"));
        }
    }

    #[test]
    fn test_cancel_order_request_json_serialization() {
        let request = CancelOrderRequest {
            currency_pair: "BTC_USDT".to_string(),
            account: Some("spot".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["account"], "spot");
    }

    #[test]
    fn test_cancel_order_request_json_serialization_no_account() {
        let request = CancelOrderRequest {
            currency_pair: "ETH_USDT".to_string(),
            account: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency_pair"], "ETH_USDT");
        assert!(!json.as_object().unwrap().contains_key("account"));
    }

    #[test]
    fn test_cancel_order_request_margin_account() {
        let request = CancelOrderRequest {
            currency_pair: "BTC_USDT".to_string(),
            account: Some("margin".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("account=margin"));
        assert!(serialized.contains("currency_pair=BTC_USDT"));
    }

    #[test]
    fn test_cancel_order_request_cross_margin_account() {
        let request = CancelOrderRequest {
            currency_pair: "ETH_BTC".to_string(),
            account: Some("cross_margin".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("account=cross_margin"));
        assert!(serialized.contains("currency_pair=ETH_BTC"));
    }

    #[test]
    fn test_cancel_order_request_unified_account() {
        let request = CancelOrderRequest {
            currency_pair: "SOL_USDT".to_string(),
            account: Some("unified".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("account=unified"));
        assert!(serialized.contains("currency_pair=SOL_USDT"));
    }

    #[test]
    fn test_cancel_order_request_special_currency_pairs() {
        let special_pairs = vec![
            "BTC3L_USDT", // Leveraged token
            "ETH3S_USDT", // Short token
            "wBTC_USDT",  // Wrapped token
            "USDC_USDT",  // Stablecoin pair
            "GT_USDT",    // Exchange token
        ];

        for pair in special_pairs {
            let request = CancelOrderRequest {
                currency_pair: pair.to_string(),
                account: Some("spot".to_string()),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("currency_pair={}", pair)));
            assert!(serialized.contains("account=spot"));
        }
    }

    #[test]
    fn test_cancel_order_endpoint_path_construction() {
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
    fn test_cancel_order_endpoint_with_special_order_ids() {
        let special_order_ids = vec![
            "0",
            "1",
            "123456789012345678", // Very long ID
            "order_12345",        // With prefix
            "abc123def456",       // Alphanumeric
        ];

        for order_id in special_order_ids {
            let endpoint = format!("/spot/orders/{}", order_id);
            assert!(endpoint.starts_with("/spot/orders/"));
            assert!(endpoint.contains(order_id));
        }
    }

    #[test]
    fn test_cancel_order_request_stablecoin_pairs() {
        let stablecoin_pairs = vec![
            "USDC_USDT",
            "BUSD_USDT",
            "DAI_USDT",
            "TUSD_USDT",
            "FRAX_USDT",
        ];

        for pair in stablecoin_pairs {
            let request = CancelOrderRequest {
                currency_pair: pair.to_string(),
                account: Some("spot".to_string()),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("currency_pair={}", pair)));
        }
    }

    #[test]
    fn test_cancel_order_request_realistic_scenarios() {
        // Scenario 1: Cancel spot order
        let spot_request = CancelOrderRequest {
            currency_pair: "BTC_USDT".to_string(),
            account: Some("spot".to_string()),
        };

        let spot_serialized = serde_urlencoded::to_string(&spot_request).unwrap();
        assert!(spot_serialized.contains("account=spot"));
        assert!(spot_serialized.contains("currency_pair=BTC_USDT"));

        // Scenario 2: Cancel margin order
        let margin_request = CancelOrderRequest {
            currency_pair: "ETH_BTC".to_string(),
            account: Some("margin".to_string()),
        };

        let margin_serialized = serde_urlencoded::to_string(&margin_request).unwrap();
        assert!(margin_serialized.contains("account=margin"));
        assert!(margin_serialized.contains("currency_pair=ETH_BTC"));

        // Scenario 3: Cancel with default account
        let default_request = CancelOrderRequest {
            currency_pair: "BNB_USDT".to_string(),
            account: None,
        };

        let default_serialized = serde_urlencoded::to_string(&default_request).unwrap();
        assert!(!default_serialized.contains("account="));
        assert!(default_serialized.contains("currency_pair=BNB_USDT"));
    }

    #[test]
    fn test_cancel_order_request_altcoin_pairs() {
        let altcoin_pairs = vec![
            "SOL_USDT",
            "DOT_USDT",
            "MATIC_USDT",
            "LINK_USDT",
            "ADA_USDT",
            "AVAX_USDT",
            "ATOM_USDT",
            "FTM_USDT",
        ];

        for pair in altcoin_pairs {
            let request = CancelOrderRequest {
                currency_pair: pair.to_string(),
                account: Some("spot".to_string()),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["currency_pair"], pair);
            assert_eq!(json["account"], "spot");
        }
    }

    #[test]
    fn test_cancel_order_request_cross_pairs() {
        let cross_pairs = vec![
            "ETH_BTC",
            "BNB_BTC",
            "ADA_BTC",
            "DOT_BTC",
            "SOL_ETH",
            "MATIC_ETH",
            "LINK_ETH",
        ];

        for pair in cross_pairs {
            let request = CancelOrderRequest {
                currency_pair: pair.to_string(),
                account: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("currency_pair={}", pair));
        }
    }

    #[test]
    fn test_cancel_order_request_serialization_order() {
        let request = CancelOrderRequest {
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
    fn test_cancel_order_request_empty_account() {
        let request = CancelOrderRequest {
            currency_pair: "BTC_USDT".to_string(),
            account: Some("".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("account="));
        assert!(serialized.contains("currency_pair=BTC_USDT"));
    }

    #[test]
    fn test_cancel_order_request_clone() {
        let original = CancelOrderRequest {
            currency_pair: "BTC_USDT".to_string(),
            account: Some("spot".to_string()),
        };

        let cloned = original.clone();
        assert_eq!(cloned.currency_pair, original.currency_pair);
        assert_eq!(cloned.account, original.account);
    }

    #[test]
    fn test_cancel_order_request_debug() {
        let request = CancelOrderRequest {
            currency_pair: "BTC_USDT".to_string(),
            account: Some("spot".to_string()),
        };

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("CancelOrderRequest"));
        assert!(debug_str.contains("BTC_USDT"));
        assert!(debug_str.contains("spot"));
    }

    #[test]
    fn test_cancel_order_request_long_currency_names() {
        let request = CancelOrderRequest {
            currency_pair: "VERYLONGTOKEN_ANOTHERLONGTOKEN".to_string(),
            account: Some("unified".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("currency_pair=VERYLONGTOKEN_ANOTHERLONGTOKEN"));
        assert!(serialized.contains("account=unified"));
    }

    #[test]
    fn test_cancel_order_request_numeric_tokens() {
        let numeric_pairs = vec!["1INCH_USDT", "3CRV_USDT", "404_USDT"];

        for pair in numeric_pairs {
            let request = CancelOrderRequest {
                currency_pair: pair.to_string(),
                account: Some("spot".to_string()),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("currency_pair={}", pair)));
        }
    }

    #[test]
    fn test_cancel_order_comprehensive_account_types() {
        let comprehensive_accounts = vec![
            ("spot", "BTC_USDT"),
            ("margin", "ETH_USDT"),
            ("cross_margin", "BNB_USDT"),
            ("unified", "SOL_USDT"),
        ];

        for (account, pair) in comprehensive_accounts {
            let request = CancelOrderRequest {
                currency_pair: pair.to_string(),
                account: Some(account.to_string()),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["currency_pair"], pair);
            assert_eq!(json["account"], account);
        }
    }
}
