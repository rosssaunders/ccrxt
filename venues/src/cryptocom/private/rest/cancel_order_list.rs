use serde::{Deserialize, Serialize};

use super::client::RestClient;

/// Individual order to cancel in a list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOrderListItem {
    /// Instrument name (e.g., ETH_CRO, BTC_USDT)
    pub instrument_name: String,
    /// Order ID to cancel
    pub order_id: String,
}

/// Request for canceling an order list (LIST type)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOrderListRequest {
    /// Contingency type (must be LIST)
    pub contingency_type: crate::cryptocom::enums::ContingencyType,
    /// List of orders to cancel
    pub order_list: Vec<CancelOrderListItem>,
}

/// Result for individual order cancellation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCancellationResult {
    /// Index of the order in the request (starts from 0)
    pub index: u32,
    /// Status code (0 if successful)
    pub code: i32,
    /// Error message (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Response for canceling a list of orders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOrderListResponse {
    /// List of order cancellation results
    pub result_list: Vec<OrderCancellationResult>,
}

impl RestClient {
    /// Cancel a list of orders
    ///
    /// Cancels multiple orders in a single request using the LIST contingency type.
    ///
    /// See: <https://exchange-docs.crypto.com/derivatives/index.html>
    ///
    /// Rate limit: 10 requests per second per user
    ///
    /// # Arguments
    /// * `request` - The order list cancellation request
    ///
    /// # Returns
    /// Response with cancellation results for each order
    pub async fn cancel_order_list(&self, request: CancelOrderListRequest) -> crate::cryptocom::RestResult<CancelOrderListResponse> {
        self.send_signed_request("private/cancel-order-list", request)
            .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_cancel_order_list_item_structure() {
        let item_json = json!({
            "instrument_name": "ETH_CRO",
            "order_id": "2015106383706015873"
        });
        let item: CancelOrderListItem = serde_json::from_value(item_json).unwrap();
        assert_eq!(item.instrument_name, "ETH_CRO");
        assert_eq!(item.order_id, "2015106383706015873");
    }

    #[test]
    fn test_cancel_order_list_request_structure() {
        let request_json = json!({
            "contingency_type": "LIST",
            "order_list": [
                {"instrument_name": "ETH_CRO", "order_id": "2015106383706015873"},
                {"instrument_name": "BTC_USDT", "order_id": "2015106383706015874"}
            ]
        });
        let request: CancelOrderListRequest = serde_json::from_value(request_json).unwrap();
        assert_eq!(
            request.contingency_type,
            crate::cryptocom::enums::ContingencyType::List
        );
        assert_eq!(request.order_list.len(), 2);
        assert_eq!(request.order_list[0].instrument_name, "ETH_CRO");
        assert_eq!(request.order_list[1].order_id, "2015106383706015874");
    }

    #[test]
    fn test_cancel_order_list_response_structure() {
        let response_json = json!({
            "result_list": [
                {"index": 0, "code": 0, "message": null},
                {"index": 1, "code": 1001, "message": "Order not found"}
            ]
        });
        let response: CancelOrderListResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.result_list.len(), 2);
        assert_eq!(response.result_list[0].code, 0);
        assert_eq!(
            response.result_list[1].message.as_deref(),
            Some("Order not found")
        );
    }
}
