use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bitmart::{OrderSide, RestResult, rate_limit::EndpointType};

const CANCEL_ALL_ORDERS_ENDPOINT: &str = "/spot/v4/cancel_all";

/// Request parameters for canceling all orders.
#[derive(Debug, Clone, Serialize, Default)]
pub struct CancelAllOrdersRequest {
    /// Trading pair (optional, e.g., BTC_USDT).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Order side (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<OrderSide>,
}

/// Response for canceling all orders.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelAllOrdersResponse {
    // Empty response data - success is indicated by HTTP status code
}

impl RestClient {
    /// Cancel All Order(v4)
    ///
    /// Cancels all outstanding orders for a symbol and/or side.
    ///
    /// [docs]: https://developer-pro.bitmart.com/en/spot/#cancel-all-order-v4-signed
    ///
    /// Rate limit: UID-based, 1 times/3 sec
    ///
    /// # Arguments
    /// * `request` - The cancel all request parameters
    ///
    /// # Returns
    /// Empty response - success indicated by HTTP status
    pub async fn cancel_all_orders(
        &self,
        request: CancelAllOrdersRequest,
    ) -> RestResult<CancelAllOrdersResponse> {
        self.send_post_signed_request(CANCEL_ALL_ORDERS_ENDPOINT, request,
            EndpointType::SpotTrading,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_all_orders_request() {
        let request = CancelAllOrdersRequest {
            symbol: Some("BTC_USDT".to_string()),
            side: Some(OrderSide::Buy),
        };

        assert_eq!(request.symbol, Some("BTC_USDT".to_string()));
        assert_eq!(request.side, Some(OrderSide::Buy));
    }

    #[test]
    fn test_cancel_all_orders_request_empty() {
        let request = CancelAllOrdersRequest {
            symbol: None,
            side: None,
        };

        assert!(request.symbol.is_none());
        assert!(request.side.is_none());
    }

    #[test]
    fn test_cancel_all_orders_request_default() {
        let request = CancelAllOrdersRequest::default();

        assert!(request.symbol.is_none());
        assert!(request.side.is_none());
    }

    #[test]
    fn test_cancel_all_orders_response_structure() {
        let response = CancelAllOrdersResponse {};

        // This is an empty struct, just test it can be constructed
        drop(response);
    }

    #[test]
    fn test_clone_derives() {
        let request = CancelAllOrdersRequest {
            symbol: Some("BTC_USDT".to_string()),
            side: Some(OrderSide::Buy),
        };

        let cloned_request = request.clone();
        assert_eq!(request.symbol, cloned_request.symbol);
        assert_eq!(request.side, cloned_request.side);

        let response = CancelAllOrdersResponse {};
        let cloned_response = response.clone();

        // Just verify the clone works
        drop(cloned_response);
    }
}
