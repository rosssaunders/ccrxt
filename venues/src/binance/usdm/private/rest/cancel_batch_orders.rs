use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::UsdmClient;
use crate::binance::usdm::RestResult;
use crate::binance::usdm::enums::{
    OrderSide, OrderStatus, OrderType, PositionSide, PriceMatch, SelfTradePreventionMode,
    TimeInForce, WorkingType,
};

/// Endpoint path for cancelling multiple orders in a single batch for USDM futures.
const CANCEL_BATCH_ORDERS_ENDPOINT: &str = "/fapi/v1/batchOrders";

/// Request parameters for cancelling multiple orders in a single batch for USDM futures.
///
/// See the [Binance API documentation][docs] for details.
///
/// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Cancel-Multiple-Orders
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancelBatchOrdersRequest {
    /// Trading symbol for which to cancel orders (e.g., "BTCUSDT").
    ///
    /// Must be a valid symbol listed on Binance USDM futures.
    pub symbol: Cow<'static, str>,

    /// List of order IDs to cancel (either this or `orig_client_order_id_list` must be provided, max 10).
    ///
    /// If provided, must be a list of up to 10 order IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id_list: Option<Vec<u64>>,

    /// List of original client order IDs to cancel (either this or `order_id_list` must be provided, max 10).
    ///
    /// If provided, must be a list of up to 10 client order IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id_list: Option<Vec<Cow<'static, str>>>,

    /// The number of milliseconds the request is valid for after timestamp. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp (milliseconds since epoch).
    pub timestamp: u64,
}

/// Response for a cancelled order in a batch (success or error).
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum CancelBatchOrderResponse {
    /// Successful cancellation response.
    Success(CancelBatchOrderSuccess),
    /// Error response for a specific order in the batch.
    Error {
        /// Error code returned by the API.
        code: i64,

        /// Error message returned by the API.
        msg: String,
    },
}

/// Successful order cancellation response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelBatchOrderSuccess {
    /// Trading symbol for the cancelled order.
    pub symbol: String,

    /// Order ID of the cancelled order.
    pub order_id: u64,

    /// Client order ID of the cancelled order.
    pub client_order_id: String,

    /// Order price as a string.
    pub price: String,

    /// Original order quantity as a string.
    pub orig_qty: String,

    /// Executed quantity as a string.
    pub executed_qty: String,

    /// Cumulative quote quantity as a string.
    pub cum_quote: String,

    /// Order status.
    pub status: OrderStatus,

    /// Time in force.
    pub time_in_force: TimeInForce,

    /// Order type.
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Order side.
    pub side: OrderSide,

    /// Position side.
    pub position_side: PositionSide,

    /// Update time (timestamp in milliseconds).
    pub update_time: u64,

    /// Stop price. Only present for STOP/TAKE_PROFIT orders.
    #[serde(default)]
    pub stop_price: Option<String>,

    /// Activation price. Only present for TRAILING_STOP_MARKET orders.
    #[serde(default)]
    pub activate_price: Option<String>,

    /// Callback rate. Only present for TRAILING_STOP_MARKET orders.
    #[serde(default)]
    pub price_rate: Option<String>,

    /// Order original type.
    #[serde(default)]
    pub orig_type: Option<OrderType>,

    /// Reduce only flag.
    #[serde(default)]
    pub reduce_only: Option<bool>,

    /// Close position flag.
    #[serde(default)]
    pub close_position: Option<bool>,

    /// Working type.
    #[serde(default)]
    pub working_type: Option<WorkingType>,

    /// Price protection flag.
    #[serde(default)]
    pub price_protect: Option<bool>,

    /// Price match mode.
    #[serde(default)]
    pub price_match: Option<PriceMatch>,

    /// Self trade prevention mode.
    #[serde(default)]
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,

    /// Good till date (timestamp in ms). Only present for GTD orders.
    #[serde(default)]
    pub good_till_date: Option<u64>,
}

impl UsdmClient {
    /// Cancel Multiple Orders (TRADE)
    ///
    /// Cancels multiple orders in a single batch for USDM futures.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Cancel-Multiple-Orders
    ///
    /// Rate limit: 1
    ///
    /// # Arguments
    /// * `request` - The batch order cancellation request parameters.
    ///
    /// # Returns
    /// Vector of cancellation responses, one for each order in the batch.
    pub async fn cancel_batch_orders(
        &self,
        request: CancelBatchOrdersRequest,
    ) -> RestResult<Vec<CancelBatchOrderResponse>> {
        self.send_delete_signed_request(
            CANCEL_BATCH_ORDERS_ENDPOINT,
            request,
            1,
            true,)
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{from_value, json};
    use std::borrow::Cow;

    #[test]
    fn test_cancel_batch_orders_request_serialization() {
        let req = CancelBatchOrdersRequest {
            symbol: Cow::Borrowed("BTCUSDT"),
            order_id_list: Some(vec![123456, 234567]),
            orig_client_order_id_list: None,
            recv_window: Some(5000),
            timestamp: 1234567890,
        };
        let ser = serde_json::to_string(&req).unwrap();
        assert!(ser.contains("BTCUSDT"));
        assert!(ser.contains("orderIdList"));
        assert!(ser.contains("recvWindow"));
    }

    #[test]
    fn test_cancel_batch_order_success_deserialization() {
        let val = json!({
            "symbol": "BTCUSDT",
            "orderId": 283194212,
            "clientOrderId": "myOrder1",
            "price": "0",
            "origQty": "11",
            "executedQty": "0",
            "cumQuote": "0",
            "status": "CANCELED",
            "timeInForce": "GTC",
            "type": "TRAILING_STOP_MARKET",
            "side": "BUY",
            "positionSide": "SHORT",
            "updateTime": 1571110484038u64,
            "stopPrice": "9300",
            "activatePrice": "9020",
            "priceRate": "0.3",
            "origType": "TRAILING_STOP_MARKET",
            "reduceOnly": false,
            "closePosition": false,
            "workingType": "CONTRACT_PRICE",
            "priceProtect": false,
            "priceMatch": "NONE",
            "selfTradePreventionMode": "NONE",
            "goodTillDate": 1693207680000u64
        });
        let res: CancelBatchOrderSuccess = from_value(val).unwrap();
        assert_eq!(res.symbol, "BTCUSDT");
        assert_eq!(res.status, OrderStatus::Canceled);
        assert_eq!(res.time_in_force, TimeInForce::GTC);
        assert_eq!(res.order_type, OrderType::TrailingStopMarket);
        assert_eq!(res.side, OrderSide::Buy);
        assert_eq!(res.position_side, PositionSide::Short);
        assert_eq!(res.working_type, Some(WorkingType::ContractPrice));
        assert_eq!(res.price_match, Some(PriceMatch::None));
        assert_eq!(
            res.self_trade_prevention_mode,
            Some(SelfTradePreventionMode::None)
        );
    }

    #[test]
    fn test_cancel_batch_order_error_deserialization() {
        let val = json!({
            "code": -2011,
            "msg": "Unknown order sent."
        });
        let res: CancelBatchOrderResponse = from_value(val).unwrap();
        match res {
            CancelBatchOrderResponse::Error { code, msg } => {
                assert_eq!(code, -2011);
                assert_eq!(msg, "Unknown order sent.");
            }
            _ => panic!("Expected error variant"),
        }
    }
}
