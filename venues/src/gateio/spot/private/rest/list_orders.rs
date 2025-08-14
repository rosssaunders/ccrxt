use serde::Serialize;

use super::RestClient;
use crate::gateio::spot::{OrderSide, OrderStatus, private::rest::create_order::Order};

const LIST_ORDERS_ENDPOINT: &str = "/spot/orders";

/// Request parameters for listing historical and current orders.
///
/// Used to retrieve order history with comprehensive filtering options including
/// currency pair, order status, time ranges, and account types. Supports pagination
/// for efficient handling of large order histories and trading records.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListOrdersRequest {
    /// Trading pair filter for order history query.
    ///
    /// Optional filter to retrieve orders for a specific currency pair.
    /// Format should be "BASE_QUOTE" (e.g., "BTC_USDT", "ETH_BTC"). If not specified,
    /// returns orders for all trading pairs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,

    /// Order status filter for the query.
    ///
    /// Filters orders by their current status. Options include open, closed, or cancelled.
    /// If not specified, returns orders of all statuses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatus>,

    /// Page number for pagination (starting from 1).
    ///
    /// Used for paginated results when there are many orders in the history.
    /// Default is 1 if not specified. Page numbers start from 1, not 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Maximum number of orders to return per page.
    ///
    /// Controls the number of orders returned in a single response. Larger limits
    /// may improve efficiency but could increase response times. Typical range is 1-1000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Account type filter for the order query.
    ///
    /// Specifies which account type to query orders from. Common values include
    /// "spot", "margin", "cross_margin", or "unified". If not specified, uses the
    /// default account context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// Start time filter as Unix timestamp.
    ///
    /// Filters orders created on or after this timestamp. Used in combination with
    /// 'to' parameter to define time ranges for historical order queries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time filter as Unix timestamp.
    ///
    /// Filters orders created on or before this timestamp. Used in combination with
    /// 'from' parameter to define time ranges for historical order queries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Trading side filter for the order query.
    ///
    /// Filters orders by trading direction. Options are buy or sell orders.
    /// If not specified, returns orders for both sides.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<OrderSide>,
}

