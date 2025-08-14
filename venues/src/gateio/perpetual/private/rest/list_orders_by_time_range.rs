use serde::{Deserialize, Serialize};

use super::{RestClient, order::FuturesOrder};

const ENDPOINT_FUTURES_PREFIX: &str = "/futures";

/// Request parameters for listing orders by time range
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListOrdersByTimeRangeRequest {
    /// Settlement currency
    pub settle: String,

    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,

    /// Start time (Unix timestamp in seconds)
    pub from: i64,

    /// End time (Unix timestamp in seconds)
    pub to: i64,

    /// Maximum number of records to return (1-1000, default: 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Page offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

impl RestClient {
    /// List futures orders by time range
    ///
    /// Returns orders within a specific time range for better performance.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/#query-futures-orders-by-time-range)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `params` - The time range request parameters
    ///
    /// # Returns
    /// List of orders within the specified time range
    pub async fn list_futures_orders_by_time_range(
        &self,
        params: ListOrdersByTimeRangeRequest,
    ) -> crate::gateio::perpetual::RestResult<Vec<FuturesOrder>> {
        let endpoint = format!(
            "{}/{}/orders_timerange",
            ENDPOINT_FUTURES_PREFIX, params.settle
        );
        self.get_with_query(&endpoint, &params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_orders_by_time_range_minimal() {
        let request = ListOrdersByTimeRangeRequest {
            settle: "USDT".to_string(),
            contract: None,
            from: 1640995200,
            to: 1640998800,
            limit: None,
            offset: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["to"], 1640998800);
        assert!(!json.as_object().unwrap().contains_key("contract"));
        assert!(!json.as_object().unwrap().contains_key("limit"));
        assert!(!json.as_object().unwrap().contains_key("offset"));
    }

    #[test]
    fn test_list_orders_by_time_range_full() {
        let request = ListOrdersByTimeRangeRequest {
            settle: "USDT".to_string(),
            contract: Some("BTC_USDT".to_string()),
            from: 1640995200,
            to: 1640998800,
            limit: Some(500),
            offset: Some(0),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT");
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["to"], 1640998800);
        assert_eq!(json["limit"], 500);
        assert_eq!(json["offset"], 0);
    }

    #[test]
    fn test_time_range_validation() {
        let test_cases = vec![
            (1640995200, 1640998800, "1 hour range"),
            (1640995200, 1641081600, "24 hour range"),
            (1640995200, 1641600000, "7 day range"),
            (1640995200, 1643673600, "30 day range"),
        ];

        for (from, to, _description) in test_cases {
            let request = ListOrdersByTimeRangeRequest {
                settle: "USDT".to_string(),
                contract: None,
                from,
                to,
                limit: None,
                offset: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            let from_val = json["from"].as_i64().unwrap();
            let to_val = json["to"].as_i64().unwrap();
            assert!(
                to_val > from_val,
                "To timestamp must be after from timestamp"
            );
        }
    }

    #[test]
    fn test_pagination_limits() {
        let limit_scenarios = vec![
            (1, "Minimum limit"),
            (100, "Default limit"),
            (500, "Mid-range limit"),
            (1000, "Maximum limit"),
        ];

        for (limit, _description) in limit_scenarios {
            let request = ListOrdersByTimeRangeRequest {
                settle: "USDT".to_string(),
                contract: None,
                from: 1640995200,
                to: 1640998800,
                limit: Some(limit),
                offset: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["limit"], limit);
        }
    }

    #[test]
    fn test_pagination_offsets() {
        let offset_scenarios = vec![
            (0, "First page"),
            (100, "Second page"),
            (500, "Sixth page"),
            (1000, "Eleventh page"),
        ];

        for (offset, _description) in offset_scenarios {
            let request = ListOrdersByTimeRangeRequest {
                settle: "USDT".to_string(),
                contract: None,
                from: 1640995200,
                to: 1640998800,
                limit: Some(100),
                offset: Some(offset),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["offset"], offset);
        }
    }

    #[test]
    fn test_contract_filters() {
        let contracts = vec!["BTC_USDT", "ETH_USDT", "SOL_USDT", "MATIC_USDT"];

        for contract in contracts {
            let request = ListOrdersByTimeRangeRequest {
                settle: "USDT".to_string(),
                contract: Some(contract.to_string()),
                from: 1640995200,
                to: 1640998800,
                limit: None,
                offset: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["contract"], contract);
        }
    }

    #[test]
    fn test_settlement_currencies() {
        let settlements = vec!["USDT", "BTC", "ETH"];

        for settle in settlements {
            let request = ListOrdersByTimeRangeRequest {
                settle: settle.to_string(),
                contract: None,
                from: 1640995200,
                to: 1640998800,
                limit: None,
                offset: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["settle"], settle);
        }
    }

    #[test]
    fn test_realistic_time_queries() {
        // Query last hour's orders
        let last_hour = ListOrdersByTimeRangeRequest {
            settle: "USDT".to_string(),
            contract: Some("BTC_USDT".to_string()),
            from: 1640995200,
            to: 1640998800, // 1 hour later
            limit: Some(100),
            offset: Some(0),
        };

        let json = serde_json::to_value(&last_hour).unwrap();
        let duration = json["to"].as_i64().unwrap() - json["from"].as_i64().unwrap();
        assert_eq!(duration, 3600); // 1 hour in seconds

        // Query last day's orders
        let last_day = ListOrdersByTimeRangeRequest {
            settle: "USDT".to_string(),
            contract: None,
            from: 1640995200,
            to: 1641081600, // 24 hours later
            limit: Some(1000),
            offset: Some(0),
        };

        let json = serde_json::to_value(&last_day).unwrap();
        let duration = json["to"].as_i64().unwrap() - json["from"].as_i64().unwrap();
        assert_eq!(duration, 86400); // 24 hours in seconds

        // Query specific trading session
        let trading_session = ListOrdersByTimeRangeRequest {
            settle: "USDT".to_string(),
            contract: Some("ETH_USDT".to_string()),
            from: 1640995200,
            to: 1641016800, // 6 hours later
            limit: Some(500),
            offset: Some(0),
        };

        let json = serde_json::to_value(&trading_session).unwrap();
        let duration = json["to"].as_i64().unwrap() - json["from"].as_i64().unwrap();
        assert_eq!(duration, 21600); // 6 hours in seconds
    }

    #[test]
    fn test_large_dataset_pagination() {
        // First page of large dataset
        let page1 = ListOrdersByTimeRangeRequest {
            settle: "USDT".to_string(),
            contract: None,
            from: 1640995200,
            to: 1641600000, // 7 days
            limit: Some(1000),
            offset: Some(0),
        };

        let json = serde_json::to_value(&page1).unwrap();
        assert_eq!(json["limit"], 1000);
        assert_eq!(json["offset"], 0);

        // Fifth page of large dataset
        let page5 = ListOrdersByTimeRangeRequest {
            settle: "USDT".to_string(),
            contract: None,
            from: 1640995200,
            to: 1641600000,
            limit: Some(1000),
            offset: Some(4000), // Skip first 4000 records
        };

        let json = serde_json::to_value(&page5).unwrap();
        assert_eq!(json["limit"], 1000);
        assert_eq!(json["offset"], 4000);
    }

    #[test]
    fn test_default_behavior() {
        let request = ListOrdersByTimeRangeRequest::default();

        // Default values should be empty/zero
        assert_eq!(request.settle, "");
        assert!(request.contract.is_none());
        assert_eq!(request.from, 0);
        assert_eq!(request.to, 0);
        assert!(request.limit.is_none());
        assert!(request.offset.is_none());
    }

    #[test]
    fn test_serialization_omits_null() {
        let request = ListOrdersByTimeRangeRequest {
            settle: "USDT".to_string(),
            contract: None,
            from: 1640995200,
            to: 1640998800,
            limit: None,
            offset: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();

        // Only required fields should be present
        assert_eq!(obj.len(), 3); // settle, from, to
        assert!(obj.contains_key("settle"));
        assert!(obj.contains_key("from"));
        assert!(obj.contains_key("to"));
        assert!(!obj.contains_key("contract"));
        assert!(!obj.contains_key("limit"));
        assert!(!obj.contains_key("offset"));
    }
}
