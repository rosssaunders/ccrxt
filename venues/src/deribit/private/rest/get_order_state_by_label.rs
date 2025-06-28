//! Retrieves the state of recent orders with a given label via /private/get_order_state_by_label
//!
//! This module defines the request/response types and logic for the Deribit private endpoint.

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::enums::{
    CancelReason, Currency, OrderDirection, OrderState, OrderType, TriggerType,
};
use crate::deribit::{EndpointType, RestResult, TimeInForce, TriggerFillCondition};

/// Request for /private/get_order_state_by_label
#[derive(Debug, Clone, Serialize)]
pub struct GetOrderStateByLabelRequest {
    /// The currency symbol (e.g., BTC, ETH, USDC, USDT, EURR).
    #[serde(rename = "currency")]
    pub currency: Currency,

    /// User defined label for the order (maximum 64 characters). Optional.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// Response for /private/get_order_state_by_label
#[derive(Debug, Clone, Deserialize)]
pub struct GetOrderStateByLabelResponse {
    /// The id that was sent in the request
    #[serde(rename = "id")]
    pub id: u64,

    /// The JSON-RPC version (2.0)
    #[serde(rename = "jsonrpc")]
    pub jsonrpc: String,

    /// The result array containing order state information
    #[serde(rename = "result")]
    pub result: Vec<OrderStateByLabelInfo>,
}

/// Represents the state of an order returned by /private/get_order_state_by_label
#[derive(Debug, Clone, Deserialize)]
pub struct OrderStateByLabelInfo {
    /// Unique order identifier
    #[serde(rename = "order_id")]
    pub order_id: String,

    /// Order state
    #[serde(rename = "order_state")]
    pub order_state: OrderState,

    /// Order type
    #[serde(rename = "order_type")]
    pub order_type: OrderType,

    /// Direction: buy or sell
    #[serde(rename = "direction")]
    pub direction: OrderDirection,

    /// The instrument name
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,

    /// The requested order size
    #[serde(rename = "amount")]
    pub amount: f64,

    /// Filled amount of the order
    #[serde(rename = "filled_amount")]
    pub filled_amount: f64,

    /// Average fill price of the order
    #[serde(rename = "average_price")]
    pub average_price: Option<f64>,

    /// Price in base currency or "market_price" in case of open trigger market orders
    #[serde(rename = "price")]
    pub price: Option<serde_json::Value>,

    /// Order time in force
    #[serde(rename = "time_in_force")]
    pub time_in_force: Option<TimeInForce>,

    /// User defined label (up to 64 characters)
    #[serde(rename = "label")]
    pub label: Option<String>,

    /// The timestamp (milliseconds since the Unix epoch) when the order was created
    #[serde(rename = "creation_timestamp")]
    pub creation_timestamp: Option<u64>,

    /// The timestamp (milliseconds since the Unix epoch) when the order was last updated
    #[serde(rename = "last_update_timestamp")]
    pub last_update_timestamp: Option<u64>,

    /// Cancel reason, if applicable
    #[serde(rename = "cancel_reason")]
    pub cancel_reason: Option<CancelReason>,

    /// Trigger type (only for trigger orders)
    #[serde(rename = "trigger")]
    pub trigger: Option<TriggerType>,

    /// Trigger price (Only for future trigger orders)
    #[serde(rename = "trigger_price")]
    pub trigger_price: Option<f64>,

    /// Trigger order id (Only for orders created by triggered orders)
    #[serde(rename = "trigger_order_id")]
    pub trigger_order_id: Option<String>,

    /// Triggered (whether the trigger order has been triggered)
    #[serde(rename = "triggered")]
    pub triggered: Option<bool>,

    /// Trigger fill condition (Only for linked order types)
    #[serde(rename = "trigger_fill_condition")]
    pub trigger_fill_condition: Option<TriggerFillCondition>,

    /// Contracts (order size in contract units, optional)
    #[serde(rename = "contracts")]
    pub contracts: Option<f64>,

    /// The initial display amount of iceberg order (optional)
    #[serde(rename = "refresh_amount")]
    pub refresh_amount: Option<f64>,

    /// The actual display amount of iceberg order (optional)
    #[serde(rename = "display_amount")]
    pub display_amount: Option<f64>,

    /// Option price in USD (Only if advanced="usd")
    #[serde(rename = "usd")]
    pub usd: Option<f64>,

    /// Implied volatility in percent (Only if advanced="implv")
    #[serde(rename = "implv")]
    pub implv: Option<f64>,

    /// Advanced type (Only for options)
    #[serde(rename = "advanced")]
    pub advanced: Option<String>,

    /// OTO order ids (orders that will be triggered if the order is filled)
    #[serde(rename = "oto_order_ids")]
    pub oto_order_ids: Option<Vec<String>>,

    /// Whether the order is a quote
    #[serde(rename = "quote")]
    pub quote: Option<bool>,

