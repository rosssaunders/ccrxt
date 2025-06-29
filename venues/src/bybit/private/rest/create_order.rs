use serde::{Deserialize, Serialize};

use crate::bybit::{enums::*, EndpointType, RestResult};

use super::client::RestClient;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderRequest {
    pub category: Category,
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_leverage: Option<i32>,
    pub side: Side,
    pub order_type: OrderType,
    pub qty: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_unit: Option<MarketUnit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slippage_tolerance_type: Option<SlippageToleranceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slippage_tolerance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_direction: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_filter: Option<OrderFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_by: Option<TriggerBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_iv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_idx: Option<PositionIdx>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_by: Option<TriggerBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_by: Option<TriggerBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_on_trigger: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smp_type: Option<SmpType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mmp: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpsl_mode: Option<TpSlMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_limit_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_limit_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_order_type: Option<OrderType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_order_type: Option<OrderType>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderData {
    pub order_id: String,
    pub order_link_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateOrderResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: CreateOrderData,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

impl RestClient {
    /// Create a new order
    ///
    /// Create orders for Spot, Margin trading, USDT perpetual, USDT futures, USDC perpetual,
    /// USDC futures, Inverse Futures and Options.
    ///
    /// # Arguments
    /// * `request` - The order creation request parameters
    ///
    /// # Returns
    /// A result containing the order creation response or an error
    pub async fn create_order(
        &self,
        request: CreateOrderRequest,
    ) -> RestResult<CreateOrderResponse> {
        self.send_signed_request(
            "/v5/order/create",
            reqwest::Method::POST,
            request,
            EndpointType::Trade,
        )
        .await
    }
}

impl Default for CreateOrderRequest {
    fn default() -> Self {
        Self {
            category: Category::Linear,
            symbol: String::new(),
            is_leverage: None,
            side: Side::Buy,
            order_type: OrderType::Market,
            qty: String::new(),
            market_unit: None,
            slippage_tolerance_type: None,
            slippage_tolerance: None,
            price: None,
            trigger_direction: None,
            order_filter: None,
            trigger_price: None,
            trigger_by: None,
            order_iv: None,
            time_in_force: None,
            position_idx: None,
            order_link_id: None,
            take_profit: None,
            stop_loss: None,
            tp_trigger_by: None,
            sl_trigger_by: None,
            reduce_only: None,
            close_on_trigger: None,
            smp_type: None,
            mmp: None,
            tpsl_mode: None,
            tp_limit_price: None,
            sl_limit_price: None,
            tp_order_type: None,
            sl_order_type: None,
        }
    }
}

impl CreateOrderRequest {
    /// Create a new order request builder
    pub fn new(category: Category, symbol: String, side: Side, order_type: OrderType, qty: String) -> Self {
        Self {
            category,
            symbol,
            side,
            order_type,
            qty,
            ..Default::default()
        }
    }

    /// Set the price for limit orders
    pub fn price(mut self, price: String) -> Self {
        self.price = Some(price);
        self
    }

    /// Set time in force
    pub fn time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.time_in_force = Some(time_in_force);
        self
    }

    /// Set order link ID for custom order identification
    pub fn order_link_id(mut self, order_link_id: String) -> Self {
        self.order_link_id = Some(order_link_id);
        self
    }

    /// Set take profit price
    pub fn take_profit(mut self, take_profit: String) -> Self {
        self.take_profit = Some(take_profit);
        self
    }

    /// Set stop loss price
    pub fn stop_loss(mut self, stop_loss: String) -> Self {
        self.stop_loss = Some(stop_loss);
        self
    }

    /// Set reduce only flag
    pub fn reduce_only(mut self, reduce_only: bool) -> Self {
        self.reduce_only = Some(reduce_only);
        self
    }

    /// Set position index for hedge mode
    pub fn position_idx(mut self, position_idx: PositionIdx) -> Self {
        self.position_idx = Some(position_idx);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_order_request_serialization() {
        let request = CreateOrderRequest::new(
            Category::Linear,
            "BTCUSDT".to_string(),
            Side::Buy,
            OrderType::Limit,
            "0.001".to_string(),
        )
        .price("50000".to_string())
        .time_in_force(TimeInForce::GTC);

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"category\":\"linear\""));
        assert!(json.contains("\"symbol\":\"BTCUSDT\""));
        assert!(json.contains("\"side\":\"Buy\""));
        assert!(json.contains("\"orderType\":\"Limit\""));
        assert!(json.contains("\"qty\":\"0.001\""));
        assert!(json.contains("\"price\":\"50000\""));
        assert!(json.contains("\"timeInForce\":\"GTC\""));
    }

    #[test]
    fn test_create_order_request_builder() {
        let request = CreateOrderRequest::new(
            Category::Spot,
            "ETHUSDT".to_string(),
            Side::Sell,
            OrderType::Market,
            "1.0".to_string(),
        )
        .order_link_id("custom-123".to_string())
        .reduce_only(true);

        assert_eq!(request.category, Category::Spot);
        assert_eq!(request.symbol, "ETHUSDT");
        assert_eq!(request.side, Side::Sell);
        assert_eq!(request.order_type, OrderType::Market);
        assert_eq!(request.qty, "1.0");
        assert_eq!(request.order_link_id, Some("custom-123".to_string()));
        assert_eq!(request.reduce_only, Some(true));
    }
}