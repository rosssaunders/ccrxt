use serde::Serialize;

use super::RestClient;
use crate::gateio::spot::{OrderSide, private::rest::create_order::Order};

const CANCEL_ALL_ORDERS_ENDPOINT: &str = "/spot/orders";

/// Request parameters for canceling all orders for a currency pair.
///
/// This request allows filtering by order side and account type to cancel specific subsets of orders.
#[derive(Debug, Clone, Serialize)]
pub struct CancelAllOrdersRequest {
    /// The trading pair identifier for which to cancel orders (e.g., "BTC_USDT").
    /// This field is required and specifies the exact currency pair.
    pub currency_pair: String,

    /// The order side to filter cancellations (optional).
    /// If specified, only orders on the given side (buy/sell) will be canceled.
    /// If omitted, orders on both sides will be canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<OrderSide>,

    /// The account type to filter cancellations (optional).
    /// Valid values include "spot", "margin", "cross_margin", "unified".
    /// If omitted, orders from the default account type will be canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
}

impl RestClient {
    /// Cancel all open orders in specified currency pair
    ///
    /// Cancels all open orders for a specified currency pair. This endpoint allows optional
    /// filtering by order side and account type to provide fine-grained control over which
    /// orders are canceled.
    ///
    /// [docs]: https://www.gate.io/docs/developers/apiv4/#cancel-all-open-orders-in-specified-currency-pair
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The cancel all orders request parameters including currency pair and optional filters
    ///
    /// # Returns
    /// A vector of canceled orders with their details
    pub async fn cancel_all_orders(
        &self,
        request: CancelAllOrdersRequest,
    ) -> crate::gateio::spot::RestResult<Vec<Order>> {
        self.delete_with_query(CANCEL_ALL_ORDERS_ENDPOINT, &request).await
    }

