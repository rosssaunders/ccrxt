use serde::{Deserialize, Serialize};

use crate::deribit::{
    EndpointType, JsonRpcResult, PrivateRestClient, RestResult,
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
    /// Unique identifier of the order
    pub order_id: String,

    /// Instrument name of the order (e.g., "BTC-PERPETUAL")
    pub instrument_name: String,

    /// Buy or sell direction
    pub direction: OrderDirection,

    /// Order amount in instrument's units
    pub amount: f64,

    /// Price value or string "market_price" for market orders
    pub price: serde_json::Value, // can be number or "market_price"

    /// Order type (e.g., limit, market)
    pub order_type: OrderType,

    /// Current order state
    pub order_state: OrderState,

    /// Creation timestamp in milliseconds since epoch
    pub creation_timestamp: i64,

    /// Last update timestamp in milliseconds since epoch
    pub last_update_timestamp: i64,

    /// Total filled amount; defaults to 0.0 if absent
    #[serde(default)]
    pub filled_amount: f64,

    /// Average execution price; defaults to 0.0 if absent
    #[serde(default)]
    pub average_price: f64,

    /// Optional order label
    #[serde(default)]
    pub label: Option<String>,

    /// Whether the order is post-only
    #[serde(default)]
    pub post_only: Option<bool>,

    /// Whether the order is reduce-only
    #[serde(default)]
    pub reduce_only: Option<bool>,

    /// Trigger type for conditional orders
    #[serde(default)]
    pub trigger: Option<TriggerType>,

    /// Whether the trigger condition has been met
    #[serde(default)]
    pub triggered: Option<bool>,

    /// Trigger price for conditional orders
    #[serde(default)]
    pub trigger_price: Option<f64>,

    /// Related trigger order id
    #[serde(default)]
    pub trigger_order_id: Option<String>,

    /// Advanced type (e.g., OCO/OTO)
    #[serde(default)]
    pub advanced: Option<AdvancedType>,

    /// USD value of the order, if applicable
    #[serde(default)]
    pub usd: Option<f64>,

    /// Implied volatility for options, if applicable
    #[serde(default)]
    pub implv: Option<f64>,

    /// Number of contracts, for derivatives
    #[serde(default)]
    pub contracts: Option<f64>,

    /// Display amount for iceberg orders
    #[serde(default)]
    pub display_amount: Option<f64>,

    /// Refresh amount for iceberg orders
    #[serde(default)]
    pub refresh_amount: Option<f64>,

    /// Time in force policy
    #[serde(default)]
    pub time_in_force: Option<String>,

    /// Whether the order was placed by the liquidation engine
    #[serde(default)]
    pub is_liquidation: Option<bool>,

    /// Whether this is a secondary leg in an OTO order
    #[serde(default)]
    pub is_secondary_oto: Option<bool>,

    /// Whether this is a primary leg in an OTOCO order
    #[serde(default)]
    pub is_primary_otoco: Option<bool>,

    /// Whether the order was replaced
    #[serde(default)]
    pub replaced: Option<bool>,

    /// Whether the order was automatically replaced by the system
    #[serde(default)]
    pub auto_replaced: Option<bool>,

    /// Reason for cancellation, if applicable
    #[serde(default)]
    pub cancel_reason: Option<CancelReason>,

    /// Whether MMP is enabled for this order
    #[serde(default)]
    pub mmp: Option<bool>,

    /// MMP group identifier
    #[serde(default)]
    pub mmp_group: Option<String>,

    /// Whether the order was cancelled by MMP
    #[serde(default)]
    pub mmp_cancelled: Option<bool>,

    /// Whether the order is a quote
    #[serde(default)]
    pub quote: Option<bool>,

    /// Quote identifier
    #[serde(default)]
    pub quote_id: Option<String>,

    /// Quote set identifier
    #[serde(default)]
    pub quote_set_id: Option<String>,

    /// OCO reference string
    #[serde(default)]
    pub oco_ref: Option<String>,

    /// Linked OTO order ids
    #[serde(default)]
    pub oto_order_ids: Option<Vec<String>>,

    /// Primary order id for linked orders
    #[serde(default)]
    pub primary_order_id: Option<String>,

    /// Application name that placed the order
    #[serde(default)]
    pub app_name: Option<String>,

    /// Whether order was placed via API
    #[serde(default)]
    pub api: Option<bool>,

    /// Whether order was placed via web
    #[serde(default)]
    pub web: Option<bool>,

    /// Whether order was placed via mobile
    #[serde(default)]
    pub mobile: Option<bool>,

    /// Whether the order is a block trade
    #[serde(default)]
    pub block_trade: Option<bool>,

    /// Whether the order reduces risk exposure
    #[serde(default)]
    pub risk_reducing: Option<bool>,

    /// Original order type, if available
    #[serde(default)]
    pub original_order_type: Option<OrderType>,

    /// Whether post-only order would be rejected
    #[serde(default)]
    pub reject_post_only: Option<bool>,

    /// Whether the order is part of a rebalance
    #[serde(default)]
    pub is_rebalance: Option<bool>,

    /// Reference price used for trigger
    #[serde(default)]
    pub trigger_reference_price: Option<f64>,

    /// Offset applied to trigger reference price
    #[serde(default)]
    pub trigger_offset: Option<f64>,

    /// Fill condition for trigger order
    #[serde(default)]
    pub trigger_fill_condition: Option<String>,
}

/// Response for /private/get_open_orders_by_instrument
pub type GetOpenOrdersByInstrumentResponse = JsonRpcResult<Vec<OpenOrder>>;

impl PrivateRestClient {
    /// Retrieves user's open orders for a given instrument on Deribit.
    /// Endpoint: /private/get_open_orders_by_instrument
    ///
    /// [docs](https://docs.deribit.com/v2/#private-get_open_orders_by_instrument)
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
