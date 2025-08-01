use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::cryptocom::{
    ApiResult, ExecInst, OrderType, RefPriceType, RestResult, SpotMarginType, StpInst, StpScope,
    TimeInForce, TradeSide,
};

/// Endpoint path for the create-order API
const CREATE_ORDER_ENDPOINT: &str = "private/create-order";

/// Request parameters for creating a new order
#[derive(Debug, Clone, Serialize)]
pub struct CreateOrderRequest {
    /// Instrument name e.g. BTCUSD-PERP
    pub instrument_name: String,

    /// Order side: BUY or SELL
    pub side: TradeSide,

    /// Order type: LIMIT, MARKET, STOP_LOSS, STOP_LIMIT, TAKE_PROFIT, TAKE_PROFIT_LIMIT
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Price (required for most order types)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Order quantity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,

    /// For MARKET (BUY), STOP_LOSS (BUY), TAKE_PROFIT (BUY) orders only: Amount to spend
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notional: Option<f64>,

    /// Client Order ID (Maximum 36 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,

    /// Execution instruction: POST_ONLY, SMART_POST_ONLY
    /// Note: POST_ONLY and SMART_POST_ONLY cannot be used together.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_inst: Option<Vec<ExecInst>>,

    /// Time in force: GOOD_TILL_CANCEL, IMMEDIATE_OR_CANCEL, FILL_OR_KILL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,

    /// Trigger price required for conditional orders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_price: Option<String>,

    /// Reference price type: MARK_PRICE (default), INDEX_PRICE, LAST_PRICE
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_price_type: Option<RefPriceType>,

    /// Spot margin: SPOT (non-margin order), MARGIN (margin order)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_margin: Option<SpotMarginType>,

    /// Self-trade prevention scope: M (Matches Master or Sub a/c), S (Matches Sub a/c only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_scope: Option<StpScope>,

    /// Self-trade prevention instruction: M (Cancel Maker), T (Cancel Taker), B (Cancel Both)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_inst: Option<StpInst>,

    /// STP ID value: 0 to 32767
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_id: Option<u16>,

    /// Preferred fee token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_instrument_name: Option<String>,
}

/// Response data for creating a new order
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct CreateOrderResult {
    /// Newly created order ID
    pub order_id: String,

    /// Client Order ID if provided, otherwise the nonce
    pub client_oid: String,
}

/// Response wrapper for create-order endpoint
pub type CreateOrderResponse = ApiResult<CreateOrderResult>;

