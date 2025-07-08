//! Retrieves user's open orders for a given instrument on Deribit.
//! Endpoint: /private/get_open_orders_by_instrument

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{
    EndpointType, JsonRpcResult, RestResult,
    enums::{
        AdvancedType, CancelReason, OpenOrdersOrderType, OrderDirection, OrderState, OrderType,
        TriggerType,
    },
};

/// REST API endpoint constant
const GET_OPEN_ORDERS_BY_INSTRUMENT_ENDPOINT: &str = "private/get_open_orders_by_instrument";

/// Request for /private/get_open_orders_by_instrument
#[derive(Debug, Clone, Serialize)]
pub struct GetOpenOrdersByInstrumentRequest {
    /// Instrument name (required)
    pub instrument_name: String,
    /// Order type filter (optional, default: all)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<OpenOrdersOrderType>,
}

/// Open order object returned by the endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct OpenOrder {
    pub order_id: String,
    pub instrument_name: String,
    pub direction: OrderDirection,
    pub amount: f64,
    pub price: serde_json::Value, // can be number or "market_price"
    pub order_type: OrderType,
    pub order_state: OrderState,
    pub creation_timestamp: i64,
    pub last_update_timestamp: i64,
    #[serde(default)]
    pub filled_amount: f64,
    #[serde(default)]
    pub average_price: f64,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub post_only: Option<bool>,
    #[serde(default)]
    pub reduce_only: Option<bool>,
    #[serde(default)]
    pub trigger: Option<TriggerType>,
    #[serde(default)]
    pub triggered: Option<bool>,
    #[serde(default)]
    pub trigger_price: Option<f64>,
    #[serde(default)]
    pub trigger_order_id: Option<String>,
    #[serde(default)]
    pub advanced: Option<AdvancedType>,
    #[serde(default)]
    pub usd: Option<f64>,
    #[serde(default)]
    pub implv: Option<f64>,
    #[serde(default)]
    pub contracts: Option<f64>,
    #[serde(default)]
    pub display_amount: Option<f64>,
    #[serde(default)]
    pub refresh_amount: Option<f64>,
    #[serde(default)]
    pub time_in_force: Option<String>,
    #[serde(default)]
    pub is_liquidation: Option<bool>,
    #[serde(default)]
    pub is_secondary_oto: Option<bool>,
    #[serde(default)]
    pub is_primary_otoco: Option<bool>,
    #[serde(default)]
    pub replaced: Option<bool>,
    #[serde(default)]
    pub auto_replaced: Option<bool>,
    #[serde(default)]
    pub cancel_reason: Option<CancelReason>,
    #[serde(default)]
    pub mmp: Option<bool>,
    #[serde(default)]
    pub mmp_group: Option<String>,
    #[serde(default)]
    pub mmp_cancelled: Option<bool>,
    #[serde(default)]
    pub quote: Option<bool>,
    #[serde(default)]
    pub quote_id: Option<String>,
    #[serde(default)]
    pub quote_set_id: Option<String>,
    #[serde(default)]
    pub oco_ref: Option<String>,
    #[serde(default)]
    pub oto_order_ids: Option<Vec<String>>,
    #[serde(default)]
    pub primary_order_id: Option<String>,
    #[serde(default)]
    pub app_name: Option<String>,
    #[serde(default)]
    pub api: Option<bool>,
    #[serde(default)]
    pub web: Option<bool>,
    #[serde(default)]
    pub mobile: Option<bool>,
    #[serde(default)]
    pub block_trade: Option<bool>,
    #[serde(default)]
    pub risk_reducing: Option<bool>,
    #[serde(default)]
    pub original_order_type: Option<OrderType>,
    #[serde(default)]
    pub reject_post_only: Option<bool>,
    #[serde(default)]
    pub is_rebalance: Option<bool>,
    #[serde(default)]
    pub trigger_reference_price: Option<f64>,
    #[serde(default)]
    pub trigger_offset: Option<f64>,
    #[serde(default)]
    pub trigger_fill_condition: Option<String>,
}

/// Response for /private/get_open_orders_by_instrument
pub type GetOpenOrdersByInstrumentResponse = JsonRpcResult<Vec<OpenOrder>>;

impl RestClient {
    /// Retrieves user's open orders for a given instrument on Deribit.
    /// Endpoint: /private/get_open_orders_by_instrument
    pub async fn get_open_orders_by_instrument(
        &self,
        request: GetOpenOrdersByInstrumentRequest,
    ) -> RestResult<GetOpenOrdersByInstrumentResponse> {
        self.send_signed_request(
            GET_OPEN_ORDERS_BY_INSTRUMENT_ENDPOINT,
            &request,
            EndpointType::MatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    /// REST API endpoint constant
    use super::*;

    #[test]
    fn test_request_serialization() {
        let req = GetOpenOrdersByInstrumentRequest {
            instrument_name: "BTC-PERPETUAL".to_string(),
            r#type: Some(OpenOrdersOrderType::Limit),
        };
        let j = serde_json::to_string(&req).unwrap();
        assert!(j.contains("\"instrument_name\":\"BTC-PERPETUAL\""));
        assert!(j.contains("\"type\":\"limit\""));
    }

    #[test]
    fn test_response_deserialization() {
        let data = json!({
            "id": 42,
            "jsonrpc": "2.0",
            "result": [{
                "order_id": "abc123",
                "instrument_name": "BTC-PERPETUAL",
                "direction": "buy",
                "amount": 100.0,
                "price": "market_price",
                "order_type": "limit",
                "order_state": "open",
                "creation_timestamp": 1234567890,
                "last_update_timestamp": 1234567891
            }]
        });
        let resp: GetOpenOrdersByInstrumentResponse = serde_json::from_value(data).unwrap();
        assert_eq!(resp.result.len(), 1);
        assert_eq!(resp.result[0].order_id, "abc123");
    }
}