impl RestClient {
    /// List orders
    ///
    /// Retrieves a comprehensive list of historical and current orders with advanced filtering
    /// capabilities. This endpoint supports querying orders by currency pair, status, time ranges,
    /// and account types, making it essential for order management, trade history analysis, and
    /// reconciliation processes. Results are paginated for efficient handling of large datasets.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#list-orders)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - Request parameters for filtering and pagination of order history
    ///
    /// # Returns
    /// List of orders matching the specified criteria with detailed execution information
    pub async fn list_orders(
        &self,
        request: ListOrdersRequest,
    ) -> crate::gateio::spot::RestResult<Vec<Order>> {
        self.get_with_query(LIST_ORDERS_ENDPOINT, &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_orders_request_minimal_serialization() {
        let request = ListOrdersRequest::default();

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_list_orders_request_with_currency_pair() {
        let request = ListOrdersRequest {
            currency_pair: Some("BTC_USDT".to_string()),
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "currency_pair=BTC_USDT");
    }

    #[test]
    fn test_list_orders_request_with_status() {
        let request = ListOrdersRequest {
            status: Some(OrderStatus::Open),
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "status=open");
    }

    #[test]
    fn test_list_orders_request_with_pagination() {
        let request = ListOrdersRequest {
            page: Some(2),
            limit: Some(50),
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("page=2"));
        assert!(serialized.contains("limit=50"));
    }

    #[test]
    fn test_list_orders_request_with_time_range() {
        let request = ListOrdersRequest {
            from: Some(1640995200),
            to: Some(1641081600),
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("from=1640995200"));
        assert!(serialized.contains("to=1641081600"));
    }

    #[test]
    fn test_list_orders_request_with_side() {
        let request = ListOrdersRequest {
            side: Some(OrderSide::Buy),
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "side=buy");
    }

    #[test]
    fn test_list_orders_request_with_account() {
        let request = ListOrdersRequest {
            account: Some("spot".to_string()),
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "account=spot");
    }

    #[test]
    fn test_list_orders_request_full_parameters() {
        let request = ListOrdersRequest {
            currency_pair: Some("ETH_USDT".to_string()),
            status: Some(OrderStatus::Closed),
            page: Some(1),
            limit: Some(100),
            account: Some("spot".to_string()),
            from: Some(1640995200),
            to: Some(1641081600),
            side: Some(OrderSide::Sell),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("currency_pair=ETH_USDT"));
        assert!(serialized.contains("status=closed"));
        assert!(serialized.contains("page=1"));
        assert!(serialized.contains("limit=100"));
        assert!(serialized.contains("account=spot"));
        assert!(serialized.contains("from=1640995200"));
        assert!(serialized.contains("to=1641081600"));
        assert!(serialized.contains("side=sell"));
    }

    #[test]
    fn test_list_orders_request_different_currency_pairs() {
        let pairs = vec!["BTC_USDT", "ETH_BTC", "BNB_USDT", "SOL_USDC", "ADA_USDT"];

        for pair in pairs {
            let request = ListOrdersRequest {
                currency_pair: Some(pair.to_string()),
                ..Default::default()
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("currency_pair={}", pair));
        }
    }

    #[test]
    fn test_list_orders_request_different_statuses() {
        let statuses = vec![
            (OrderStatus::Open, "open"),
            (OrderStatus::Closed, "closed"),
            (OrderStatus::Cancelled, "cancelled"),
        ];

        for (status, expected) in statuses {
            let request = ListOrdersRequest {
                status: Some(status),
                ..Default::default()
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("status={}", expected));
        }
    }

    #[test]
    fn test_list_orders_request_different_sides() {
        let sides = vec![(OrderSide::Buy, "buy"), (OrderSide::Sell, "sell")];

        for (side, expected) in sides {
            let request = ListOrdersRequest {
                side: Some(side),
                ..Default::default()
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("side={}", expected));
        }
    }

    #[test]
    fn test_list_orders_request_different_accounts() {
        let accounts = vec!["spot", "margin", "cross_margin", "unified"];

        for account in accounts {
            let request = ListOrdersRequest {
                account: Some(account.to_string()),
                ..Default::default()
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("account={}", account));
        }
    }

    #[test]
    fn test_list_orders_request_pagination_ranges() {
        let pagination_tests = vec![(1, 10), (1, 100), (5, 50), (10, 25), (100, 1000)];

        for (page, limit) in pagination_tests {
            let request = ListOrdersRequest {
                page: Some(page),
                limit: Some(limit),
                ..Default::default()
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("page={}", page)));
            assert!(serialized.contains(&format!("limit={}", limit)));
        }
    }

    #[test]
    fn test_list_orders_request_zero_pagination() {
        let request = ListOrdersRequest {
            page: Some(0),
            limit: Some(0),
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("page=0"));
        assert!(serialized.contains("limit=0"));
    }

    #[test]
    fn test_list_orders_request_large_pagination() {
        let request = ListOrdersRequest {
            page: Some(u32::MAX),
            limit: Some(u32::MAX),
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains(&format!("page={}", u32::MAX)));
        assert!(serialized.contains(&format!("limit={}", u32::MAX)));
    }

    #[test]
    fn test_list_orders_request_time_range_scenarios() {
        // Recent orders (last hour)
        let recent_request = ListOrdersRequest {
            from: Some(1640995200),
            to: Some(1640998800), // 1 hour later
            ..Default::default()
        };

        let recent_serialized = serde_urlencoded::to_string(&recent_request).unwrap();
        assert!(recent_serialized.contains("from=1640995200"));
        assert!(recent_serialized.contains("to=1640998800"));

        // Historical orders (last 30 days)
        let historical_request = ListOrdersRequest {
            from: Some(1638403200),
            to: Some(1640995200), // 30 days
            ..Default::default()
        };

        let historical_serialized = serde_urlencoded::to_string(&historical_request).unwrap();
        assert!(historical_serialized.contains("from=1638403200"));
        assert!(historical_serialized.contains("to=1640995200"));

        // Open-ended from time
        let from_only_request = ListOrdersRequest {
            from: Some(1640995200),
            ..Default::default()
        };

        let from_only_serialized = serde_urlencoded::to_string(&from_only_request).unwrap();
        assert!(from_only_serialized.contains("from=1640995200"));
        assert!(!from_only_serialized.contains("to="));
    }

    #[test]
    fn test_list_orders_request_negative_timestamps() {
        let request = ListOrdersRequest {
            from: Some(-1640995200),
            to: Some(-1640995000),
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("from=-1640995200"));
        assert!(serialized.contains("to=-1640995000"));
    }

    #[test]
    fn test_list_orders_request_extreme_timestamps() {
        let request = ListOrdersRequest {
            from: Some(i64::MIN),
            to: Some(i64::MAX),
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains(&format!("from={}", i64::MIN)));
        assert!(serialized.contains(&format!("to={}", i64::MAX)));
    }

    #[test]
    fn test_list_orders_request_json_serialization() {
        let request = ListOrdersRequest {
            currency_pair: Some("BTC_USDT".to_string()),
            status: Some(OrderStatus::Open),
            page: Some(1),
            limit: Some(50),
            account: Some("spot".to_string()),
            from: Some(1640995200),
            to: Some(1641081600),
            side: Some(OrderSide::Buy),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["status"], "open");
        assert_eq!(json["page"], 1);
        assert_eq!(json["limit"], 50);
        assert_eq!(json["account"], "spot");
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["to"], 1641081600);
        assert_eq!(json["side"], "buy");
    }

    #[test]
    fn test_list_orders_request_json_serialization_defaults() {
        let request = ListOrdersRequest::default();

        let json = serde_json::to_value(&request).unwrap();

        // All fields should be omitted when None
        let obj = json.as_object().unwrap();
        assert!(!obj.contains_key("currency_pair"));
        assert!(!obj.contains_key("status"));
        assert!(!obj.contains_key("page"));
        assert!(!obj.contains_key("limit"));
        assert!(!obj.contains_key("account"));
        assert!(!obj.contains_key("from"));
        assert!(!obj.contains_key("to"));
        assert!(!obj.contains_key("side"));
    }

    #[test]
    fn test_list_orders_request_partial_parameters() {
        // Only currency pair and status
        let request1 = ListOrdersRequest {
            currency_pair: Some("ETH_USDT".to_string()),
            status: Some(OrderStatus::Closed),
            ..Default::default()
        };

        let serialized1 = serde_urlencoded::to_string(&request1).unwrap();
        assert!(serialized1.contains("currency_pair=ETH_USDT"));
        assert!(serialized1.contains("status=closed"));
        assert!(!serialized1.contains("page="));
        assert!(!serialized1.contains("limit="));

        // Only pagination
        let request2 = ListOrdersRequest {
            page: Some(3),
            limit: Some(25),
            ..Default::default()
        };

        let serialized2 = serde_urlencoded::to_string(&request2).unwrap();
        assert!(serialized2.contains("page=3"));
        assert!(serialized2.contains("limit=25"));
        assert!(!serialized2.contains("currency_pair="));
        assert!(!serialized2.contains("status="));
    }

    #[test]
    fn test_list_orders_request_realistic_trading_scenarios() {
        // Scenario 1: Recent BTC buy orders
        let btc_buys = ListOrdersRequest {
            currency_pair: Some("BTC_USDT".to_string()),
            side: Some(OrderSide::Buy),
            status: Some(OrderStatus::Closed),
            from: Some(1640995200),
            limit: Some(50),
            ..Default::default()
        };

        let btc_serialized = serde_urlencoded::to_string(&btc_buys).unwrap();
        assert!(btc_serialized.contains("currency_pair=BTC_USDT"));
        assert!(btc_serialized.contains("side=buy"));
        assert!(btc_serialized.contains("status=closed"));

        // Scenario 2: All open orders for margin account
        let margin_opens = ListOrdersRequest {
            status: Some(OrderStatus::Open),
            account: Some("margin".to_string()),
            limit: Some(100),
            ..Default::default()
        };

        let margin_serialized = serde_urlencoded::to_string(&margin_opens).unwrap();
        assert!(margin_serialized.contains("status=open"));
        assert!(margin_serialized.contains("account=margin"));

        // Scenario 3: Historical ETH trades in specific time window
        let eth_history = ListOrdersRequest {
            currency_pair: Some("ETH_USDT".to_string()),
            from: Some(1638403200),
            to: Some(1640995200),
            page: Some(1),
            limit: Some(200),
            ..Default::default()
        };

        let eth_serialized = serde_urlencoded::to_string(&eth_history).unwrap();
        assert!(eth_serialized.contains("currency_pair=ETH_USDT"));
        assert!(eth_serialized.contains("from=1638403200"));
        assert!(eth_serialized.contains("to=1640995200"));
    }

    #[test]
    fn test_list_orders_request_stablecoin_trading() {
        let stablecoin_pairs = vec!["USDC_USDT", "BUSD_USDT", "DAI_USDT"];

        for pair in stablecoin_pairs {
            let request = ListOrdersRequest {
                currency_pair: Some(pair.to_string()),
                status: Some(OrderStatus::Closed),
                limit: Some(10),
                ..Default::default()
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("currency_pair={}", pair)));
            assert!(serialized.contains("status=closed"));
        }
    }

