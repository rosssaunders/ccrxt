use serde::{Deserialize, Serialize};

use crate::bullish::{EndpointType, RestResult, private::rest::client::RestClient};

/// Endpoint URL for order-related commands (cancellations, amend, etc.)
const COMMAND_ENDPOINT: &str = "/v2/command";

/// Command type for cancelling an order
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub enum CommandType {
    #[serde(rename = "V3CancelOrder")]
    #[default]
    V3CancelOrder,
}

/// Request parameters for cancelling an order.
///
/// Exactly one of `order_id` or `client_order_id` should be provided.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    /// The command type, it must be 'V3CancelOrder'.
    #[serde(rename = "commandType")]
    pub command_type: CommandType,

    /// Unique order ID.
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    /// Client-generated unique ID.
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,

    /// Market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market.
    pub symbol: String,

    /// Unique trading account ID.
    #[serde(rename = "tradingAccountId")]
    pub trading_account_id: String,
}

/// Response for cancel order request
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    /// Acknowledgment message
    pub message: String,

    /// Request ID
    #[serde(rename = "requestId")]
    pub request_id: String,

    /// Cancelled order ID (if provided/known)
    #[serde(rename = "orderId")]
    pub order_id: Option<String>,

    /// Client order ID (if provided/known)
    #[serde(rename = "clientOrderId")]
    pub client_order_id: Option<String>,
}

impl RestClient {
    /// Cancel order (V3CancelOrder)
    ///
    /// Submits a cancel command for a specific order by `orderId` or `clientOrderId`.
    ///
    /// [docs](https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#post-/v2/command#cancellations)
    ///
    /// Rate limit: private orders category
    ///
    /// # Arguments
    /// * `request` - Cancel order command with ids and routing info
    ///
    /// # Returns
    /// Command acknowledgement including requestId and optionally order/client ids
    pub async fn cancel_order(
        &mut self,
        request: CancelOrderRequest,
    ) -> RestResult<CancelOrderResponse> {
        self.send_post_request(COMMAND_ENDPOINT, request, EndpointType::PrivateOrders)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_order_request_serialization_with_order_id() {
        let req = CancelOrderRequest {
            command_type: CommandType::V3CancelOrder,
            order_id: Some("297735387747975680".to_string()),
            client_order_id: None,
            symbol: "BTCUSDC".to_string(),
            trading_account_id: "111000000000001".to_string(),
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("V3CancelOrder"));
        assert!(json.contains("orderId"));
        assert!(!json.contains("clientOrderId\":null"));
    }

    #[test]
    fn test_cancel_order_request_serialization_with_client_order_id() {
        let req = CancelOrderRequest {
            command_type: CommandType::V3CancelOrder,
            order_id: None,
            client_order_id: Some("633914459442118656".to_string()),
            symbol: "BTCUSDC".to_string(),
            trading_account_id: "111000000000001".to_string(),
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("clientOrderId"));
        assert!(!json.contains("orderId\":null"));
    }
}
