use serde::{Deserialize, Serialize};

use crate::bybit::{enums::*, EndpointType, RestResult};

use super::client::RestClient;

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

impl GetExecutionListRequest {
    /// Create a new get execution list request
    pub fn new(category: Category) -> Self {
        Self {
            category,
            symbol: None,
            order_id: None,
            order_link_id: None,
            base_coin: None,
            start_time: None,
            end_time: None,
            exec_type: None,
            limit: None,
            cursor: None,
        }
    }

    /// Filter by symbol
    pub fn symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    /// Filter by order ID
    pub fn order_id(mut self, order_id: String) -> Self {
        self.order_id = Some(order_id);
        self
    }

    /// Filter by order link ID
    pub fn order_link_id(mut self, order_link_id: String) -> Self {
        self.order_link_id = Some(order_link_id);
        self
    }

    /// Filter by base coin
    pub fn base_coin(mut self, base_coin: String) -> Self {
        self.base_coin = Some(base_coin);
        self
    }

    /// Set start time (timestamp in milliseconds)
    pub fn start_time(mut self, start_time: u64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /// Set end time (timestamp in milliseconds)
    pub fn end_time(mut self, end_time: u64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    /// Filter by execution type
    pub fn exec_type(mut self, exec_type: ExecType) -> Self {
        self.exec_type = Some(exec_type);
        self
    }

    /// Set page limit (1-100, default 50)
    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set pagination cursor
    pub fn cursor(mut self, cursor: String) -> Self {
        self.cursor = Some(cursor);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_execution_list_request_builder() {
        let request = GetExecutionListRequest::new(Category::Linear)
            .symbol("BTCUSDT".to_string())
            .exec_type(ExecType::Trade)
            .limit(25)
            .start_time(1640995200000);

        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.symbol, Some("BTCUSDT".to_string()));
        assert_eq!(request.exec_type, Some(ExecType::Trade));
        assert_eq!(request.limit, Some(25));
        assert_eq!(request.start_time, Some(1640995200000));
    }

    #[test]
    fn test_get_execution_list_request_serialization() {
        let request = GetExecutionListRequest::new(Category::Spot)
            .base_coin("BTC".to_string())
            .limit(50);

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"category\":\"spot\""));
        assert!(json.contains("\"baseCoin\":\"BTC\""));
        assert!(json.contains("\"limit\":50"));
        assert!(!json.contains("symbol")); // Should be skipped when None
    }
}