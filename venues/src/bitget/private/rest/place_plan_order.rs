//! Place Plan Order endpoint for Bitget Spot API
//!
//! This endpoint allows placing trigger/stop orders (plan orders) for spot trading.
//!
//! Reference: https://www.bitget.com/api-doc/spot/plan/Place-Plan-Order
//! Endpoint: POST /api/v2/spot/plan/place-plan-order
//! Rate limit: 10 requests/second/UID

use serde::{Deserialize, Serialize};

use super::super::RestClient;
use super::place_order::{Force, STPMode};
use crate::bitget::{OrderSide, OrderType, RestResult};

/// Plan order type (trigger condition)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PlanType {
    /// Normal profit/loss plan
    #[serde(rename = "normal_plan")]
    NormalPlan,
    /// Track profit/loss plan (trailing stop)
    #[serde(rename = "track_plan")]
    TrackPlan,
}

/// Plan order trigger price type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TriggerType {
    /// Fill price
    #[serde(rename = "fill_price")]
    FillPrice,
    /// Mark price
    #[serde(rename = "mark_price")]
    MarkPrice,
}

/// Request parameters for placing a plan order
#[derive(Debug, Clone, Serialize)]
pub struct PlacePlanOrderRequest {
    /// Trading pair name, e.g. BTCUSDT
    pub symbol: String,

    /// Order direction: buy or sell
    pub side: OrderSide,

    /// Order type: limit or market
    #[serde(rename = "orderType")]
    pub order_type: OrderType,

    /// Execution strategy (invalid when orderType is market)
    pub force: Force,

    /// Plan order type
    #[serde(rename = "planType")]
    pub plan_type: PlanType,

    /// Trigger price type
    #[serde(rename = "triggerType")]
    pub trigger_type: TriggerType,

    /// Trigger price - when market price reaches this price, the order will be triggered
    #[serde(rename = "triggerPrice")]
    pub trigger_price: String,

    /// Limit price (required for limit orders)
    /// The decimal places of price can be obtained from Get Symbol Info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Order amount
    /// For Limit and Market-Sell orders: represents the number of base coins
    /// For Market-Buy orders: represents the number of quote coins
    pub size: String,

    /// Custom order ID (optional)
    #[serde(rename = "clientOid", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,

    /// Self-trade prevention mode
    #[serde(rename = "stpMode", skip_serializing_if = "Option::is_none")]
    pub stp_mode: Option<STPMode>,

    /// Callback rate for track plan orders (percentage, e.g., "0.1" for 0.1%)
    /// Only valid when planType is track_plan
    #[serde(rename = "callbackRatio", skip_serializing_if = "Option::is_none")]
    pub callback_ratio: Option<String>,

    /// Request timestamp (Unix milliseconds)
    #[serde(rename = "requestTime", skip_serializing_if = "Option::is_none")]
    pub request_time: Option<i64>,

    /// Valid time window (Unix milliseconds)
    /// If set, request is valid only when server time is within receiveWindow
    #[serde(rename = "receiveWindow", skip_serializing_if = "Option::is_none")]
    pub receive_window: Option<i64>,
}

/// Response from placing a plan order
#[derive(Debug, Clone, Deserialize)]
pub struct PlacePlanOrderResponse {
    /// Plan order ID assigned by the system
    #[serde(rename = "orderId")]
    pub order_id: String,

    /// Custom order ID (if provided in request)
    #[serde(rename = "clientOid")]
    pub client_order_id: Option<String>,
}

impl RestClient {
    /// Place a spot plan order (trigger/stop order)
    ///
    /// Places a new plan order for spot trading that will be triggered when the specified
    /// trigger conditions are met.
    ///
    /// # Arguments
    /// * `request` - The plan order placement request parameters
    ///
    /// # Rate Limit
    /// 10 requests per second per UID
    ///
    /// # Returns
    /// A result containing the plan order placement response or an error
    pub async fn place_plan_order(
        &self,
        request: PlacePlanOrderRequest,
    ) -> RestResult<PlacePlanOrderResponse> {
        let body = serde_json::to_string(&request).map_err(|e| {
            crate::bitget::Errors::Error(format!("Failed to serialize request: {e}"))
        })?;

        self.send_signed_request(
            "/api/v2/spot/plan/place-plan-order",
            reqwest::Method::POST,
            None,        // No query parameters
            Some(&body), // JSON body
            10,          // 10 requests per second rate limit
            true,        // This is an order endpoint
            Some(10),    // Order-specific rate limit
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_plan_order_request_normal_limit() {
        let request = PlacePlanOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            force: Force::GTC,
            plan_type: PlanType::NormalPlan,
            trigger_type: TriggerType::FillPrice,
            trigger_price: "49000".to_string(),
            price: Some("50000".to_string()),
            size: "0.001".to_string(),
            client_order_id: None,
            stp_mode: None,
            callback_ratio: None,
            request_time: None,
            receive_window: None,
        };

        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.side, OrderSide::Buy);
        assert_eq!(request.order_type, OrderType::Limit);
        assert_eq!(request.plan_type, PlanType::NormalPlan);
        assert_eq!(request.trigger_type, TriggerType::FillPrice);
        assert_eq!(request.trigger_price, "49000");
        assert_eq!(request.price, Some("50000".to_string()));
        assert_eq!(request.size, "0.001");
        assert!(request.callback_ratio.is_none());
    }

