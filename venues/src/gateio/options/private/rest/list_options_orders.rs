use serde::Serialize;

use super::{RestClient, order::OptionsOrder};

const LIST_OPTIONS_ORDERS_ENDPOINT: &str = "/options/orders";

/// Request parameters for listing options orders
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListOptionsOrdersRequest {
    /// Order status (open, finished)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,

    /// Underlying filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,

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
}

impl RestClient {
    /// List options orders
    ///
    /// This endpoint returns options orders for the authenticated user.
    ///
    /// See: Gate.io API documentation
    /// <https://www.gate.io/docs/developers/apiv4/#list-options-orders>
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `params` - The options orders list request parameters
    ///
    /// # Returns
    /// List of options orders
    pub async fn list_options_orders(
        &self,
        params: ListOptionsOrdersRequest,
    ) -> crate::gateio::options::Result<Vec<OptionsOrder>> {
        self.get_with_query(LIST_OPTIONS_ORDERS_ENDPOINT, &params)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_options_orders_request_minimal() {
        let request = ListOptionsOrdersRequest::default();

        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.is_empty());
    }

    #[test]
    fn test_list_options_orders_request_with_status() {
        let request = ListOptionsOrdersRequest {
            status: Some("open".to_string()),
            contract: None,
            underlying: None,
            from: None,
            to: None,
            limit: None,
            offset: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["status"], "open");
        assert!(!json.as_object().unwrap().contains_key("contract"));
    }

    #[test]
    fn test_list_options_orders_request_full() {
        let request = ListOptionsOrdersRequest {
            status: Some("finished".to_string()),
            contract: Some("BTC-20240101-50000-C".to_string()),
            underlying: Some("BTC_USDT".to_string()),
            from: Some(1640995200),
            to: Some(1640998800),
            limit: Some(100),
            offset: Some(0),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["status"], "finished");
        assert_eq!(json["contract"], "BTC-20240101-50000-C");
        assert_eq!(json["underlying"], "BTC_USDT");
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["to"], 1640998800);
        assert_eq!(json["limit"], 100);
        assert_eq!(json["offset"], 0);
    }

    #[test]
    fn test_status_filter_values() {
        let statuses = vec!["open", "finished"];

        for status in statuses {
            let request = ListOptionsOrdersRequest {
                status: Some(status.to_string()),
                contract: None,
                underlying: None,
                from: None,
                to: None,
                limit: None,
                offset: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["status"], status);
        }
    }

    #[test]
    fn test_underlying_filter_values() {
        let underlyings = vec!["BTC_USDT", "ETH_USDT", "SOL_USDT"];

        for underlying in underlyings {
            let request = ListOptionsOrdersRequest {
                status: None,
                contract: None,
                underlying: Some(underlying.to_string()),
                from: None,
                to: None,
                limit: None,
                offset: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["underlying"], underlying);
        }
    }

    #[test]
    fn test_time_range_filters() {
        // Last hour
        let last_hour = ListOptionsOrdersRequest {
            status: Some("finished".to_string()),
            contract: None,
            underlying: None,
            from: Some(1640995200),
            to: Some(1640998800),
            limit: None,
            offset: None,
        };

        let json = serde_json::to_value(&last_hour).unwrap();
        let from = json["from"].as_i64().unwrap();
        let to = json["to"].as_i64().unwrap();
        assert_eq!(to - from, 3600); // 1 hour in seconds

        // Last day
        let last_day = ListOptionsOrdersRequest {
            status: None,
            contract: None,
            underlying: None,
            from: Some(1640908800),
            to: Some(1640995200),
            limit: None,
            offset: None,
        };

        let json = serde_json::to_value(&last_day).unwrap();
        let from = json["from"].as_i64().unwrap();
        let to = json["to"].as_i64().unwrap();
        assert_eq!(to - from, 86400); // 24 hours in seconds
    }

    #[test]
    fn test_pagination_limits() {
        let limits = vec![1, 10, 50, 100, 500, 1000];

        for limit in limits {
            let request = ListOptionsOrdersRequest {
                status: None,
                contract: None,
                underlying: None,
                from: None,
                to: None,
                limit: Some(limit),
                offset: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["limit"], limit);
        }
    }

    #[test]
    fn test_pagination_offsets() {
        let offsets = vec![0, 10, 50, 100, 500, 1000];

        for offset in offsets {
            let request = ListOptionsOrdersRequest {
                status: None,
                contract: None,
                underlying: None,
                from: None,
                to: None,
                limit: Some(100),
                offset: Some(offset),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["offset"], offset);
        }
    }

    #[test]
    fn test_combined_filters() {
        let request = ListOptionsOrdersRequest {
            status: Some("open".to_string()),
            contract: None,
            underlying: Some("ETH_USDT".to_string()),
            from: None,
            to: None,
            limit: Some(50),
            offset: Some(0),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["status"], "open");
        assert_eq!(json["underlying"], "ETH_USDT");
        assert_eq!(json["limit"], 50);
        assert!(!json.as_object().unwrap().contains_key("contract"));
        assert!(!json.as_object().unwrap().contains_key("from"));
        assert!(!json.as_object().unwrap().contains_key("to"));
    }

    #[test]
    fn test_serialization_omits_none() {
        let request = ListOptionsOrdersRequest {
            status: Some("finished".to_string()),
            contract: None,
            underlying: None,
            from: Some(1640995200),
            to: None,
            limit: None,
            offset: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 2); // Only status and from
        assert!(obj.contains_key("status"));
        assert!(obj.contains_key("from"));
        assert!(!obj.contains_key("contract"));
        assert!(!obj.contains_key("underlying"));
        assert!(!obj.contains_key("to"));
        assert!(!obj.contains_key("limit"));
        assert!(!obj.contains_key("offset"));
    }
}
