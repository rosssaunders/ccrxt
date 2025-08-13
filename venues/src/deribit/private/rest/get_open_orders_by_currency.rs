use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{
    AdvancedType, CancelReason, Currency, EndpointType, InstrumentKind, JsonRpcResult,
    OrderDirection, OrderState, RestResult, TriggerType,
};

/// REST API endpoint constant
const GET_OPEN_ORDERS_BY_CURRENCY_ENDPOINT: &str = "private/get_open_orders_by_currency";

/// Order type filter for get_open_orders_by_currency endpoint
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenOrderType {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "limit")]
    Limit,
    #[serde(rename = "trigger_all")]
    TriggerAll,
    #[serde(rename = "stop_all")]
    StopAll,
    #[serde(rename = "stop_limit")]
    StopLimit,
    #[serde(rename = "stop_market")]
    StopMarket,
    #[serde(rename = "take_all")]
    TakeAll,
    #[serde(rename = "take_limit")]
    TakeLimit,
    #[serde(rename = "take_market")]
    TakeMarket,
    #[serde(rename = "trailing_all")]
    TrailingAll,
    #[serde(rename = "trailing_stop")]
    TrailingStop,
}

impl std::fmt::Display for OpenOrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenOrderType::All => write!(f, "all"),
            OpenOrderType::Limit => write!(f, "limit"),
            OpenOrderType::TriggerAll => write!(f, "trigger_all"),
            OpenOrderType::StopAll => write!(f, "stop_all"),
            OpenOrderType::StopLimit => write!(f, "stop_limit"),
            OpenOrderType::StopMarket => write!(f, "stop_market"),
            OpenOrderType::TakeAll => write!(f, "take_all"),
            OpenOrderType::TakeLimit => write!(f, "take_limit"),
            OpenOrderType::TakeMarket => write!(f, "take_market"),
            OpenOrderType::TrailingAll => write!(f, "trailing_all"),
            OpenOrderType::TrailingStop => write!(f, "trailing_stop"),
        }
    }
}

/// Request parameters for get_open_orders_by_currency
#[derive(Debug, Clone, Serialize)]
pub struct GetOpenOrdersByCurrencyRequest {
    /// The currency symbol
    pub currency: Currency,

    /// Instrument kind, if not provided instruments of all kinds are considered
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<InstrumentKind>,

    /// Order type, default - all
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub order_type: Option<OpenOrderType>,
}

