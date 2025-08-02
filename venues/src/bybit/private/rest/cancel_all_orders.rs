use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bybit::{EndpointType, RestResult, enums::*};

/// Endpoint URL for cancelling all orders
const CANCEL_ALL_ORDERS_ENDPOINT: &str = "/v5/order/cancel-all";

/// Request parameters for cancelling all orders.
///
/// Cancels all open orders that match the specified criteria. You can filter by
/// symbol, base coin, settle coin, order type, and stop order type.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrdersRequest {
    /// Product type (linear, spot, option, inverse)
    pub category: Category,

    /// Trading symbol (e.g., "BTCUSDT"). If not provided, cancels all symbols
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Base coin (e.g., "BTC"). Valid for option only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,

    /// Settle coin (e.g., "USDT"). Valid for linear and inverse only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_coin: Option<String>,

    /// Order filter for conditional orders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_filter: Option<OrderFilter>,

    /// Stop order type filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_order_type: Option<StopOrderType>,
}

/// Information about a cancelled order.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelledOrder {
    /// Unique order ID of the cancelled order
    pub order_id: String,

    /// User-defined order ID of the cancelled order
    pub order_link_id: String,
}

/// Data from the cancel all orders response.
#[derive(Debug, Clone, Deserialize)]
pub struct CancelAllOrdersData {
    /// List of cancelled orders
    pub list: Vec<CancelledOrder>,

    /// Success status message
    pub success: String,
}

/// Response from the cancel all orders API endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct CancelAllOrdersResponse {
    /// Return code (0 indicates success)
    #[serde(rename = "retCode")]
    pub ret_code: i32,

    /// Return message
    #[serde(rename = "retMsg")]
    pub ret_msg: String,

    /// Cancel all orders result data
    pub result: CancelAllOrdersData,

    /// Extended information (varies by endpoint)
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,

    /// Response timestamp in milliseconds
    pub time: u64,
}

impl RestClient {
    /// Cancel all orders
    ///
    /// Cancel all open orders that match the specified criteria. You can filter by
    /// symbol, base coin, settle coin, order type, and stop order type.
    ///
    /// [API Documentation](https://bybit-exchange.github.io/docs/v5/order/cancel-all)
    ///
    /// Rate limit: 10 requests per second per UID
    ///
    /// # Arguments
    /// * `request` - The cancellation request with optional filters for symbol, coin, and order type
    ///
    /// # Returns
    /// A result containing the list of cancelled orders and success status
    pub async fn cancel_all_orders(
        &self,
        request: CancelAllOrdersRequest,
    ) -> RestResult<CancelAllOrdersResponse> {
        self.send_post_request(CANCEL_ALL_ORDERS_ENDPOINT, request, EndpointType::Trade)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_all_orders_request_by_symbol() {
        let request = CancelAllOrdersRequest {
            category: Category::Linear,
            symbol: Some("BTCUSDT".to_string()),
            base_coin: None,
            settle_coin: None,
            order_filter: None,
            stop_order_type: None,
        };

        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.symbol, Some("BTCUSDT".to_string()));
        assert!(request.base_coin.is_none());
        assert!(request.settle_coin.is_none());
    }

    #[test]
    fn test_cancel_all_orders_request_by_base_coin() {
        let request = CancelAllOrdersRequest {
            category: Category::Spot,
            symbol: None,
            base_coin: Some("BTC".to_string()),
            settle_coin: None,
            order_filter: Some(OrderFilter::Order),
            stop_order_type: None,
        };

        assert_eq!(request.category, Category::Spot);
        assert!(request.symbol.is_none());
        assert_eq!(request.base_coin, Some("BTC".to_string()));
        assert_eq!(request.order_filter, Some(OrderFilter::Order));
    }

    #[test]
    fn test_cancel_all_orders_request_serialization() {
        let request = CancelAllOrdersRequest {
            category: Category::Linear,
            symbol: None,
            base_coin: None,
            settle_coin: Some("USDT".to_string()),
            order_filter: None,
            stop_order_type: Some(StopOrderType::Stop),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"category\":\"linear\""));
        assert!(json.contains("\"settleCoin\":\"USDT\""));
        assert!(json.contains("\"stopOrderType\":\"Stop\""));
        assert!(!json.contains("symbol")); // Should be skipped when None
    }
}
