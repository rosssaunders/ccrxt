use serde::{Deserialize, Serialize};

use super::{RestClient, order::FuturesOrder};

const ENDPOINT_FUTURES_ORDERS_PREFIX: &str = "/futures";

/// Request to create futures order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFuturesOrderRequest {
    /// Settlement currency
    pub settle: String,

    /// Contract name
    pub contract: String,

    /// Order size (positive for long, negative for short)
    pub size: i64,

    /// Order price (omit for market orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Time in force (gtc, ioc, poc, fok)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tif: Option<String>,

    /// Text label for order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Reduce only order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,

    /// Close position order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close: Option<bool>,

    /// Iceberg order amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg: Option<i64>,

    /// Auto size for closing position
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_size: Option<String>,
}

impl RestClient {
    /// Create a futures order
    ///
    /// This endpoint creates a new futures order for the authenticated user.
    ///
    /// See: Gate.io API documentation
    /// <https://www.gate.io/docs/developers/apiv4/#create-a-futures-order>
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The order creation request parameters
    ///
    /// # Returns
    /// Created order details
    pub async fn create_futures_order(
        &self,
        request: CreateFuturesOrderRequest,
    ) -> crate::gateio::perpetual::RestResult<FuturesOrder> {
        let endpoint = format!(
            "{}/{}/orders",
            ENDPOINT_FUTURES_ORDERS_PREFIX, request.settle
        );
        self.post(&endpoint, &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_futures_order_request_minimal() {
        let request = CreateFuturesOrderRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            size: 1000,
            price: None,
            tif: None,
            text: None,
            reduce_only: None,
            close: None,
            iceberg: None,
            auto_size: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT");
        assert_eq!(json["size"], 1000);

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 3); // Only required fields
    }

    #[test]
    fn test_create_futures_order_request_full() {
        let request = CreateFuturesOrderRequest {
            settle: "USDT".to_string(),
            contract: "ETH_USDT".to_string(),
            size: -2000, // Short position
            price: Some("2650.50".to_string()),
            tif: Some("gtc".to_string()),
            text: Some("test-order-123".to_string()),
            reduce_only: Some(false),
            close: Some(false),
            iceberg: Some(500),
            auto_size: Some("close_short".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "ETH_USDT");
        assert_eq!(json["size"], -2000);
        assert_eq!(json["price"], "2650.50");
        assert_eq!(json["tif"], "gtc");
        assert_eq!(json["text"], "test-order-123");
        assert_eq!(json["reduce_only"], false);
        assert_eq!(json["close"], false);
        assert_eq!(json["iceberg"], 500);
        assert_eq!(json["auto_size"], "close_short");
    }

    #[test]
    fn test_long_vs_short_positions() {
        let long_scenarios = vec![
            (1000, "Long 1000 contracts"),
            (5000, "Long 5000 contracts"),
            (10000, "Long 10000 contracts"),
        ];

        let short_scenarios = vec![
            (-1000, "Short 1000 contracts"),
            (-5000, "Short 5000 contracts"),
            (-10000, "Short 10000 contracts"),
        ];

        for (size, _description) in long_scenarios {
            let request = CreateFuturesOrderRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                size,
                price: Some("43250.0".to_string()),
                tif: None,
                text: None,
                reduce_only: None,
                close: None,
                iceberg: None,
                auto_size: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert!(json["size"].as_i64().unwrap() > 0);
        }

        for (size, _description) in short_scenarios {
            let request = CreateFuturesOrderRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                size,
                price: Some("43250.0".to_string()),
                tif: None,
                text: None,
                reduce_only: None,
                close: None,
                iceberg: None,
                auto_size: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert!(json["size"].as_i64().unwrap() < 0);
        }
    }

    #[test]
    fn test_order_types() {
        // Market order (no price)
        let market_order = CreateFuturesOrderRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            size: 1000,
            price: None,
            tif: Some("ioc".to_string()),
            text: None,
            reduce_only: None,
            close: None,
            iceberg: None,
            auto_size: None,
        };

        let json = serde_json::to_value(&market_order).unwrap();
        assert!(!json.as_object().unwrap().contains_key("price"));
        assert_eq!(json["tif"], "ioc");

        // Limit order (with price)
        let limit_order = CreateFuturesOrderRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            size: 1000,
            price: Some("43000.0".to_string()),
            tif: Some("gtc".to_string()),
            text: None,
            reduce_only: None,
            close: None,
            iceberg: None,
            auto_size: None,
        };

        let json = serde_json::to_value(&limit_order).unwrap();
        assert_eq!(json["price"], "43000.0");
        assert_eq!(json["tif"], "gtc");
    }