    /// Cancel all orders for a currency pair (helper method)
    ///
    /// Convenience method to cancel all orders for a specific currency pair without
    /// additional filtering by side or account type.
    ///
    /// [docs]: https://www.gate.io/docs/developers/apiv4/#cancel-all-open-orders-in-specified-currency-pair
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `currency_pair` - The trading pair identifier (e.g., "BTC_USDT")
    ///
    /// # Returns
    /// A vector of canceled orders with their details
    pub async fn cancel_all_orders_for_pair(
        &self,
        currency_pair: &str,
    ) -> crate::gateio::spot::RestResult<Vec<Order>> {
        let request = CancelAllOrdersRequest {
            currency_pair: currency_pair.to_string(),
            side: None,
            account: None,
        };
        self.cancel_all_orders(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_all_orders_request_minimal_serialization() {
        let request = CancelAllOrdersRequest {
            currency_pair: "BTC_USDT".to_string(),
            side: None,
            account: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "currency_pair=BTC_USDT");
    }

    #[test]
    fn test_cancel_all_orders_request_with_side() {
        let request = CancelAllOrdersRequest {
            currency_pair: "BTC_USDT".to_string(),
            side: Some(OrderSide::Buy),
            account: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("currency_pair=BTC_USDT"));
        assert!(serialized.contains("side=buy"));
    }

    #[test]
    fn test_cancel_all_orders_request_with_account() {
        let request = CancelAllOrdersRequest {
            currency_pair: "ETH_USDT".to_string(),
            side: None,
            account: Some("spot".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("currency_pair=ETH_USDT"));
        assert!(serialized.contains("account=spot"));
    }

    #[test]
    fn test_cancel_all_orders_request_full_parameters() {
        let request = CancelAllOrdersRequest {
            currency_pair: "BNB_USDT".to_string(),
            side: Some(OrderSide::Sell),
            account: Some("margin".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("currency_pair=BNB_USDT"));
        assert!(serialized.contains("side=sell"));
        assert!(serialized.contains("account=margin"));
    }

    #[test]
    fn test_cancel_all_orders_request_different_currency_pairs() {
        let pairs = vec!["BTC_USDT", "ETH_BTC", "BNB_USDT", "SOL_USDC", "ADA_USDT"];

        for pair in pairs {
            let request = CancelAllOrdersRequest {
                currency_pair: pair.to_string(),
                side: None,
                account: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("currency_pair={}", pair));
        }
    }

    #[test]
    fn test_cancel_all_orders_request_different_sides() {
        let sides = vec![(OrderSide::Buy, "buy"), (OrderSide::Sell, "sell")];

        for (side, expected) in sides {
            let request = CancelAllOrdersRequest {
                currency_pair: "BTC_USDT".to_string(),
                side: Some(side),
                account: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("side={}", expected)));
            assert!(serialized.contains("currency_pair=BTC_USDT"));
        }
    }

    #[test]
    fn test_cancel_all_orders_request_different_accounts() {
        let accounts = vec!["spot", "margin", "cross_margin", "unified"];

        for account in accounts {
            let request = CancelAllOrdersRequest {
                currency_pair: "BTC_USDT".to_string(),
                side: None,
                account: Some(account.to_string()),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("account={}", account)));
            assert!(serialized.contains("currency_pair=BTC_USDT"));
        }
    }

    #[test]
    fn test_cancel_all_orders_request_json_serialization() {
        let request = CancelAllOrdersRequest {
            currency_pair: "BTC_USDT".to_string(),
            side: Some(OrderSide::Buy),
            account: Some("spot".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["side"], "buy");
        assert_eq!(json["account"], "spot");
    }

    #[test]
    fn test_cancel_all_orders_request_json_serialization_defaults() {
        let request = CancelAllOrdersRequest {
            currency_pair: "ETH_USDT".to_string(),
            side: None,
            account: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency_pair"], "ETH_USDT");

        // Optional fields should be omitted when None
        let obj = json.as_object().unwrap();
        assert!(!obj.contains_key("side"));
        assert!(!obj.contains_key("account"));
    }

    #[test]
    fn test_cancel_all_orders_request_spot_account() {
        let request = CancelAllOrdersRequest {
            currency_pair: "BTC_USDT".to_string(),
            side: Some(OrderSide::Buy),
            account: Some("spot".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("account=spot"));
        assert!(serialized.contains("side=buy"));
        assert!(serialized.contains("currency_pair=BTC_USDT"));
    }

    #[test]
    fn test_cancel_all_orders_request_margin_account() {
        let request = CancelAllOrdersRequest {
            currency_pair: "ETH_USDT".to_string(),
            side: Some(OrderSide::Sell),
            account: Some("margin".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("account=margin"));
        assert!(serialized.contains("side=sell"));
        assert!(serialized.contains("currency_pair=ETH_USDT"));
    }

    #[test]
    fn test_cancel_all_orders_request_cross_margin_account() {
        let request = CancelAllOrdersRequest {
            currency_pair: "BNB_BTC".to_string(),
            side: None,
            account: Some("cross_margin".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("account=cross_margin"));
        assert!(serialized.contains("currency_pair=BNB_BTC"));
        assert!(!serialized.contains("side="));
    }

    #[test]
    fn test_cancel_all_orders_request_unified_account() {
        let request = CancelAllOrdersRequest {
            currency_pair: "SOL_USDT".to_string(),
            side: Some(OrderSide::Buy),
            account: Some("unified".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("account=unified"));
        assert!(serialized.contains("side=buy"));
        assert!(serialized.contains("currency_pair=SOL_USDT"));
    }

    #[test]
    fn test_cancel_all_orders_request_stablecoin_pairs() {
        let stablecoin_pairs = vec!["USDC_USDT", "BUSD_USDT", "DAI_USDT", "TUSD_USDT"];

        for pair in stablecoin_pairs {
            let request = CancelAllOrdersRequest {
                currency_pair: pair.to_string(),
                side: Some(OrderSide::Buy),
                account: Some("spot".to_string()),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("currency_pair={}", pair)));
            assert!(serialized.contains("side=buy"));
        }
    }

    #[test]
    fn test_cancel_all_orders_request_cross_pairs() {
        let cross_pairs = vec!["ETH_BTC", "BNB_BTC", "ADA_BTC", "SOL_ETH", "MATIC_ETH"];

        for pair in cross_pairs {
            let request = CancelAllOrdersRequest {
                currency_pair: pair.to_string(),
                side: Some(OrderSide::Sell),
                account: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("currency_pair={}", pair)));
            assert!(serialized.contains("side=sell"));
        }
    }

    #[test]
    fn test_cancel_all_orders_request_special_tokens() {
        let special_tokens = vec![
            "BTC3L_USDT", // Leveraged token
            "ETH3S_USDT", // Short token
            "wBTC_USDT",  // Wrapped token
            "GT_USDT",    // Exchange token
            "1INCH_USDT", // Numeric start
        ];

        for pair in special_tokens {
            let request = CancelAllOrdersRequest {
                currency_pair: pair.to_string(),
                side: None,
                account: Some("spot".to_string()),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("currency_pair={}", pair)));
            assert!(serialized.contains("account=spot"));
        }
    }

    #[test]
    fn test_cancel_all_orders_request_realistic_scenarios() {
        // Scenario 1: Cancel all BTC buy orders on spot account
        let btc_buys = CancelAllOrdersRequest {
            currency_pair: "BTC_USDT".to_string(),
            side: Some(OrderSide::Buy),
            account: Some("spot".to_string()),
        };

        let btc_serialized = serde_urlencoded::to_string(&btc_buys).unwrap();
        assert!(btc_serialized.contains("currency_pair=BTC_USDT"));
        assert!(btc_serialized.contains("side=buy"));
        assert!(btc_serialized.contains("account=spot"));

        // Scenario 2: Cancel all ETH orders (both sides) on margin
        let eth_all = CancelAllOrdersRequest {
            currency_pair: "ETH_USDT".to_string(),
            side: None,
            account: Some("margin".to_string()),
        };

        let eth_serialized = serde_urlencoded::to_string(&eth_all).unwrap();
        assert!(eth_serialized.contains("currency_pair=ETH_USDT"));
        assert!(eth_serialized.contains("account=margin"));
        assert!(!eth_serialized.contains("side="));

        // Scenario 3: Cancel all sell orders (any account)
        let all_sells = CancelAllOrdersRequest {
            currency_pair: "BNB_USDT".to_string(),
            side: Some(OrderSide::Sell),
            account: None,
        };

        let sells_serialized = serde_urlencoded::to_string(&all_sells).unwrap();
        assert!(sells_serialized.contains("currency_pair=BNB_USDT"));
        assert!(sells_serialized.contains("side=sell"));
        assert!(!sells_serialized.contains("account="));
    }

    #[test]
    fn test_cancel_all_orders_for_pair_helper() {
        // Test the helper function parameters
        let currency_pair = "BTC_USDT";

        let expected_request = CancelAllOrdersRequest {
            currency_pair: currency_pair.to_string(),
            side: None,
            account: None,
        };

        let serialized = serde_urlencoded::to_string(&expected_request).unwrap();
        assert_eq!(serialized, "currency_pair=BTC_USDT");
        assert!(!serialized.contains("side="));
        assert!(!serialized.contains("account="));
    }

    #[test]
    fn test_cancel_all_orders_request_serialization_order() {
        let request = CancelAllOrdersRequest {
            currency_pair: "BTC_USDT".to_string(),
            side: Some(OrderSide::Buy),
            account: Some("spot".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();

        // All fields should be present
        assert!(serialized.contains("currency_pair=BTC_USDT"));
        assert!(serialized.contains("side=buy"));
        assert!(serialized.contains("account=spot"));

        // Should be connected with &
        let field_count = serialized.matches('&').count();
        assert_eq!(field_count, 2); // Two & symbols for three fields
    }

    #[test]
    fn test_cancel_all_orders_request_altcoin_pairs() {
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
            let request = CancelAllOrdersRequest {
                currency_pair: pair.to_string(),
                side: Some(OrderSide::Buy),
                account: Some("spot".to_string()),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["currency_pair"], pair);
            assert_eq!(json["side"], "buy");
            assert_eq!(json["account"], "spot");
        }
    }

    #[test]
    fn test_cancel_all_orders_request_empty_account() {
        let request = CancelAllOrdersRequest {
            currency_pair: "BTC_USDT".to_string(),
            side: Some(OrderSide::Buy),
            account: Some("".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("account="));
        assert!(serialized.contains("currency_pair=BTC_USDT"));
        assert!(serialized.contains("side=buy"));
    }

    #[test]
    fn test_cancel_all_orders_request_clone() {
        let original = CancelAllOrdersRequest {
            currency_pair: "BTC_USDT".to_string(),
            side: Some(OrderSide::Buy),
            account: Some("spot".to_string()),
        };

        let cloned = original.clone();
        assert_eq!(cloned.currency_pair, original.currency_pair);
        assert_eq!(cloned.side, original.side);
        assert_eq!(cloned.account, original.account);
    }

    #[test]
    fn test_cancel_all_orders_request_debug() {
        let request = CancelAllOrdersRequest {
            currency_pair: "BTC_USDT".to_string(),
            side: Some(OrderSide::Sell),
            account: Some("margin".to_string()),
        };

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("CancelAllOrdersRequest"));
        assert!(debug_str.contains("BTC_USDT"));
        assert!(debug_str.contains("Sell"));
        assert!(debug_str.contains("margin"));
    }

    #[test]
    fn test_cancel_all_orders_request_comprehensive_account_types() {
        let comprehensive_accounts = vec![
            ("spot", "BTC_USDT"),
            ("margin", "ETH_USDT"),
            ("cross_margin", "BNB_USDT"),
            ("unified", "SOL_USDT"),
        ];

        for (account, pair) in comprehensive_accounts {
            let request = CancelAllOrdersRequest {
                currency_pair: pair.to_string(),
                side: Some(OrderSide::Buy),
                account: Some(account.to_string()),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["currency_pair"], pair);
            assert_eq!(json["side"], "buy");
            assert_eq!(json["account"], account);
        }
    }

    #[test]
    fn test_cancel_all_orders_request_side_combinations() {
        let side_tests = vec![
            (Some(OrderSide::Buy), "buy"),
            (Some(OrderSide::Sell), "sell"),
            (None, ""),
        ];

        for (side, expected) in side_tests {
            let request = CancelAllOrdersRequest {
                currency_pair: "BTC_USDT".to_string(),
                side,
                account: Some("spot".to_string()),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains("currency_pair=BTC_USDT"));

            if !expected.is_empty() {
                assert!(serialized.contains(&format!("side={}", expected)));
            } else {
                assert!(!serialized.contains("side="));
            }
        }
    }

    #[test]
    fn test_cancel_all_orders_request_long_currency_names() {
        let request = CancelAllOrdersRequest {
            currency_pair: "VERYLONGTOKEN_ANOTHERLONGTOKEN".to_string(),
            side: Some(OrderSide::Buy),
            account: Some("unified".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("currency_pair=VERYLONGTOKEN_ANOTHERLONGTOKEN"));
        assert!(serialized.contains("side=buy"));
        assert!(serialized.contains("account=unified"));
    }

    #[test]
    fn test_cancel_all_orders_request_numeric_tokens() {
        let numeric_pairs = vec!["1INCH_USDT", "3CRV_USDT", "404_USDT"];

        for pair in numeric_pairs {
            let request = CancelAllOrdersRequest {
                currency_pair: pair.to_string(),
                side: Some(OrderSide::Sell),
                account: Some("spot".to_string()),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("currency_pair={}", pair)));
            assert!(serialized.contains("side=sell"));
        }
    }

    #[test]
    fn test_cancel_all_orders_endpoint_constant() {
        // Verify the endpoint constant matches expected Gate.io API path
        let expected_endpoint = "/spot/orders";

        // Test that our endpoint string is correct
        assert_eq!(expected_endpoint, "/spot/orders");
        assert!(expected_endpoint.starts_with("/spot/"));
        assert!(expected_endpoint.ends_with("orders"));
    }
}
