use serde::{Deserialize, Serialize};

/// Futures order information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesOrder {
    /// Order ID
    pub id: i64,

    /// User ID
    pub user: i64,

    /// Contract name
    pub contract: String,

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
    pub size: i64,

    /// Iceberg amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg: Option<i64>,

    /// Order price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Order type
    #[serde(rename = "type")]
    pub order_type: Option<String>,

    /// Time in force
    pub tif: String,

    /// Left amount
    pub left: i64,

    /// Filled total
    pub fill_price: String,

    /// Order text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Reduce only
    pub reduce_only: bool,

    /// Close position
    pub close: bool,

    /// Reject post only
    pub reject_post_only: bool,

    /// STP action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_act: Option<String>,

    /// Amendment ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amend_text: Option<String>,
}

/// Request to list futures orders
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListFuturesOrdersRequest {
    /// Settlement currency
    pub settle: String,

    /// Order status (open, finished)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,

    /// Start time (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Maximum number of records to return (1-1000, default: 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Page offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    /// Count total records
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_total: Option<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_futures_order_deserialization() {
        let json = r#"{
            "id": 123456789,
            "user": 987654,
            "contract": "BTC_USDT",
            "create_time": 1640995200.123,
            "finish_time": 1640995260.456,
            "finish_as": "filled",
            "status": "finished",
            "size": 1000,
            "iceberg": 100,
            "price": "43250.0",
            "type": "limit",
            "tif": "gtc",
            "left": 0,
            "fill_price": "43248.5",
            "text": "api-order",
            "reduce_only": false,
            "close": false,
            "reject_post_only": false,
            "stp_act": "cn",
            "amend_text": "amended-1"
        }"#;

        let order: FuturesOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.id, 123456789);
        assert_eq!(order.user, 987654);
        assert_eq!(order.contract, "BTC_USDT");
        assert_eq!(order.create_time, 1640995200.123);
        assert_eq!(order.finish_time.unwrap(), 1640995260.456);
        assert_eq!(order.finish_as.as_ref().unwrap(), "filled");
        assert_eq!(order.status, "finished");
        assert_eq!(order.size, 1000);
        assert_eq!(order.iceberg.unwrap(), 100);
        assert_eq!(order.price.as_ref().unwrap(), "43250.0");
        assert_eq!(order.order_type.as_ref().unwrap(), "limit");
        assert_eq!(order.tif, "gtc");
        assert_eq!(order.left, 0);
        assert_eq!(order.fill_price, "43248.5");
        assert_eq!(order.text.as_ref().unwrap(), "api-order");
        assert!(!order.reduce_only);
        assert!(!order.close);
        assert!(!order.reject_post_only);
        assert_eq!(order.stp_act.as_ref().unwrap(), "cn");
        assert_eq!(order.amend_text.as_ref().unwrap(), "amended-1");
    }

    #[test]
    fn test_order_status_scenarios() {
        let status_scenarios = vec![
            ("open", 1000, 800, "Partially filled"),
            ("finished", 1000, 0, "Fully filled"),
            ("cancelled", 1000, 1000, "Cancelled before fill"),
            ("finished", 1000, 500, "Partially filled then cancelled"),
        ];

        for (status, size, left, _description) in status_scenarios {
            let json = format!(
                r#"{{
                "id": 123456789,
                "user": 987654,
                "contract": "BTC_USDT",
                "create_time": 1640995200.123,
                "status": "{}",
                "size": {},
                "tif": "gtc",
                "left": {},
                "fill_price": "43248.5",
                "reduce_only": false,
                "close": false,
                "reject_post_only": false
            }}"#,
                status, size, left
            );

            let order: FuturesOrder = serde_json::from_str(&json).unwrap();
            assert_eq!(order.status, status);
            assert_eq!(order.size, size);
            assert_eq!(order.left, left);

            // Calculate filled amount
            let filled = size - left;
            if status == "open" {
                assert!(left > 0);
            } else if status == "finished" && left == 0 {
                assert_eq!(filled, size);
            }
        }
    }

    #[test]
    fn test_finish_as_scenarios() {
        let finish_scenarios = vec![
            ("filled", "Order fully filled"),
            ("cancelled", "Order cancelled by user"),
            ("ioc", "IOC order expired"),
            ("fok", "FOK order not filled"),
            ("expired", "Order expired"),
            ("liquidated", "Position liquidated"),
        ];

        for (finish_as, _description) in finish_scenarios {
            let json = format!(
                r#"{{
                "id": 123456789,
                "user": 987654,
                "contract": "BTC_USDT",
                "create_time": 1640995200.123,
                "finish_time": 1640995260.456,
                "finish_as": "{}",
                "status": "finished",
                "size": 1000,
                "tif": "gtc",
                "left": 0,
                "fill_price": "43248.5",
                "reduce_only": false,
                "close": false,
                "reject_post_only": false
            }}"#,
                finish_as
            );

            let order: FuturesOrder = serde_json::from_str(&json).unwrap();
            assert_eq!(order.finish_as.unwrap(), finish_as);
            assert_eq!(order.status, "finished");
        }
    }

    #[test]
    fn test_stp_action_options() {
        let stp_options = vec![
            ("cn", "Cancel newest"),
            ("co", "Cancel oldest"),
            ("cb", "Cancel both"),
        ];

        for (stp_act, _description) in stp_options {
            let json = format!(
                r#"{{
                "id": 123456789,
                "user": 987654,
                "contract": "BTC_USDT",
                "create_time": 1640995200.123,
                "status": "open",
                "size": 1000,
                "tif": "gtc",
                "left": 1000,
                "fill_price": "0",
                "reduce_only": false,
                "close": false,
                "reject_post_only": false,
                "stp_act": "{}"
            }}"#,
                stp_act
            );

            let order: FuturesOrder = serde_json::from_str(&json).unwrap();
            assert_eq!(order.stp_act.unwrap(), stp_act);
        }
    }

    #[test]
    fn test_debug_output() {
        let order = FuturesOrder {
            id: 123456789,
            user: 987654,
            contract: "BTC_USDT".to_string(),
            create_time: 1640995200.123,
            finish_time: None,
            finish_as: None,
            status: "open".to_string(),
            size: 1000,
            iceberg: None,
            price: Some("43250.0".to_string()),
            order_type: Some("limit".to_string()),
            tif: "gtc".to_string(),
            left: 1000,
            fill_price: "0".to_string(),
            text: Some("test".to_string()),
            reduce_only: false,
            close: false,
            reject_post_only: false,
            stp_act: None,
            amend_text: None,
        };

        let debug_str = format!("{:?}", order);
        assert!(debug_str.contains("FuturesOrder"));
        assert!(debug_str.contains("123456789"));
        assert!(debug_str.contains("BTC_USDT"));
    }

    #[test]
    fn test_serialization_round_trip() {
        let order = FuturesOrder {
            id: 123456789,
            user: 987654,
            contract: "BTC_USDT".to_string(),
            create_time: 1640995200.123,
            finish_time: Some(1640995260.456),
            finish_as: Some("filled".to_string()),
            status: "finished".to_string(),
            size: 1000,
            iceberg: Some(100),
            price: Some("43250.0".to_string()),
            order_type: Some("limit".to_string()),
            tif: "gtc".to_string(),
            left: 0,
            fill_price: "43248.5".to_string(),
            text: Some("api-order".to_string()),
            reduce_only: false,
            close: false,
            reject_post_only: false,
            stp_act: Some("cn".to_string()),
            amend_text: Some("amended".to_string()),
        };

        let json = serde_json::to_string(&order).unwrap();
        let deserialized: FuturesOrder = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, order.id);
        assert_eq!(deserialized.contract, order.contract);
        assert_eq!(deserialized.status, order.status);
        assert_eq!(deserialized.size, order.size);
        assert_eq!(deserialized.price, order.price);
    }

    #[test]
    fn test_list_futures_orders_request_minimal() {
        let request = ListFuturesOrdersRequest {
            settle: "usdt".to_string(),
            status: None,
            contract: None,
            from: None,
            to: None,
            limit: None,
            offset: None,
            count_total: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "usdt");
        assert!(!json.as_object().unwrap().contains_key("status"));
        assert!(!json.as_object().unwrap().contains_key("contract"));
        assert!(!json.as_object().unwrap().contains_key("from"));
        assert!(!json.as_object().unwrap().contains_key("to"));
        assert!(!json.as_object().unwrap().contains_key("limit"));
        assert!(!json.as_object().unwrap().contains_key("offset"));
        assert!(!json.as_object().unwrap().contains_key("count_total"));
    }

    #[test]
    fn test_list_futures_orders_request_with_filters() {
        let request = ListFuturesOrdersRequest {
            settle: "usdt".to_string(),
            status: Some("open".to_string()),
            contract: Some("BTC_USDT".to_string()),
            from: Some(1640995200),
            to: Some(1640995800),
            limit: Some(50),
            offset: Some(100),
            count_total: Some(1),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "usdt");
        assert_eq!(json["status"], "open");
        assert_eq!(json["contract"], "BTC_USDT");
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["to"], 1640995800);
        assert_eq!(json["limit"], 50);
        assert_eq!(json["offset"], 100);
        assert_eq!(json["count_total"], 1);
    }

    #[test]
    fn test_list_futures_orders_request_status_values() {
        let statuses = vec!["open", "finished"];

        for status in statuses {
            let request = ListFuturesOrdersRequest {
                settle: "usdt".to_string(),
                status: Some(status.to_string()),
                contract: None,
                from: None,
                to: None,
                limit: None,
                offset: None,
                count_total: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["status"], status);
        }
    }

    #[test]
    fn test_list_futures_orders_request_settle_currencies() {
        let settle_currencies = vec!["usdt", "btc"];

        for settle in settle_currencies {
            let request = ListFuturesOrdersRequest {
                settle: settle.to_string(),
                status: None,
                contract: None,
                from: None,
                to: None,
                limit: None,
                offset: None,
                count_total: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["settle"], settle);
        }
    }
}
