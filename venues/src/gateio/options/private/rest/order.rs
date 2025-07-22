use serde::{Deserialize, Serialize};

/// Options order information (common struct used by multiple endpoints)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsOrder {
    /// Order ID
    pub id: i64,

    /// User ID
    pub user: i64,

    /// Contract name
    pub contract: String,

    /// Underlying asset
    pub underlying: String,

    /// Creation timestamp
    pub create_time: f64,

    /// Finish timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_time: Option<f64>,

    /// Finish reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_as: Option<String>,

    /// Order status
    pub status: String,

    /// Order size
    pub size: String,

    /// Order price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Time in force
    pub tif: String,

    /// Left amount
    pub left: String,

    /// Filled total
    pub filled_total: String,

    /// Average fill price
    pub avg_deal_price: String,

    /// Order text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Order type
    #[serde(rename = "type")]
    pub order_type: String,

    /// Is reduce only
    pub is_reduce_only: bool,

    /// Is close order
    pub is_close: bool,

    /// Order fee
    pub fee: String,

    /// Rebate
    pub rebate: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_options_order_deserialization() {
        let json = r#"{
            "id": 12345,
            "user": 67890,
            "contract": "BTC-20240101-50000-C",
            "underlying": "BTC_USDT",
            "create_time": 1640995200.123,
            "finish_time": 1640995300.456,
            "finish_as": "filled",
            "status": "finished",
            "size": "1",
            "price": "0.1",
            "tif": "gtc",
            "left": "0",
            "filled_total": "1",
            "avg_deal_price": "0.1",
            "text": "test order",
            "type": "limit",
            "is_reduce_only": false,
            "is_close": false,
            "fee": "0.001",
            "rebate": "0.0"
        }"#;

        let order: OptionsOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.id, 12345);
        assert_eq!(order.user, 67890);
        assert_eq!(order.contract, "BTC-20240101-50000-C");
        assert_eq!(order.underlying, "BTC_USDT");
        assert_eq!(order.create_time, 1640995200.123);
        assert_eq!(order.finish_time, Some(1640995300.456));
        assert_eq!(order.finish_as, Some("filled".to_string()));
        assert_eq!(order.status, "finished");
        assert_eq!(order.size, "1");
        assert_eq!(order.price, Some("0.1".to_string()));
        assert_eq!(order.tif, "gtc");
        assert_eq!(order.left, "0");
        assert_eq!(order.filled_total, "1");
        assert_eq!(order.avg_deal_price, "0.1");
        assert_eq!(order.text, Some("test order".to_string()));
        assert_eq!(order.order_type, "limit");
        assert!(!order.is_reduce_only);
        assert!(!order.is_close);
        assert_eq!(order.fee, "0.001");
        assert_eq!(order.rebate, "0.0");
    }

    #[test]
    fn test_options_order_minimal_deserialization() {
        let json = r#"{
            "id": 98765,
            "user": 54321,
            "contract": "ETH-20240101-3000-P",
            "underlying": "ETH_USDT",
            "create_time": 1640995200.0,
            "status": "open",
            "size": "2.5",
            "tif": "ioc",
            "left": "2.5",
            "filled_total": "0",
            "avg_deal_price": "0",
            "type": "market",
            "is_reduce_only": true,
            "is_close": false,
            "fee": "0",
            "rebate": "0"
        }"#;

        let order: OptionsOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.id, 98765);
        assert!(order.finish_time.is_none());
        assert!(order.finish_as.is_none());
        assert!(order.price.is_none());
        assert!(order.text.is_none());
        assert_eq!(order.status, "open");
        assert!(order.is_reduce_only);
    }

    #[test]
    fn test_options_order_status_values() {
        let statuses = vec!["open", "finished", "cancelled"];
        
        for status in statuses {
            let json = format!(r#"{{
                "id": 12345,
                "user": 67890,
                "contract": "BTC-20240101-50000-C",
                "underlying": "BTC_USDT",
                "create_time": 1640995200.0,
                "status": "{}",
                "size": "1",
                "tif": "gtc",
                "left": "0",
                "filled_total": "1",
                "avg_deal_price": "0.1",
                "type": "limit",
                "is_reduce_only": false,
                "is_close": false,
                "fee": "0.001",
                "rebate": "0"
            }}"#, status);

            let order: OptionsOrder = serde_json::from_str(&json).unwrap();
            assert_eq!(order.status, status);
        }
    }

    #[test]
    fn test_options_order_finish_as_values() {
        let finish_as_values = vec!["filled", "cancelled", "ioc", "expired", "reduced"];
        
        for finish_as in finish_as_values {
            let json = format!(r#"{{
                "id": 12345,
                "user": 67890,
                "contract": "BTC-20240101-50000-C",
                "underlying": "BTC_USDT",
                "create_time": 1640995200.0,
                "finish_time": 1640995300.0,
                "finish_as": "{}",
                "status": "finished",
                "size": "1",
                "tif": "gtc",
                "left": "0",
                "filled_total": "1",
                "avg_deal_price": "0.1",
                "type": "limit",
                "is_reduce_only": false,
                "is_close": false,
                "fee": "0.001",
                "rebate": "0"
            }}"#, finish_as);

            let order: OptionsOrder = serde_json::from_str(&json).unwrap();
            assert_eq!(order.finish_as, Some(finish_as.to_string()));
        }
    }

    #[test]
    fn test_options_order_type_values() {
        let order_types = vec!["limit", "market"];
        
        for order_type in order_types {
            let json = format!(r#"{{
                "id": 12345,
                "user": 67890,
                "contract": "BTC-20240101-50000-C",
                "underlying": "BTC_USDT",
                "create_time": 1640995200.0,
                "status": "open",
                "size": "1",
                "tif": "gtc",
                "left": "1",
                "filled_total": "0",
                "avg_deal_price": "0",
                "type": "{}",
                "is_reduce_only": false,
                "is_close": false,
                "fee": "0",
                "rebate": "0"
            }}"#, order_type);

            let order: OptionsOrder = serde_json::from_str(&json).unwrap();
            assert_eq!(order.order_type, order_type);
        }
    }
}