    #[test]
    fn test_list_orders_request_cross_margin_scenarios() {
        let request = ListOrdersRequest {
            account: Some("cross_margin".to_string()),
            status: Some(OrderStatus::Open),
            currency_pair: Some("BTC_USDT".to_string()),
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("account=cross_margin"));
        assert!(serialized.contains("status=open"));
        assert!(serialized.contains("currency_pair=BTC_USDT"));
    }

    #[test]
    fn test_list_orders_request_unified_account() {
        let request = ListOrdersRequest {
            account: Some("unified".to_string()),
            from: Some(1640995200),
            limit: Some(500),
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("account=unified"));
        assert!(serialized.contains("from=1640995200"));
        assert!(serialized.contains("limit=500"));
    }

    #[test]
    fn test_list_orders_request_default_values() {
        let request = ListOrdersRequest::default();

        assert_eq!(request.currency_pair, None);
        assert_eq!(request.status, None);
        assert_eq!(request.page, None);
        assert_eq!(request.limit, None);
        assert_eq!(request.account, None);
        assert_eq!(request.from, None);
        assert_eq!(request.to, None);
        assert_eq!(request.side, None);
    }

    #[test]
    fn test_list_orders_request_clone() {
        let original = ListOrdersRequest {
            currency_pair: Some("BTC_USDT".to_string()),
            status: Some(OrderStatus::Open),
            page: Some(1),
            limit: Some(50),
            account: Some("spot".to_string()),
            from: Some(1640995200),
            to: Some(1641081600),
            side: Some(OrderSide::Buy),
        };

        let cloned = original.clone();
        assert_eq!(cloned.currency_pair, original.currency_pair);
        assert_eq!(cloned.status, original.status);
        assert_eq!(cloned.page, original.page);
        assert_eq!(cloned.limit, original.limit);
        assert_eq!(cloned.account, original.account);
        assert_eq!(cloned.from, original.from);
        assert_eq!(cloned.to, original.to);
        assert_eq!(cloned.side, original.side);
    }

    #[test]
    fn test_list_orders_request_debug() {
        let request = ListOrdersRequest {
            currency_pair: Some("BTC_USDT".to_string()),
            status: Some(OrderStatus::Open),
            side: Some(OrderSide::Buy),
            ..Default::default()
        };

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("ListOrdersRequest"));
        assert!(debug_str.contains("BTC_USDT"));
        assert!(debug_str.contains("Open"));
        assert!(debug_str.contains("Buy"));
    }
}
