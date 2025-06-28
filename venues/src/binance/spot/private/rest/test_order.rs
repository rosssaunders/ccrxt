use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::spot::{
    OrderResponseType, OrderSide, OrderType, RestResult, SelfTradePreventionMode, TimeInForce,
};

use super::client::RestClient;

/// Request parameters for testing a new order
#[derive(Debug, Clone, Serialize)]
pub struct TestNewOrderRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order side (BUY or SELL)
    #[serde(rename = "side")]
    pub side: OrderSide,

    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Time in force
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,

    /// Order quantity
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Decimal>,

    /// Quote order quantity
    #[serde(rename = "quoteOrderQty", skip_serializing_if = "Option::is_none")]
    pub quote_order_qty: Option<Decimal>,

    /// Order price
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,

    /// Client order ID
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<String>,

    /// Strategy ID
    #[serde(rename = "strategyId", skip_serializing_if = "Option::is_none")]
    pub strategy_id: Option<u32>,

    /// Strategy type
    #[serde(rename = "strategyType", skip_serializing_if = "Option::is_none")]
    pub strategy_type: Option<u32>,

    /// Stop price
    #[serde(rename = "stopPrice", skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<Decimal>,

    /// Trailing delta
    #[serde(rename = "trailingDelta", skip_serializing_if = "Option::is_none")]
    pub trailing_delta: Option<u32>,

    /// Iceberg quantity
    #[serde(rename = "icebergQty", skip_serializing_if = "Option::is_none")]
    pub iceberg_qty: Option<Decimal>,

    /// Response type
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    pub new_order_resp_type: Option<OrderResponseType>,

    /// Self-trade prevention mode
    #[serde(
        rename = "selfTradePreventionMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,

    /// Compute commission rates
    #[serde(
        rename = "computeCommissionRates",
        skip_serializing_if = "Option::is_none"
    )]
    pub compute_commission_rates: Option<bool>,

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Test order response (empty object)
#[derive(Debug, Clone, Deserialize)]
pub struct TestOrderResponse {}

/// Commission rates response (when computeCommissionRates=true)
#[derive(Debug, Clone, Deserialize)]
pub struct CommissionRatesResponse {
    /// Standard commission rates
    #[serde(rename = "standardCommission")]
    pub standard_commission: CommissionRates,

    /// Tax commission rates
    #[serde(rename = "taxCommission")]
    pub tax_commission: CommissionRates,

    /// Discount information
    #[serde(rename = "discount")]
    pub discount: Discount,
}

/// Commission rates structure
#[derive(Debug, Clone, Deserialize)]
pub struct CommissionRates {
    /// Maker commission rate
    #[serde(rename = "maker")]
    pub maker: Decimal,

    /// Taker commission rate
    #[serde(rename = "taker")]
    pub taker: Decimal,

    /// Buyer commission rate
    #[serde(rename = "buyer")]
    pub buyer: Decimal,

    /// Seller commission rate
    #[serde(rename = "seller")]
    pub seller: Decimal,
}

/// Discount information
#[derive(Debug, Clone, Deserialize)]
pub struct Discount {
    /// Enable buy back
    #[serde(rename = "enabledForAccount")]
    pub enabled_for_account: bool,

    /// Enable buy back for symbol
    #[serde(rename = "enabledForSymbol")]
    pub enabled_for_symbol: bool,

    /// Discount asset
    #[serde(rename = "discountAsset")]
    pub discount_asset: String,

    /// Discount rate
    #[serde(rename = "discount")]
    pub discount: Decimal,
}

impl RestClient {
    /// Test new order creation and signature/recvWindow
    ///
    /// Test new order creation and signature/recvWindow long.
    /// Creates and validates a new order but does not send it into the matching engine.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#test-new-order--trade)
    /// Method: POST /api/v3/order/test
    /// Weight: 1 (without computeCommissionRates), 20 (with computeCommissionRates)
    /// Security: TRADE
    pub async fn test_new_order(
        &self,
        params: TestNewOrderRequest,
    ) -> RestResult<serde_json::Value> {
        let weight = if params.compute_commission_rates.unwrap_or(false) {
            20
        } else {
            1
        };

        let body_params: Vec<(&str, String)> = vec![
            ("symbol", params.symbol),
            ("side", params.side.to_string()),
            ("type", params.order_type.to_string()),
        ]
        .into_iter()
        .chain(params.time_in_force.map(|v| ("timeInForce", v.to_string())))
        .chain(params.quantity.map(|v| ("quantity", v.to_string())))
        .chain(
            params
                .quote_order_qty
                .map(|v| ("quoteOrderQty", v.to_string())),
        )
        .chain(params.price.map(|v| ("price", v.to_string())))
        .chain(params.new_client_order_id.map(|v| ("newClientOrderId", v)))
        .chain(params.strategy_id.map(|v| ("strategyId", v.to_string())))
        .chain(
            params
                .strategy_type
                .map(|v| ("strategyType", v.to_string())),
        )
        .chain(params.stop_price.map(|v| ("stopPrice", v.to_string())))
        .chain(
            params
                .trailing_delta
                .map(|v| ("trailingDelta", v.to_string())),
        )
        .chain(params.iceberg_qty.map(|v| ("icebergQty", v.to_string())))
        .chain(
            params
                .new_order_resp_type
                .map(|v| ("newOrderRespType", v.to_string())),
        )
        .chain(
            params
                .self_trade_prevention_mode
                .map(|v| ("selfTradePreventionMode", v.to_string())),
        )
        .chain(
            params
                .compute_commission_rates
                .map(|v| ("computeCommissionRates", v.to_string())),
        )
        .chain(params.recv_window.map(|v| ("recvWindow", v.to_string())))
        .collect();

        let body: Vec<(&str, &str)> = body_params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        self.send_request(
            "/api/v3/order/test",
            reqwest::Method::POST,
            None,
            Some(&body),
            weight,
            true,
        )
        .await
    }
}
