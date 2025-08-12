use serde::{Deserialize, Serialize};

use crate::bullish::{EndpointType, RestResult, private::rest::client::RestClient};

const COMMAND_ENDPOINT: &str = "/v2/command";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub enum CommandType {
    #[serde(rename = "V1DelayedCancelAllOrders")]
    #[default]
    V1DelayedCancelAllOrders,

    #[serde(rename = "V1UnsetDelayedCancelAllOrders")]
    V1UnsetDelayedCancelAllOrders,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DelaySeconds {
    #[serde(rename = "5")]
    Five,
    #[serde(rename = "10")]
    Ten,
    #[serde(rename = "15")]
    Fifteen,
    #[serde(rename = "20")]
    Twenty,
    #[serde(rename = "25")]
    TwentyFive,
    #[serde(rename = "30")]
    Thirty,
    #[serde(rename = "40")]
    Forty,
    #[serde(rename = "50")]
    Fifty,
    #[serde(rename = "60")]
    Sixty,
}


/// Request parameters for delayed cancel all orders.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DelayedCancelAllOrdersRequest {
    /// The command type, it must be 'V1DelayedCancelAllOrders'.
    #[serde(rename = "commandType")]
    pub command_type: CommandType,

    /// Optional cancel request id (stringified u64).
    #[serde(rename = "cancelId", skip_serializing_if = "Option::is_none")]
    pub cancel_id: Option<String>,

    /// Delay the cancel-all-orders request by seconds. Allowed values: 5,10,15,20,25,30,40,50,60
    #[serde(rename = "delayBySeconds")]
    pub delay_by_seconds: DelaySeconds,

    /// Unique trading account ID.
    #[serde(rename = "tradingAccountId")]
    pub trading_account_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelayedCancelAllOrdersResponse {
    pub message: String,
    #[serde(rename = "requestId")]
    pub request_id: String,
}

impl RestClient {
    /// Delayed cancel all orders (V1DelayedCancelAllOrders)
    ///
    /// Schedules a cancel-all within a delay window.
    ///
    /// [docs]: https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#post-/v2/command#cancellations
    pub async fn delayed_cancel_all_orders(
        &mut self,
        request: DelayedCancelAllOrdersRequest,
    ) -> RestResult<DelayedCancelAllOrdersResponse> {
        self.send_post_request(COMMAND_ENDPOINT, request, EndpointType::PrivateOrders)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delayed_cancel_all_orders_request_serialization() {
        let req = DelayedCancelAllOrdersRequest {
            command_type: CommandType::V1DelayedCancelAllOrders,
            cancel_id: Some("123456789".to_string()),
            delay_by_seconds: DelaySeconds::Five,
            trading_account_id: "111000000000001".to_string(),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("V1DelayedCancelAllOrders"));
        assert!(json.contains("delayBySeconds"));
    }

    #[test]
    fn test_delay_seconds_serialization() {
        assert_eq!(serde_json::to_string(&DelaySeconds::Five).unwrap(), "\"5\"");
        assert_eq!(serde_json::to_string(&DelaySeconds::Ten).unwrap(), "\"10\"");
        assert_eq!(
            serde_json::to_string(&DelaySeconds::Fifteen).unwrap(),
            "\"15\""
        );
        assert_eq!(
            serde_json::to_string(&DelaySeconds::Twenty).unwrap(),
            "\"20\""
        );
        assert_eq!(
            serde_json::to_string(&DelaySeconds::TwentyFive).unwrap(),
            "\"25\""
        );
        assert_eq!(
            serde_json::to_string(&DelaySeconds::Thirty).unwrap(),
            "\"30\""
        );
        assert_eq!(
            serde_json::to_string(&DelaySeconds::Forty).unwrap(),
            "\"40\""
        );
        assert_eq!(
            serde_json::to_string(&DelaySeconds::Fifty).unwrap(),
            "\"50\""
        );
        assert_eq!(
            serde_json::to_string(&DelaySeconds::Sixty).unwrap(),
            "\"60\""
        );
    }
}