impl RestClient {
    /// Creates a new BUY or SELL Order on the Exchange
    ///
    /// This call is asynchronous, so the response is simply a confirmation of the request.
    /// The user.order subscription can be used to check when the order is successfully created.
    ///
    /// See: <https://exchange-docs.crypto.com/derivatives/index.html>
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The order parameters
    ///
    /// # Returns
    /// Order ID and client order ID
    pub async fn create_order(
        &self,
        request: CreateOrderRequest,
    ) -> RestResult<CreateOrderResponse> {
        self.send_signed_request(CREATE_ORDER_ENDPOINT, request)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rest::secrets::ExposableSecret;
    use serde_json::json;

    use super::*;

    /// A plain text implementation of ExposableSecret for testing purposes.
    #[derive(Clone)]
    #[allow(dead_code)]
    struct PlainTextSecret {
        secret: String,
    }

    impl ExposableSecret for PlainTextSecret {
        fn expose_secret(&self) -> String {
            self.secret.clone()
        }
    }

    impl PlainTextSecret {
        #[allow(dead_code)]
        fn new(secret: String) -> Self {
            Self { secret }
        }
    }

    #[test]
    fn test_create_order_request_serialization() {
        let request = CreateOrderRequest {
            instrument_name: "BTCUSD-PERP".to_string(),
            side: TradeSide::Sell,
            order_type: OrderType::Limit,
            price: Some("50000.5".to_string()),
            quantity: Some("1".to_string()),
            notional: None,
            client_oid: Some("c5f682ed-7108-4f1c-b755-972fcdca0f02".to_string()),
            exec_inst: Some(vec![ExecInst::PostOnly]),
            time_in_force: Some(TimeInForce::FillOrKill),
            ref_price: None,
            ref_price_type: None,
            spot_margin: None,
            stp_scope: None,
            stp_inst: None,
            stp_id: None,
            fee_instrument_name: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("instrument_name").unwrap(), "BTCUSD-PERP");
        assert_eq!(serialized.get("side").unwrap(), "SELL");
        assert_eq!(serialized.get("type").unwrap(), "LIMIT");
        assert_eq!(serialized.get("price").unwrap(), "50000.5");
        assert_eq!(serialized.get("quantity").unwrap(), "1");
        assert_eq!(
            serialized.get("client_oid").unwrap(),
            "c5f682ed-7108-4f1c-b755-972fcdca0f02"
        );
        assert_eq!(serialized.get("exec_inst").unwrap()[0], "POST_ONLY");
        assert_eq!(serialized.get("time_in_force").unwrap(), "FILL_OR_KILL");
    }

    #[test]
    fn test_create_order_request_market_buy_with_notional() {
        let request = CreateOrderRequest {
            instrument_name: "BTCUSD-PERP".to_string(),
            side: TradeSide::Buy,
            order_type: OrderType::Market,
            price: None,
            quantity: None,
            notional: Some(1000.0),
            client_oid: None,
            exec_inst: None,
            time_in_force: None,
            ref_price: None,
            ref_price_type: None,
            spot_margin: None,
            stp_scope: None,
            stp_inst: None,
            stp_id: None,
            fee_instrument_name: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("instrument_name").unwrap(), "BTCUSD-PERP");
        assert_eq!(serialized.get("side").unwrap(), "BUY");
        assert_eq!(serialized.get("type").unwrap(), "MARKET");
        assert_eq!(serialized.get("notional").unwrap(), 1000.0);
        assert!(!serialized.as_object().unwrap().contains_key("price"));
        assert!(!serialized.as_object().unwrap().contains_key("quantity"));
    }

    #[test]
    fn test_create_order_request_conditional_order() {
        let request = CreateOrderRequest {
            instrument_name: "BTCUSD-PERP".to_string(),
            side: TradeSide::Sell,
            order_type: OrderType::StopLoss,
            price: Some("49000.0".to_string()),
            quantity: Some("0.5".to_string()),
            notional: None,
            client_oid: None,
            exec_inst: None,
            time_in_force: None,
            ref_price: Some("50000.0".to_string()),
            ref_price_type: Some(RefPriceType::MarkPrice),
            spot_margin: None,
            stp_scope: None,
            stp_inst: None,
            stp_id: None,
            fee_instrument_name: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("type").unwrap(), "STOP_LOSS");
        assert_eq!(serialized.get("ref_price").unwrap(), "50000.0");
        assert_eq!(serialized.get("ref_price_type").unwrap(), "MARK_PRICE");
    }

    #[test]
    fn test_create_order_request_with_stp_settings() {
        let request = CreateOrderRequest {
            instrument_name: "BTCUSD-PERP".to_string(),
            side: TradeSide::Buy,
            order_type: OrderType::Limit,
            price: Some("50000.0".to_string()),
            quantity: Some("1".to_string()),
            notional: None,
            client_oid: None,
            exec_inst: None,
            time_in_force: None,
            ref_price: None,
            ref_price_type: None,
            spot_margin: None,
            stp_scope: Some(StpScope::MasterOrSubAccount),
            stp_inst: Some(StpInst::CancelMaker),
            stp_id: Some(100),
            fee_instrument_name: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("stp_scope").unwrap(), "M");
        assert_eq!(serialized.get("stp_inst").unwrap(), "M");
        assert_eq!(serialized.get("stp_id").unwrap(), 100);
    }

    #[test]
    fn test_create_order_request_with_smart_post_only() {
        let request = CreateOrderRequest {
            instrument_name: "ETHUSD-PERP".to_string(),
            side: TradeSide::Buy,
            order_type: OrderType::Limit,
            price: Some("2500.0".to_string()),
            quantity: Some("0.5".to_string()),
            notional: None,
            client_oid: Some("smart_post_only_test".to_string()),
            exec_inst: Some(vec![ExecInst::SmartPostOnly]),
            time_in_force: Some(TimeInForce::GoodTillCancel),
            ref_price: None,
            ref_price_type: None,
            spot_margin: None,
            stp_scope: None,
            stp_inst: None,
            stp_id: None,
            fee_instrument_name: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("instrument_name").unwrap(), "ETHUSD-PERP");
        assert_eq!(serialized.get("exec_inst").unwrap()[0], "SMART_POST_ONLY");
        assert_eq!(serialized.get("time_in_force").unwrap(), "GOOD_TILL_CANCEL");
    }

    #[test]
    fn test_exec_inst_enum_serialization() {
        // Test serialization of both POST_ONLY and SMART_POST_ONLY
        let post_only = ExecInst::PostOnly;
        let smart_post_only = ExecInst::SmartPostOnly;

        let post_only_serialized = serde_json::to_string(&post_only).unwrap();
        let smart_post_only_serialized = serde_json::to_string(&smart_post_only).unwrap();

        assert_eq!(post_only_serialized, "\"POST_ONLY\"");
        assert_eq!(smart_post_only_serialized, "\"SMART_POST_ONLY\"");

        // Test deserialization
        let post_only_deserialized: ExecInst = serde_json::from_str("\"POST_ONLY\"").unwrap();
        let smart_post_only_deserialized: ExecInst =
            serde_json::from_str("\"SMART_POST_ONLY\"").unwrap();

        assert_eq!(post_only_deserialized, ExecInst::PostOnly);
        assert_eq!(smart_post_only_deserialized, ExecInst::SmartPostOnly);
    }

    #[test]
    fn test_create_order_response_structure() {
        let response_json = json!({
            "code": 0,
            "id": 1,
            "result": {
                "order_id": "18342311",
                "client_oid": "c5f682ed-7108-4f1c-b755-972fcdca0f02"
            }
        });

        let response: CreateOrderResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.result.order_id, "18342311");
        assert_eq!(
            response.result.client_oid,
            "c5f682ed-7108-4f1c-b755-972fcdca0f02"
        );
    }
}