    #[test]
    fn test_time_in_force_options() {
        let tif_options = vec![
            ("gtc", "Good Till Cancelled"),
            ("ioc", "Immediate Or Cancel"),
            ("poc", "Post Only Crossing"),
            ("fok", "Fill Or Kill"),
        ];

        for (tif, _description) in tif_options {
            let request = CreateFuturesOrderRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                size: 1000,
                price: Some("43250.0".to_string()),
                tif: Some(tif.to_string()),
                text: None,
                reduce_only: None,
                close: None,
                iceberg: None,
                auto_size: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["tif"], tif);
        }
    }

    #[test]
    fn test_reduce_only_orders() {
        let request = CreateFuturesOrderRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            size: -1000,
            price: Some("43500.0".to_string()),
            tif: Some("gtc".to_string()),
            text: None,
            reduce_only: Some(true),
            close: None,
            iceberg: None,
            auto_size: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["reduce_only"], true);
    }

    #[test]
    fn test_close_position_orders() {
        let request = CreateFuturesOrderRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            size: 0, // Size 0 for close orders
            price: None,
            tif: Some("ioc".to_string()),
            text: None,
            reduce_only: None,
            close: Some(true),
            iceberg: None,
            auto_size: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["close"], true);
        assert_eq!(json["size"], 0);
    }

    #[test]
    fn test_iceberg_orders() {
        let request = CreateFuturesOrderRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            size: 10000,
            price: Some("43000.0".to_string()),
            tif: Some("gtc".to_string()),
            text: None,
            reduce_only: None,
            close: None,
            iceberg: Some(1000), // Show only 1000 contracts at a time
            auto_size: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["iceberg"], 1000);
        assert_eq!(json["size"], 10000);
    }

    #[test]
    fn test_auto_size_scenarios() {
        let auto_size_options = vec![
            ("close_long", "Close long position"),
            ("close_short", "Close short position"),
        ];

        for (auto_size, _description) in auto_size_options {
            let request = CreateFuturesOrderRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                size: 0, // Size typically 0 with auto_size
                price: None,
                tif: Some("ioc".to_string()),
                text: None,
                reduce_only: None,
                close: None,
                iceberg: None,
                auto_size: Some(auto_size.to_string()),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["auto_size"], auto_size);
        }
    }

    #[test]
    fn test_realistic_order_scenarios() {
        // Scenario 1: Market buy
        let market_buy = CreateFuturesOrderRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            size: 1000,
            price: None,
            tif: Some("ioc".to_string()),
            text: Some("market-buy".to_string()),
            reduce_only: None,
            close: None,
            iceberg: None,
            auto_size: None,
        };

        let json = serde_json::to_value(&market_buy).unwrap();
        assert!(!json.as_object().unwrap().contains_key("price"));
        assert_eq!(json["tif"], "ioc");

        // Scenario 2: Limit sell with reduce only
        let limit_sell_reduce = CreateFuturesOrderRequest {
            settle: "USDT".to_string(),
            contract: "ETH_USDT".to_string(),
            size: -2000,
            price: Some("2700.0".to_string()),
            tif: Some("gtc".to_string()),
            text: Some("take-profit".to_string()),
            reduce_only: Some(true),
            close: None,
            iceberg: None,
            auto_size: None,
        };

        let json = serde_json::to_value(&limit_sell_reduce).unwrap();
        assert_eq!(json["price"], "2700.0");
        assert_eq!(json["reduce_only"], true);
        assert!(json["size"].as_i64().unwrap() < 0);

        // Scenario 3: Close entire position
        let close_position = CreateFuturesOrderRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            size: 0,
            price: None,
            tif: Some("ioc".to_string()),
            text: Some("close-all".to_string()),
            reduce_only: None,
            close: Some(true),
            iceberg: None,
            auto_size: None,
        };

        let json = serde_json::to_value(&close_position).unwrap();
        assert_eq!(json["close"], true);
        assert_eq!(json["size"], 0);
    }

    #[test]
    fn test_order_request_serialization_omits_null() {
        let request = CreateFuturesOrderRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            size: 1000,
            price: None,
            tif: None,
            text: None,
            reduce_only: None,
            close: None,
            iceberg: None,
            auto_size: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();

        // Verify None values are not serialized
        assert!(!obj.contains_key("price"));
        assert!(!obj.contains_key("tif"));
        assert!(!obj.contains_key("text"));
        assert!(!obj.contains_key("reduce_only"));
        assert!(!obj.contains_key("close"));
        assert!(!obj.contains_key("iceberg"));
        assert!(!obj.contains_key("auto_size"));
    }
}
