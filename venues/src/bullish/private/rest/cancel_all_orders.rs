use serde::{Deserialize, Serialize};

use crate::bullish::private::rest::client::RestClient;
use crate::bullish::{EndpointType, RestResult};

const COMMAND_ENDPOINT: &str = "/v2/command";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum CommandType {
    #[serde(rename = "V1CancelAllOrders")]
    V1CancelAllOrders,
}

impl Default for CommandType {
    fn default() -> Self {
        CommandType::V1CancelAllOrders
    }
}

/// Request parameters for cancelling all orders for a trading account.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrdersRequest {
    /// The command type, it must be 'V1CancelAllOrders'.
    #[serde(rename = "commandType")]
    pub command_type: CommandType,

    /// Unique trading account ID.
    #[serde(rename = "tradingAccountId")]
    pub trading_account_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrdersResponse {
    pub message: String,
    
    #[serde(rename = "requestId")]
    pub request_id: String,
}

impl RestClient {
    /// Cancel all orders (V1CancelAllOrders)
    ///
    /// Cancels all outstanding orders for the trading account.
    ///
    /// [docs]: https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#post-/v2/command#cancellations
    pub async fn cancel_all_orders(
        &mut self,
        request: CancelAllOrdersRequest,
    ) -> RestResult<CancelAllOrdersResponse> {
        self
            .send_post_request(COMMAND_ENDPOINT, request, EndpointType::PrivateOrders)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_all_orders_request_serialization() {
        let req = CancelAllOrdersRequest {
            command_type: CommandType::V1CancelAllOrders,
            trading_account_id: "111000000000001".to_string(),
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("V1CancelAllOrders"));
        assert!(json.contains("tradingAccountId"));
    }
}