/// Individual open order information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenOrder {
    /// If order is a quote. Present only if true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<bool>,

    /// Whether the trigger order has been triggered
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggered: Option<bool>,

    /// Optional field with value `true` added only when created with Mobile Application
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<bool>,

    /// The name of the application that placed the order on behalf of the user (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,

    /// Implied volatility in percent. (Only if `advanced="implv"`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implv: Option<f64>,

    /// The initial display amount of iceberg order. Iceberg order display amount will be refreshed to that value after match consuming actual display amount. Absent for other types of orders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_amount: Option<f64>,

    /// Option price in USD (Only if `advanced="usd"`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usd: Option<f64>,

    /// The Ids of the orders that will be triggered if the order is filled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oto_order_ids: Option<Vec<String>>,

    /// `true` if created with API
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<bool>,

    /// Average fill price of the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_price: Option<f64>,

    /// advanced type: "usd" or "implv" (Only for options; field is omitted if not applicable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced: Option<AdvancedType>,

    /// Unique order identifier
    pub order_id: String,

    /// `true` for post-only orders only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,

    /// Filled amount of the order. For perpetual and futures the filled_amount is in USD units, for options - in units or corresponding cryptocurrency contracts, e.g., BTC or ETH.
    pub filled_amount: f64,

    /// Trigger type (only for trigger orders). Allowed values: "index_price", "mark_price", "last_price".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<TriggerType>,

    /// Id of the trigger order that created the order (Only for orders that were created by triggered orders).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_order_id: Option<String>,

    /// Direction: `buy`, or `sell`
    pub direction: OrderDirection,

    /// It represents the order size in contract units. (Optional, may be absent in historical data).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contracts: Option<f64>,

    /// `true` if the order is an order that can be triggered by another order, otherwise not present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_secondary_oto: Option<bool>,

    /// `true` if the order was edited (by user or - in case of advanced options orders - by pricing engine), otherwise `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaced: Option<bool>,

    /// Name of the MMP group supplied in the `private/mass_quote` request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mmp_group: Option<String>,

    /// `true` if the order is a MMP order, otherwise `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mmp: Option<bool>,

    /// The timestamp (milliseconds since the Unix epoch)
    pub last_update_timestamp: i64,

    /// The timestamp (milliseconds since the Unix epoch)
    pub creation_timestamp: i64,

    /// Enumerated reason behind cancel `"user_request"`, `"autoliquidation"`, `"cancel_on_disconnect"`, `"risk_mitigation"`, `"pme_risk_reduction"` (portfolio margining risk reduction), `"pme_account_locked"` (portfolio margining account locked per currency), `"position_locked"`, `"mmp_trigger"` (market maker protection), `"mmp_config_curtailment"` (market maker configured quantity decreased), `"edit_post_only_reject"` (cancelled on edit because of `reject_post_only` setting), `"oco_other_closed"` (the oco order linked to this order was closed), `"oto_primary_closed"` (the oto primary order that was going to trigger this order was cancelled), `"settlement"` (closed because of a settlement)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_reason: Option<CancelReason>,

    /// `true` if order was cancelled by mmp trigger (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mmp_cancelled: Option<bool>,

    /// The same QuoteID as supplied in the `private/mass_quote` request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,

    /// Order state: `"open"`, `"filled"`, `"rejected"`, `"cancelled"`, `"untriggered"`
    pub order_state: OrderState,

    /// Optional (only for spot). `true` if order was automatically created during cross-collateral balance restoration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rebalance: Option<bool>,

    /// `true` if order has `reject_post_only` flag (field is present only when `post_only` is `true`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_post_only: Option<bool>,

    /// User defined label (up to 64 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// Optional (not added for spot). `true` if order was automatically created during liquidation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_liquidation: Option<bool>,

    /// Price in base currency or "market_price" in case of open trigger market orders
    pub price: serde_json::Value,

    /// `true` if created via Deribit frontend (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<bool>,

    /// Order time in force: `"good_til_cancelled"`, `"good_til_day"`, `"fill_or_kill"` or `"immediate_or_cancel"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,

    /// The price of the given trigger at the time when the order was placed (Only for trailing trigger orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_reference_price: Option<f64>,

    /// The actual display amount of iceberg order. Absent for other types of orders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_amount: Option<f64>,

    /// Order type: `"limit"`, `"market"`, `"stop_limit"`, `"stop_market"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,

    /// `true` if the order is an order that can trigger an OCO pair, otherwise not present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_primary_otoco: Option<bool>,

    /// Original order type. Optional field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_order_type: Option<String>,

    /// `true` if order made from block_trade trade, added only in that case.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_trade: Option<bool>,

    /// Trigger price (Only for future trigger orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<f64>,

    /// Unique reference that identifies a one_cancels_others (OCO) pair.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oco_ref: Option<String>,

    /// The maximum deviation from the price peak beyond which the order will be triggered (Only for trailing trigger orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_offset: Option<f64>,

    /// Identifier of the QuoteSet supplied in the `private/mass_quote` request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_set_id: Option<String>,

    /// Options, advanced orders only - `true` if last modification of the order was performed by the pricing engine, otherwise `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_replaced: Option<bool>,

    /// Optional (not added for spot). `true` for reduce-only orders only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,

    /// It represents the requested order size. For perpetual and inverse futures the amount is in USD units. For options and linear futures and it is the underlying base currency coin.
    pub amount: f64,

    /// `true` if the order is marked by the platform as a risk reducing order (can apply only to orders placed by PM users), otherwise `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_reducing: Option<bool>,

    /// Unique instrument identifier
    pub instrument_name: String,

    /// The fill condition of the linked order (Only for linked order types), default: `first_hit`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_fill_condition: Option<String>,

    /// Unique order identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_order_id: Option<String>,
}

