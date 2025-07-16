use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::{
    OrderResponseType, OrderSide, OrderType, RestResult, SelfTradePreventionMode, TimeInForce,
};

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
    pub discount_asset: Option<String>,

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

        // Build query string with all parameters
        let mut query_params: Vec<(&str, String)> = vec![
            ("symbol", params.symbol),
            ("side", params.side.to_string()),
            ("type", params.order_type.to_string()),
        ];
        
        if let Some(v) = params.time_in_force {
            query_params.push(("timeInForce", v.to_string()));
        }
        if let Some(v) = params.quantity {
            query_params.push(("quantity", v.to_string()));
        }
        if let Some(v) = params.quote_order_qty {
            query_params.push(("quoteOrderQty", v.to_string()));
        }
        if let Some(v) = params.price {
            query_params.push(("price", v.to_string()));
        }
        if let Some(v) = params.new_client_order_id {
            query_params.push(("newClientOrderId", v));
        }
        if let Some(v) = params.strategy_id {
            query_params.push(("strategyId", v.to_string()));
        }
        if let Some(v) = params.strategy_type {
            query_params.push(("strategyType", v.to_string()));
        }
        if let Some(v) = params.stop_price {
            query_params.push(("stopPrice", v.to_string()));
        }
        if let Some(v) = params.trailing_delta {
            query_params.push(("trailingDelta", v.to_string()));
        }
        if let Some(v) = params.iceberg_qty {
            query_params.push(("icebergQty", v.to_string()));
        }
        if let Some(v) = params.new_order_resp_type {
            query_params.push(("newOrderRespType", v.to_string()));
        }
        if let Some(v) = params.self_trade_prevention_mode {
            query_params.push(("selfTradePreventionMode", v.to_string()));
        }
        if let Some(v) = params.compute_commission_rates {
            query_params.push(("computeCommissionRates", v.to_string()));
        }
        if let Some(v) = params.recv_window {
            query_params.push(("recvWindow", v.to_string()));
        }

        // Convert to query string format
        let query_string = serde_urlencoded::to_string(&query_params)
            .map_err(|e| crate::binance::spot::Errors::Error(format!("Failed to encode query string: {e}")))?;

        self.send_request(
            "/api/v3/order/test",
            reqwest::Method::POST,
            Some(&query_string),
            None,
            weight,
            true,
        )
        .await
    }
}
