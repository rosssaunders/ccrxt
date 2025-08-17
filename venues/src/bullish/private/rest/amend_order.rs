use serde::{Deserialize, Serialize};

use crate::bullish::{EndpointType, RestResult, private::rest::client::RestClient};

const COMMAND_ENDPOINT: &str = "/v2/command";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub enum CommandType {
    #[serde(rename = "V1AmendOrder")]
    #[default]
    V1AmendOrder,
}

/// Request parameters for amending an order.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderRequest {
    /// The command type, it must be 'V1AmendOrder'.
    #[serde(rename = "commandType")]
    pub command_type: CommandType,

    /// Unique order ID (optional; required if clientOrderId not provided).
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    /// Market symbol.
    pub symbol: String,

    /// Order type to amend to, allowed values: LIMIT | POST_ONLY
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,

    /// New price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Client order id (optional; required if orderId not provided).
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,

    /// New quantity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,

    /// Unique trading account ID.
    #[serde(rename = "tradingAccountId")]
    pub trading_account_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderResponse {
    pub message: String,

    #[serde(rename = "requestId")]
    pub request_id: String,

    #[serde(rename = "orderId")]
    pub order_id: Option<String>,

    #[serde(rename = "clientOrderId")]
    pub client_order_id: Option<String>,
}

impl RestClient {
    /// Amend order (V1AmendOrder)
    ///
    /// Ability to amend price/quantity/type on eligible orders.
    ///
    /// [docs](https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#post-/v2/command#amend)
    pub async fn amend_order(
        &mut self,
        request: AmendOrderRequest,
    ) -> RestResult<AmendOrderResponse> {
        self.send_post_request(COMMAND_ENDPOINT, request, EndpointType::PrivateOrders)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amend_order_request_serialization() {
        let req = AmendOrderRequest {
            command_type: CommandType::V1AmendOrder,
            order_id: Some("297735387747975680".to_string()),
            symbol: "BTCUSDC".to_string(),
            order_type: Some("LIMIT".to_string()),
            price: Some("1.00000000".to_string()),
            client_order_id: Some("633914459442118656".to_string()),
            quantity: Some("1.00000000".to_string()),
            trading_account_id: "111000000000001".to_string(),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("V1AmendOrder"));
        assert!(json.contains("price"));
    }
}
