use rust_decimal::Decimal;
use serde::Serialize;

use super::client::RestClient;
use crate::binance::spot::{
    OrderResponseType, OrderSide, OrderType, RestResult, SelfTradePreventionMode, TimeInForce,
};

/// Request parameters for SOR order
#[derive(Debug, Clone, Serialize)]
pub struct SorOrderRequest {
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

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

impl RestClient {
    /// Place a SOR order
    ///
    /// Place an order using smart order routing.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#new-sor-order--trade)
    /// Method: POST /api/v3/sor/order
    /// Weight: 1
    /// Security: TRADE
    pub async fn new_sor_order(&self, params: SorOrderRequest) -> RestResult<serde_json::Value> {
        let body_params: Vec<(&str, String)> = vec![
            ("symbol", params.symbol),
            ("side", params.side.to_string()),
            ("type", params.order_type.to_string()),
        ]
        .into_iter()
        .chain(params.time_in_force.map(|v| ("timeInForce", v.to_string())))
        .chain(params.quantity.map(|v| ("quantity", v.to_string())))
        .chain(params.price.map(|v| ("price", v.to_string())))
        .chain(params.new_client_order_id.map(|v| ("newClientOrderId", v)))
        .chain(params.strategy_id.map(|v| ("strategyId", v.to_string())))
        .chain(
            params
                .strategy_type
                .map(|v| ("strategyType", v.to_string())),
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
        .chain(params.recv_window.map(|v| ("recvWindow", v.to_string())))
        .collect();

        let body: Vec<(&str, &str)> = body_params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        self.send_request(
            "/api/v3/sor/order",
            reqwest::Method::POST,
            None,
            Some(&body),
            1,
            true,
        )
        .await
    }

    /// Test SOR order creation
    ///
    /// Test SOR order creation and signature/recvWindow.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#test-new-sor-order--trade)
    /// Method: POST /api/v3/sor/order/test
    /// Weight: 1 (without computeCommissionRates), 20 (with computeCommissionRates)
    /// Security: TRADE
    pub async fn test_sor_order(
        &self,
        params: SorOrderRequest,
        compute_commission_rates: Option<bool>,
    ) -> RestResult<serde_json::Value> {
        let weight = if compute_commission_rates.unwrap_or(false) {
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
        .chain(params.price.map(|v| ("price", v.to_string())))
        .chain(params.new_client_order_id.map(|v| ("newClientOrderId", v)))
        .chain(params.strategy_id.map(|v| ("strategyId", v.to_string())))
        .chain(
            params
                .strategy_type
                .map(|v| ("strategyType", v.to_string())),
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
        .chain(compute_commission_rates.map(|v| ("computeCommissionRates", v.to_string())))
        .chain(params.recv_window.map(|v| ("recvWindow", v.to_string())))
        .collect();

        let body: Vec<(&str, &str)> = body_params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        self.send_request(
            "/api/v3/sor/order/test",
            reqwest::Method::POST,
            None,
            Some(&body),
            weight,
            true,
        )
        .await
    }
}
