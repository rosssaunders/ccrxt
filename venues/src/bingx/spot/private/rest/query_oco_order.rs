use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::spot::{EndpointType, RestResult, enums::OcoOrderStatus};

const QUERY_OCO_ORDER_ENDPOINT: &str = "/openApi/spot/v1/oco/order";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryOcoOrderRequest {
    /// OCO order ID
    pub order_list_id: Option<i64>,
    /// Original client order ID
    pub orig_client_order_id: Option<String>,
    /// Timestamp in ms
    pub recv_window: Option<i64>,
    /// Timestamp in ms
    pub timestamp: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryOcoOrderResponse {
    pub order_list_id: i64,
    pub contingency_type: String,
    pub list_status_type: OcoOrderStatus,
    pub list_order_status: OcoOrderStatus,
    pub list_client_order_id: String,
    pub transaction_time: i64,
    pub symbol: String,
    pub orders: Vec<OcoSubOrder>,
    pub order_reports: Vec<OcoOrderReport>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OcoSubOrder {
    pub symbol: String,
    pub order_id: i64,
    pub client_order_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OcoOrderReport {
    pub symbol: String,
    pub order_id: i64,
    pub order_list_id: i64,
    pub client_order_id: String,
    pub transaction_time: i64,
    pub price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cummulative_quote_qty: String,
    pub status: String,
    pub time_in_force: String,
    pub r#type: String,
    pub side: String,
    pub stop_price: Option<String>,
}

impl RestClient {
    /// Query OCO order information
    ///
    /// Retrieves information about a specific OCO order.
    ///
    /// # Arguments
    /// * `request` - The query OCO order request
    ///
    /// # Returns
    /// A result containing the OCO order information or an error
    ///
    /// # Rate Limits
    /// - UID rate limit: 10/s
    ///
    /// # API Permissions
    /// - Spot trading permission required
    pub async fn query_oco_order(
        &self,
        request: &QueryOcoOrderRequest,
    ) -> RestResult<QueryOcoOrderResponse> {
        self.send_get_signed_request(QUERY_OCO_ORDER_ENDPOINT, request, EndpointType::Trading,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_oco_order_request_serialization() {
        let request = QueryOcoOrderRequest {
            order_list_id: Some(123456789),
            orig_client_order_id: None,
            recv_window: Some(5000),
            timestamp: 1640995200000,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"orderListId\":123456789"));
        assert!(json.contains("\"recvWindow\":5000"));
        assert!(json.contains("\"timestamp\":1640995200000"));
    }

    #[test]
    fn test_query_oco_order_response_deserialization() {
        let json = r#"
        {
            "orderListId": 123456789,
            "contingencyType": "OCO",
            "listStatusType": "EXEC_STARTED",
            "listOrderStatus": "EXECUTING",
            "listClientOrderId": "client123",
            "transactionTime": 1640995200000,
            "symbol": "BTCUSDT",
            "orders": [
                {
                    "symbol": "BTCUSDT",
                    "orderId": 987654321,
                    "clientOrderId": "order1"
                }
            ],
            "orderReports": [
                {
                    "symbol": "BTCUSDT",
                    "orderId": 987654321,
                    "orderListId": 123456789,
                    "clientOrderId": "order1",
                    "transactionTime": 1640995200000,
                    "price": "50000.00",
                    "origQty": "0.001",
                    "executedQty": "0.0",
                    "cummulativeQuoteQty": "0.0",
                    "status": "NEW",
                    "timeInForce": "GTC",
                    "type": "LIMIT",
                    "side": "BUY"
                }
            ]
        }
        "#;

        let response: QueryOcoOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_list_id, 123456789);
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.orders.len(), 1);
        assert_eq!(response.order_reports.len(), 1);
    }
}