    #[test]
    fn test_place_plan_order_request_normal_market() {
        let request = PlacePlanOrderRequest {
            symbol: "ETHUSDT".to_string(),
            side: OrderSide::Sell,
            order_type: OrderType::Market,
            force: Force::GTC,
            plan_type: PlanType::NormalPlan,
            trigger_type: TriggerType::MarkPrice,
            trigger_price: "2900".to_string(),
            price: None,
            size: "1.0".to_string(),
            client_order_id: None,
            stp_mode: None,
            callback_ratio: None,
            request_time: None,
            receive_window: None,
        };

        assert_eq!(request.symbol, "ETHUSDT");
        assert_eq!(request.side, OrderSide::Sell);
        assert_eq!(request.order_type, OrderType::Market);
        assert_eq!(request.plan_type, PlanType::NormalPlan);
        assert_eq!(request.trigger_type, TriggerType::MarkPrice);
        assert_eq!(request.trigger_price, "2900");
        assert!(request.price.is_none());
        assert_eq!(request.size, "1.0");
    }

    #[test]
    fn test_place_plan_order_request_track_market() {
        let request = PlacePlanOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Sell,
            order_type: OrderType::Market,
            force: Force::GTC,
            plan_type: PlanType::TrackPlan,
            trigger_type: TriggerType::FillPrice,
            trigger_price: "51000".to_string(),
            price: None,
            size: "0.5".to_string(),
            client_order_id: None,
            stp_mode: None,
            callback_ratio: Some("0.1".to_string()),
            request_time: None,
            receive_window: None,
        };

        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.side, OrderSide::Sell);
        assert_eq!(request.order_type, OrderType::Market);
        assert_eq!(request.plan_type, PlanType::TrackPlan);
        assert_eq!(request.trigger_type, TriggerType::FillPrice);
        assert_eq!(request.trigger_price, "51000");
        assert!(request.price.is_none());
        assert_eq!(request.size, "0.5");
        assert_eq!(request.callback_ratio, Some("0.1".to_string()));
    }

    #[test]
    fn test_place_plan_order_request_builder() {
        let request = PlacePlanOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            force: Force::PostOnly,
            plan_type: PlanType::NormalPlan,
            trigger_type: TriggerType::FillPrice,
            trigger_price: "49000".to_string(),
            price: Some("50000".to_string()),
            size: "0.001".to_string(),
            client_order_id: Some("plan-order-123".to_string()),
            stp_mode: Some(STPMode::CancelTaker),
            callback_ratio: None,
            request_time: None,
            receive_window: None,
        };

        assert_eq!(request.force, Force::PostOnly);
        assert_eq!(request.client_order_id, Some("plan-order-123".to_string()));
        assert_eq!(request.stp_mode, Some(STPMode::CancelTaker));
    }

    #[test]
    fn test_place_plan_order_request_serialization() {
        let request = PlacePlanOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            force: Force::GTC,
            plan_type: PlanType::NormalPlan,
            trigger_type: TriggerType::FillPrice,
            trigger_price: "49000".to_string(),
            price: Some("50000".to_string()),
            size: "0.001".to_string(),
            client_order_id: Some("plan-123".to_string()),
            stp_mode: None,
            callback_ratio: None,
            request_time: None,
            receive_window: None,
        };

        let json = serde_json::to_string(&request).unwrap();

        assert!(json.contains("\"symbol\":\"BTCUSDT\""));
        assert!(json.contains("\"side\":\"buy\""));
        assert!(json.contains("\"orderType\":\"limit\""));
        assert!(json.contains("\"planType\":\"normal_plan\""));
        assert!(json.contains("\"triggerType\":\"fill_price\""));
        assert!(json.contains("\"triggerPrice\":\"49000\""));
        assert!(json.contains("\"price\":\"50000\""));
        assert!(json.contains("\"size\":\"0.001\""));
        assert!(json.contains("\"clientOid\":\"plan-123\""));
    }

    #[test]
    fn test_place_plan_order_response_deserialization() {
        let json = r#"{
            "orderId": "plan_1001",
            "clientOid": "plan-order-123"
        }"#;

        let response: PlacePlanOrderResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.order_id, "plan_1001");
        assert_eq!(response.client_order_id, Some("plan-order-123".to_string()));
    }

    #[test]
    fn test_place_plan_order_response_deserialization_no_client_id() {
        let json = r#"{
            "orderId": "plan_1001",
            "clientOid": null
        }"#;

        let response: PlacePlanOrderResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.order_id, "plan_1001");
        assert!(response.client_order_id.is_none());
    }

    #[test]
    fn test_plan_type_serialization() {
        assert_eq!(
            serde_json::to_string(&PlanType::NormalPlan).unwrap(),
            "\"normal_plan\""
        );
        assert_eq!(
            serde_json::to_string(&PlanType::TrackPlan).unwrap(),
            "\"track_plan\""
        );
    }

    #[test]
    fn test_trigger_type_serialization() {
        assert_eq!(
            serde_json::to_string(&TriggerType::FillPrice).unwrap(),
            "\"fill_price\""
        );
        assert_eq!(
            serde_json::to_string(&TriggerType::MarkPrice).unwrap(),
            "\"mark_price\""
        );
    }
}