    /// The same QuoteID as supplied in the private/mass_quote request
    #[serde(rename = "quote_id")]
    pub quote_id: Option<String>,

    /// Identifier of the QuoteSet supplied in the private/mass_quote request
    #[serde(rename = "quote_set_id")]
    pub quote_set_id: Option<String>,

    /// Name of the MMP group supplied in the private/mass_quote request
    #[serde(rename = "mmp_group")]
    pub mmp_group: Option<String>,

    /// True if the order is a MMP order
    #[serde(rename = "mmp")]
    pub mmp: Option<bool>,

    /// True if order was cancelled by mmp trigger
    #[serde(rename = "mmp_cancelled")]
    pub mmp_cancelled: Option<bool>,

    /// True if the order is an order that can be triggered by another order
    #[serde(rename = "is_secondary_oto")]
    pub is_secondary_oto: Option<bool>,

    /// True if the order is an order that can trigger an OCO pair
    #[serde(rename = "is_primary_otoco")]
    pub is_primary_otoco: Option<bool>,

    /// Unique reference that identifies a one_cancels_others (OCO) pair
    #[serde(rename = "oco_ref")]
    pub oco_ref: Option<String>,

    /// Unique order identifier for primary order
    #[serde(rename = "primary_order_id")]
    pub primary_order_id: Option<String>,

    /// True if order is post-only
    #[serde(rename = "post_only")]
    pub post_only: Option<bool>,

    /// True if order has reject_post_only flag
    #[serde(rename = "reject_post_only")]
    pub reject_post_only: Option<bool>,

    /// True if created with API
    #[serde(rename = "api")]
    pub api: Option<bool>,

    /// True if created with Mobile Application
    #[serde(rename = "mobile")]
    pub mobile: Option<bool>,

    /// The name of the application that placed the order on behalf of the user
    #[serde(rename = "app_name")]
    pub app_name: Option<String>,

    /// True if created via Deribit frontend
    #[serde(rename = "web")]
    pub web: Option<bool>,

    /// True if the order was edited
    #[serde(rename = "replaced")]
    pub replaced: Option<bool>,

    /// True if last modification of the order was performed by the pricing engine
    #[serde(rename = "auto_replaced")]
    pub auto_replaced: Option<bool>,

    /// True if the order is marked as risk reducing
    #[serde(rename = "risk_reducing")]
    pub risk_reducing: Option<bool>,

    /// True if the order is reduce-only
    #[serde(rename = "reduce_only")]
    pub reduce_only: Option<bool>,

    /// True if order was automatically created during liquidation
    #[serde(rename = "is_liquidation")]
    pub is_liquidation: Option<bool>,

    /// True if order was automatically created during cross-collateral balance restoration
    #[serde(rename = "is_rebalance")]
    pub is_rebalance: Option<bool>,

    /// Block trade flag
    #[serde(rename = "block_trade")]
    pub block_trade: Option<bool>,

    /// The price of the given trigger at the time when the order was placed
    #[serde(rename = "trigger_reference_price")]
    pub trigger_reference_price: Option<f64>,

    /// The maximum deviation from the price peak beyond which the order will be triggered
    #[serde(rename = "trigger_offset")]
    pub trigger_offset: Option<f64>,
}

impl RestClient {
    /// Retrieve the state of recent orders with a given label.
    ///
    /// [Deribit API docs](https://docs.deribit.com/#private-get_order_state_by_label)
    ///
    /// # Arguments
    /// * `params` - Parameters for the request (currency, optional label)
    ///
    /// # Returns
    /// * `GetOrderStateByLabelResponse` containing the order state(s)
    pub async fn get_order_state_by_label(
        &self,
        request: GetOrderStateByLabelRequest,
    ) -> RestResult<GetOrderStateByLabelResponse> {
        self.send_signed_request(
            "private/get_order_state_by_label",
            &request,
            EndpointType::MatchingEngine,
        )
        .await
    }
}

// Unit tests for serialization/deserialization
#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;
    use crate::deribit::enums::Currency;

    #[test]
    fn test_serialize_request() {
        let req = GetOrderStateByLabelRequest {
            currency: Currency::BTC,
            label: Some("mylabel".to_string()),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("currency"));
        assert!(json.contains("BTC"));
        assert!(json.contains("mylabel"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"
        {
            "jsonrpc": "2.0",
            "id": 1,
            "result": [
                {
                    "order_id": "12345",
                    "order_state": "open",
                    "order_type": "limit",
                    "direction": "buy",
                    "instrument_name": "BTC-PERPETUAL",
                    "amount": 100.0,
                    "filled_amount": 50.0
                }
            ]
        }
        "#;
        let resp: GetOrderStateByLabelResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.result[0].order_id, "12345");
        assert_eq!(resp.result[0].order_state.to_string(), "open");
    }
}