/// Response for get_open_orders_by_currency endpoint
pub type GetOpenOrdersByCurrencyResponse = JsonRpcResult<Vec<OpenOrder>>;

impl RestClient {
    /// Retrieves list of user's open orders.
    ///
    /// This is a private method; it can only be used after authentication.
    /// Scope: `trade:read`
    ///
    /// [docs](https://docs.deribit.com/v2/#private-get_open_orders_by_currency)
    ///
    /// Rate limit: Non-matching engine rate limits apply (500 credits)
    ///
    /// # Arguments
    /// * `request` - Request parameters including currency, instrument kind and order type filter
    ///
    /// # Returns
    /// List of user's open orders
    pub async fn get_open_orders_by_currency(
        &self,
        request: GetOpenOrdersByCurrencyRequest,
    ) -> RestResult<GetOpenOrdersByCurrencyResponse> {
        self.send_signed_request(
            GET_OPEN_ORDERS_BY_CURRENCY_ENDPOINT,
            &request,
            EndpointType::MatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use rest::secrets::{ExposableSecret, SecretString};
    use serde_json::{Value, json};

    use super::*;
    use crate::deribit::{AccountTier, private::rest::credentials::Credentials};

    #[test]
    fn test_request_serialization_minimal() {
        let req = GetOpenOrdersByCurrencyRequest {
            currency: Currency::BTC,
            kind: None,
            order_type: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        let value: Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value.get("currency").unwrap(), "BTC");
        assert!(!value.as_object().unwrap().contains_key("kind"));
        assert!(!value.as_object().unwrap().contains_key("type"));
    }

    #[test]
    fn test_request_serialization_full() {
        let req = GetOpenOrdersByCurrencyRequest {
            currency: Currency::ETH,
            kind: Some(InstrumentKind::Future),
            order_type: Some(OpenOrderType::StopLimit),
        };
        let json = serde_json::to_string(&req).unwrap();
        let value: Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value.get("currency").unwrap(), "ETH");
        assert_eq!(value.get("kind").unwrap(), "future");
        assert_eq!(value.get("type").unwrap(), "stop_limit");
    }

    #[test]
    fn test_response_deserialization() {
        let resp_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": [{
                "order_id": "ABC-123",
                "filled_amount": 0.0,
                "direction": "buy",
                "last_update_timestamp": 1,
                "creation_timestamp": 1,
                "order_state": "open",
                "price": "100",
                "amount": 10.0,
                "instrument_name": "BTC-PERPETUAL"
            }]
        });
        let resp: GetOpenOrdersByCurrencyResponse = serde_json::from_value(resp_json).unwrap();
        assert_eq!(resp.result.len(), 1);
        let order = &resp.result[0];
        assert_eq!(order.order_id, "ABC-123");
        assert_eq!(order.order_state, OrderState::Open);
        assert_eq!(order.amount, 10.0);
    }

    #[tokio::test]
    async fn test_method_exists() {
        let credentials = Credentials {
            api_key: SecretString::from("key".to_string()),
            api_secret: SecretString::from("secret".to_string()),
        };
        let http_client = Arc::new(rest::native::NativeHttpClient::default());
        let limiter = crate::deribit::RateLimiter::new(AccountTier::Tier4);
        let rest_client = RestClient::new(
            credentials,
            "https://test.deribit.com",
            limiter,
            http_client,
        );
        let _ = RestClient::get_open_orders_by_currency;
        let _ = &rest_client;
        println!("get_open_orders_by_currency method available");
    }
}
