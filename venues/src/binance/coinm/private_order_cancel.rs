use serde::{Deserialize, Serialize};
use super::errors::BinanceCoinMResult;
use super::private_rest::BinanceCoinMPrivateRest;
use super::types::BinanceResponse;
use super::common::request::send_request;
use crate::binance::coinm::enums::{OrderSide, PositionSide, OrderStatus, TimeInForce, OrderType, WorkingType};

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderResponse {
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
    #[serde(rename = "cumQty")]
    pub cum_qty: String,
    #[serde(rename = "cumQuote")]
    pub cum_quote: String,
    #[serde(rename = "executedQty")]
    pub executed_qty: String,
    #[serde(rename = "orderId")]
    pub order_id: i64,
    #[serde(rename = "avgPrice")]
    pub avg_price: String,
    #[serde(rename = "origQty")]
    pub orig_qty: String,
    pub price: String,
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,
    pub side: OrderSide,
    #[serde(rename = "positionSide")]
    pub position_side: PositionSide,
    pub status: OrderStatus,
    #[serde(rename = "stopPrice")]
    pub stop_price: String,
    #[serde(rename = "closePosition")]
    pub close_position: bool,
    pub symbol: String,
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    #[serde(rename = "origType")]
    pub orig_type: OrderType,
    #[serde(rename = "activatePrice")]
    pub activate_price: String,
    #[serde(rename = "priceRate")]
    pub price_rate: String,
    #[serde(rename = "updateTime")]
    pub update_time: i64,
    #[serde(rename = "workingType")]
    pub working_type: WorkingType,
    #[serde(rename = "priceProtect")]
    pub price_protect: bool,
}

impl BinanceCoinMPrivateRest {
    pub async fn cancel_order(
        &self,
        symbol: &str,
        order_id: Option<i64>,
        client_order_id: Option<&str>,
    ) -> BinanceCoinMResult<BinanceResponse<OrderResponse>> {
        let timestamp = chrono::Utc::now().timestamp_millis();
        let mut query_string = format!("symbol={}&timestamp={}", symbol, timestamp);
        
        if let Some(id) = order_id {
            query_string.push_str(&format!("&orderId={}", id));
        }
        
        if let Some(cid) = client_order_id {
            query_string.push_str(&format!("&clientOrderId={}", cid));
        }
        
        let signature = self.sign_request(&query_string);
        let endpoint = format!("/dapi/v1/order?{}&signature={}", query_string, signature);
        
        send_request(
            &self.client,
            &self.base_url,
            &endpoint,
            reqwest::Method::DELETE,
            None,
            Some(&self.api_key),
            || self.rate_limiter.check_weight_limit("order", 1)
        ).await
    }
} 