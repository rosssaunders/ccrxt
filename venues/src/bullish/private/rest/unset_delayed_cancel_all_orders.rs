use serde::{Deserialize, Serialize};

use crate::bullish::private::rest::client::RestClient;
use crate::bullish::{EndpointType, RestResult};

const COMMAND_ENDPOINT: &str = "/v2/command";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum CommandType {
    #[serde(rename = "V1DelayedCancelAllOrders")]
    V1DelayedCancelAllOrders,

    #[serde(rename = "V1UnsetDelayedCancelAllOrders")]
    V1UnsetDelayedCancelAllOrders,
}

impl Default for CommandType {
    fn default() -> Self {
        CommandType::V1DelayedCancelAllOrders
    }
}

/// Request parameters for unsetting delayed cancel all orders.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnsetDelayedCancelAllOrdersRequest {
    /// The command type, it must be 'V1UnsetDelayedCancelAllOrders'.
    #[serde(rename = "commandType")]
    pub command_type: CommandType,

    /// Unique trading account ID.
    #[serde(rename = "tradingAccountId")]
    pub trading_account_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnsetDelayedCancelAllOrdersResponse {
    pub message: String,

    #[serde(rename = "requestId")]
    pub request_id: String,
}

impl RestClient {
    /// Unset delayed cancel all orders (V1UnsetDelayedCancelAllOrders)
    ///
    /// Cancels the scheduled cancel-all operation.
    ///
    /// [docs]: https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#post-/v2/command#cancellations
    pub async fn unset_delayed_cancel_all_orders(
        &mut self,
        request: UnsetDelayedCancelAllOrdersRequest,
    ) -> RestResult<UnsetDelayedCancelAllOrdersResponse> {
        self.send_post_request(COMMAND_ENDPOINT, request, EndpointType::PrivateOrders)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unset_delayed_cancel_all_orders_request_serialization() {
        let req = UnsetDelayedCancelAllOrdersRequest {
            command_type: CommandType::V1UnsetDelayedCancelAllOrders,
            trading_account_id: "111000000000001".to_string(),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("V1UnsetDelayedCancelAllOrders"));
    }
}
