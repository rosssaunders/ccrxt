use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bybit::{EndpointType, RestResult, enums::*};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetExecutionListRequest {
    pub category: Category,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_type: Option<ExecType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionInfo {
    pub symbol: String,
    pub order_id: String,
    pub order_link_id: String,
    pub side: Side,
    pub order_qty: String,
    pub order_price: String,
    pub order_type: OrderType,
    pub stop_order_type: String,
    pub exec_fee: String,
    pub exec_id: String,
    pub exec_price: String,
    pub exec_qty: String,
    pub exec_type: ExecType,
    pub exec_value: String,
    pub exec_time: String,
    pub is_maker: bool,
    pub fee_rate: String,
    pub trade_iv: String,
    pub mark_iv: String,
    pub mark_price: String,
    pub index_price: String,
    pub underlying_price: String,
    pub block_trade_id: String,
    pub closed_size: String,
    pub seq: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetExecutionListData {
    pub category: Category,
    pub list: Vec<ExecutionInfo>,
    pub next_page_cursor: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetExecutionListResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: GetExecutionListData,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

impl RestClient {
    /// Get execution list (trade history)
    ///
    /// Query execution records sorted by execTime in descending order.
    ///
    /// # Arguments
    /// * `request` - The get execution list request parameters
    ///
    /// # Returns
    /// A result containing the execution list response or an error
    pub async fn get_execution_list(
        &self,
        request: GetExecutionListRequest,
    ) -> RestResult<GetExecutionListResponse> {
        self.send_signed_request(
            "/v5/execution/list",
            reqwest::Method::GET,
            request,
            EndpointType::Trade,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_execution_list_request_direct_construction() {
        let request = GetExecutionListRequest {
            category: Category::Linear,
            symbol: Some("BTCUSDT".to_string()),
            exec_type: Some(ExecType::Trade),
            limit: Some(25),
            start_time: Some(1640995200000),
            order_id: None,
            order_link_id: None,
            base_coin: None,
            end_time: None,
            cursor: None,
        };

        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.symbol, Some("BTCUSDT".to_string()));
        assert_eq!(request.exec_type, Some(ExecType::Trade));
        assert_eq!(request.limit, Some(25));
        assert_eq!(request.start_time, Some(1640995200000));
    }

    #[test]
    fn test_get_execution_list_request_serialization() {
        let request = GetExecutionListRequest {
            category: Category::Spot,
            base_coin: Some("BTC".to_string()),
            limit: Some(50),
            symbol: None,
            order_id: None,
            order_link_id: None,
            start_time: None,
            end_time: None,
            exec_type: None,
            cursor: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"category\":\"spot\""));
        assert!(json.contains("\"baseCoin\":\"BTC\""));
        assert!(json.contains("\"limit\":50"));
        assert!(!json.contains("symbol")); // Should be skipped when None
    }
}
