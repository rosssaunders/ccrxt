use serde::{Deserialize, Serialize};

use super::{RestClient, order::FuturesOrder};

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

impl RestClient {
    /// List futures orders
    ///
    /// This endpoint returns futures orders for the authenticated user.
    ///
    /// See: Gate.io API documentation
    /// <https://www.gate.io/docs/developers/apiv4/#list-futures-orders>
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `params` - The order listing request parameters
    ///
    /// # Returns
    /// List of orders
    pub async fn list_futures_orders(
        &self,
        params: ListFuturesOrdersRequest,
    ) -> crate::gateio::perpetual::Result<Vec<FuturesOrder>> {
        let endpoint = format!("/futures/{}/orders", params.settle);
        self.get_with_query(&endpoint, &params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_futures_orders_request_minimal() {
        let request = ListFuturesOrdersRequest {
            settle: "USDT".to_string(),
            ..Default::default()
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 1); // Only settle field
    }

    #[test]
    fn test_list_futures_orders_request_full() {
        let request = ListFuturesOrdersRequest {
            settle: "USDT".to_string(),
            status: Some("open".to_string()),
            contract: Some("BTC_USDT".to_string()),
            from: Some(1640995200),
            to: Some(1641081600),
            limit: Some(50),
            offset: Some(100),
            count_total: Some(1),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["status"], "open");
        assert_eq!(json["contract"], "BTC_USDT");
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["to"], 1641081600);
        assert_eq!(json["limit"], 50);
        assert_eq!(json["offset"], 100);
        assert_eq!(json["count_total"], 1);
    }

    #[test]
    fn test_order_status_filter() {
        let status_options = vec![
            ("open", "Active orders only"),
            ("finished", "Completed orders only"),
        ];

        for (status, _description) in status_options {
            let request = ListFuturesOrdersRequest {
                settle: "USDT".to_string(),
                status: Some(status.to_string()),
                ..Default::default()
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["status"], status);
        }
    }

    #[test]
    fn test_pagination_scenarios() {
        // Scenario 1: First page
        let first_page = ListFuturesOrdersRequest {
            settle: "USDT".to_string(),
            limit: Some(100),
            offset: Some(0),
            ..Default::default()
        };

        let json = serde_json::to_value(&first_page).unwrap();
        assert_eq!(json["limit"], 100);
        assert_eq!(json["offset"], 0);

        // Scenario 2: Second page
        let second_page = ListFuturesOrdersRequest {
            settle: "USDT".to_string(),
            limit: Some(100),
            offset: Some(100),
            ..Default::default()
        };

        let json = serde_json::to_value(&second_page).unwrap();
        assert_eq!(json["limit"], 100);
        assert_eq!(json["offset"], 100);

        // Scenario 3: Large page
        let large_page = ListFuturesOrdersRequest {
            settle: "USDT".to_string(),
            limit: Some(1000), // Maximum
            ..Default::default()
        };

        let json = serde_json::to_value(&large_page).unwrap();
        assert_eq!(json["limit"], 1000);
    }

    #[test]
    fn test_time_range_filter() {
        let start_time = 1640995200; // 2022-01-01 00:00:00 UTC
        let end_time = 1641081600; // 2022-01-02 00:00:00 UTC

        let request = ListFuturesOrdersRequest {
            settle: "USDT".to_string(),
            from: Some(start_time),
            to: Some(end_time),
            ..Default::default()
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["from"], start_time);
        assert_eq!(json["to"], end_time);
    }

    #[test]
    fn test_contract_filter() {
        let contracts = vec!["BTC_USDT", "ETH_USDT", "SOL_USDT", "MATIC_USDT"];

        for contract in contracts {
            let request = ListFuturesOrdersRequest {
                settle: "USDT".to_string(),
                contract: Some(contract.to_string()),
                ..Default::default()
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["contract"], contract);
        }
    }

    #[test]
    fn test_realistic_query_scenarios() {
        // Scenario 1: Get all open orders for BTC
        let open_btc_orders = ListFuturesOrdersRequest {
            settle: "USDT".to_string(),
            status: Some("open".to_string()),
            contract: Some("BTC_USDT".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_value(&open_btc_orders).unwrap();
        assert_eq!(json["status"], "open");
        assert_eq!(json["contract"], "BTC_USDT");

        // Scenario 2: Get recent finished orders
        let recent_finished = ListFuturesOrdersRequest {
            settle: "USDT".to_string(),
            status: Some("finished".to_string()),
            from: Some(1640995200),
            limit: Some(100),
            ..Default::default()
        };

        let json = serde_json::to_value(&recent_finished).unwrap();
        assert_eq!(json["status"], "finished");
        assert!(json["from"].is_number());

        // Scenario 3: Get all orders with total count
        let all_with_count = ListFuturesOrdersRequest {
            settle: "USDT".to_string(),
            count_total: Some(1),
            limit: Some(50),
            ..Default::default()
        };

        let json = serde_json::to_value(&all_with_count).unwrap();
        assert_eq!(json["count_total"], 1);
        assert_eq!(json["limit"], 50);
    }

    #[test]
    fn test_serialization_omits_null() {
        let request = ListFuturesOrdersRequest {
            settle: "USDT".to_string(),
            status: None,
            contract: None,
            from: None,
            to: None,
            limit: None,
            offset: None,
            count_total: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();

        // Verify None values are not serialized
        assert!(!obj.contains_key("status"));
        assert!(!obj.contains_key("contract"));
        assert!(!obj.contains_key("from"));
        assert!(!obj.contains_key("to"));
        assert!(!obj.contains_key("limit"));
        assert!(!obj.contains_key("offset"));
        assert!(!obj.contains_key("count_total"));
    }
}
