use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::{RestResult, enums::*};

/// Endpoint path for modifying an order on Binance USDM Futures.
const MODIFY_ORDER_ENDPOINT: &str = "/fapi/v1/order";

/// Request parameters for the Modify Order endpoint.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyOrderRequest {
    /// Order ID to modify. Either this or `orig_client_order_id` must be provided. If both are sent, `order_id` prevails.
    /// Securely stored and expected as SecretString.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Original client order ID. Either this or `order_id` must be provided.
    /// Securely stored and expected as SecretString.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<Cow<'static, str>>,

    /// Trading symbol (e.g., "BTCUSDT"). Required.
    pub symbol: Cow<'static, str>,

    /// Order side. Required. Valid values: BUY, SELL.
    pub side: OrderSide,

    /// Order quantity. Required. Cannot be sent with `close_position = true`.
    pub quantity: Cow<'static, str>,

    /// Order price. Required.
    pub price: Cow<'static, str>,

    /// Price match mode. Only available for LIMIT/STOP/TAKE_PROFIT orders. Valid values: see PriceMatch enum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_match: Option<PriceMatch>,

    /// Receiving window (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp (milliseconds since epoch). Required.
    pub timestamp: u64,
}

/// Response for a modified order.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyOrderResponse {
    /// Order ID.
    pub order_id: u64,

    /// Symbol.
    pub symbol: Cow<'static, str>,

    /// Trading pair.
    pub pair: Cow<'static, str>,

    /// Order status.
    pub status: OrderStatus,

    /// Client order ID.
    pub client_order_id: Cow<'static, str>,

    /// Price.
    pub price: Cow<'static, str>,

    /// Average price.
    pub avg_price: Cow<'static, str>,

    /// Original quantity.
    pub orig_qty: Cow<'static, str>,

    /// Executed quantity.
    pub executed_qty: Cow<'static, str>,

    /// Cumulative quantity.
    pub cum_qty: Cow<'static, str>,

    /// Cumulative base asset quantity.
    pub cum_base: Cow<'static, str>,

    /// Time in force.
    pub time_in_force: TimeInForce,

    /// Order type.
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Reduce only flag.
    pub reduce_only: bool,

    /// Close position flag.
    pub close_position: bool,

    /// Order side.
    pub side: OrderSide,

    /// Position side.
    pub position_side: PositionSide,

    /// Stop price.
    pub stop_price: Cow<'static, str>,

    /// Working type.
    pub working_type: WorkingType,

    /// Price protect flag.
    pub price_protect: bool,

    /// Original order type.
    pub orig_type: OrderType,

    /// Price match mode.
    pub price_match: PriceMatch,

    /// Self trade prevention mode.
    pub self_trade_prevention_mode: SelfTradePreventionMode,

    /// Good till date (timestamp).
    pub good_till_date: u64,

    /// Update time (milliseconds since epoch).
    pub update_time: u64,
}

impl UsdmClient {
    /// Modify Order
    ///
    /// Modifies an existing order on Binance USDM Futures.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Modify-Order)
    ///
    /// Rate limit: 1 on 10s order rate limit (X-MBX-ORDER-COUNT-10S); 1 on 1min order rate limit (X-MBX-ORDER-COUNT-1M); 1 on IP rate limit (x-mbx-used-weight-1m)
    ///
    /// # Arguments
    /// * `request` - The modify order request parameters
    ///
    /// # Returns
    /// Response containing modified order details
    pub async fn modify_order(
        &self,
        request: ModifyOrderRequest,
    ) -> RestResult<ModifyOrderResponse> {
        self.send_put_signed_request(MODIFY_ORDER_ENDPOINT, request, 1, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modify_order_request_fields() {
        let req = ModifyOrderRequest {
            order_id: Some(12345),
            orig_client_order_id: None,
            symbol: Cow::Borrowed("BTCUSDT"),
            side: OrderSide::Buy,
            quantity: Cow::Borrowed("1"),
            price: Cow::Borrowed("30000"),
            price_match: Some(PriceMatch::Opponent),
            recv_window: Some(5000),
            timestamp: 1629182711600,
        };
        assert_eq!(req.symbol, "BTCUSDT");
        assert_eq!(req.side, OrderSide::Buy);
        assert_eq!(req.price, "30000");
        assert_eq!(req.price_match, Some(PriceMatch::Opponent));
        assert_eq!(req.recv_window, Some(5000));
        assert_eq!(req.timestamp, 1629182711600);
    }

    #[test]
    fn test_modify_order_response_deserialize() {
        let json = r#"{
            "orderId": 20072994037,
            "symbol": "BTCUSDT",
            "pair": "BTCUSDT",
            "status": "NEW",
            "clientOrderId": "LJ9R4QZDihCaS8UAOOLpgW",
            "price": "30005",
            "avgPrice": "0.0",
            "origQty": "1",
            "executedQty": "0",
            "cumQty": "0",
            "cumBase": "0",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "reduceOnly": false,
            "closePosition": false,
            "side": "BUY",
            "positionSide": "LONG",
            "stopPrice": "0",
            "workingType": "CONTRACT_PRICE",
            "priceProtect": false,
            "origType": "LIMIT",
            "priceMatch": "NONE",
            "selfTradePreventionMode": "NONE",
            "goodTillDate": 0,
            "updateTime": 1629182711600
        }"#;
        let resp: ModifyOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.order_id, 20072994037);
        assert_eq!(resp.symbol, "BTCUSDT");
        assert_eq!(resp.status, OrderStatus::New);
        assert_eq!(resp.price, "30005");
        assert_eq!(resp.price_match, PriceMatch::None);
        assert_eq!(
            resp.self_trade_prevention_mode,
            SelfTradePreventionMode::None
        );
    }
}
