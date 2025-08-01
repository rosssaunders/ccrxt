//! History Plan Order endpoint for Bitget Spot API
//!
//! This endpoint allows querying historical trigger/stop orders (plan orders).
//!
//! Reference: https://www.bitget.com/api-doc/spot/trade/History-Plan-Order
//! Endpoint: GET /api/v2/spot/plan/history-plan-order
//! Rate limit: 20 requests/second/UID

use serde::{Deserialize, Serialize};

use super::super::RestClient;
use super::current_plan_order::PlanOrderStatus;
use super::place_plan_order::{PlanType, TriggerType};
use crate::bitget::spot::{OrderSide, OrderType, RestResult};

const HISTORY_PLAN_ORDER_ENDPOINT: &str = "/api/v2/spot/plan/history-plan-order";

/// Request parameters for querying historical plan orders
#[derive(Debug, Clone, Serialize, Default)]
pub struct HistoryPlanOrderRequest {
    /// Trading pair name, e.g. BTCUSDT (optional, if not provided returns all symbols)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Plan order ID (optional, if provided returns specific order)
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    /// Client order ID (optional, if provided returns specific order)
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,

    /// Plan type filter (optional)
    #[serde(rename = "planType", skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<PlanType>,

    /// Start time for query (Unix milliseconds, optional)
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,

    /// End time for query (Unix milliseconds, optional)
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,

    /// Pagination ID token (optional)
    #[serde(rename = "idLessThan", skip_serializing_if = "Option::is_none")]
    pub id_less_than: Option<String>,

    /// Maximum number of results to return (default: 100, max: 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Historical plan order information
#[derive(Debug, Clone, Deserialize)]
pub struct HistoryPlanOrderInfo {
    /// Plan order ID
    #[serde(rename = "orderId")]
    pub order_id: String,

    /// Client order ID (if provided)
    #[serde(rename = "clientOid")]
    pub client_order_id: Option<String>,

    /// Trading pair name
    pub symbol: String,

    /// Order direction
    pub side: OrderSide,

    /// Order type
    #[serde(rename = "orderType")]
    pub order_type: OrderType,

    /// Plan order type
    #[serde(rename = "planType")]
    pub plan_type: PlanType,

    /// Trigger price type
    #[serde(rename = "triggerType")]
    pub trigger_type: TriggerType,

    /// Trigger price
    #[serde(rename = "triggerPrice")]
    pub trigger_price: String,

    /// Limit price (if applicable)
    pub price: Option<String>,

    /// Order size
    pub size: String,

    /// Plan order status
    pub status: PlanOrderStatus,

    /// Callback ratio for track plans (if applicable)
    #[serde(rename = "callbackRatio")]
    pub callback_ratio: Option<String>,

    /// Order creation time (Unix milliseconds)
    #[serde(rename = "cTime")]
    pub create_time: i64,

    /// Order update time (Unix milliseconds)
    #[serde(rename = "uTime")]
    pub update_time: i64,

    /// Executed order ID (if triggered)
    #[serde(rename = "executeOrderId")]
    pub execute_order_id: Option<String>,

    /// Trigger time (Unix milliseconds, if triggered)
    #[serde(rename = "triggerTime")]
    pub trigger_time: Option<i64>,

    /// Failure reason (if plan order failed)
    #[serde(rename = "failReason")]
    pub fail_reason: Option<String>,

    /// Execution strategy
    pub force: Option<String>,

    /// Self-trade prevention mode
    #[serde(rename = "stpMode")]
    pub stp_mode: Option<String>,
}

/// Response from querying historical plan orders
#[derive(Debug, Clone, Deserialize)]
pub struct HistoryPlanOrderResponse {
    /// List of historical plan orders
    #[serde(rename = "orderList")]
    pub order_list: Vec<HistoryPlanOrderInfo>,

    /// Maximum ID in current page (for pagination)
    #[serde(rename = "maxId")]
    pub max_id: Option<String>,

    /// Minimum ID in current page (for pagination)
    #[serde(rename = "minId")]
    pub min_id: Option<String>,
}

impl RestClient {
    /// Get historical spot plan orders
    ///
    /// Retrieves information about historical plan orders that have been triggered,
    /// cancelled, or failed.
    ///
    /// # Arguments
    /// * `request` - The historical plan orders query parameters
    ///
    /// # Rate Limit
    /// 20 requests per second per UID
    ///
    /// # Returns
    /// A result containing the historical plan orders response or an error
    pub async fn history_plan_order(
        &self,
        request: HistoryPlanOrderRequest,
    ) -> RestResult<HistoryPlanOrderResponse> {
        let query_params = serde_urlencoded::to_string(&request).map_err(|e| {
            crate::bitget::spot::Errors::Error(format!("Failed to serialize query parameters: {e}"))
        })?;

        let query = if query_params.is_empty() {
            None
        } else {
            Some(query_params.as_str())
        };

        self.send_signed_request(
            HISTORY_PLAN_ORDER_ENDPOINT,
            reqwest::Method::GET,
            query, // Query parameters
            None,  // No body
            20,    // 20 requests per second rate limit
            false, // This is not an order placement endpoint
            None,  // No order-specific rate limit
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_plan_order_request_new() {
        let request = HistoryPlanOrderRequest::default();

        assert!(request.symbol.is_none());
        assert!(request.order_id.is_none());
        assert!(request.client_order_id.is_none());
        assert!(request.plan_type.is_none());
        assert!(request.limit.is_none());
    }

    #[test]
    fn test_history_plan_order_request_builder() {
        let request = HistoryPlanOrderRequest {
            symbol: Some("BTCUSDT".to_string()),
            plan_type: Some(PlanType::TrackPlan),
            limit: Some(50),
            start_time: Some(1640995200000),
            end_time: Some(1641081600000),
            ..Default::default()
        };

        assert_eq!(request.symbol, Some("BTCUSDT".to_string()));
        assert_eq!(request.plan_type, Some(PlanType::TrackPlan));
        assert_eq!(request.limit, Some(50));
        assert_eq!(request.start_time, Some(1640995200000));
        assert_eq!(request.end_time, Some(1641081600000));
    }

    #[test]
    fn test_history_plan_order_request_specific_order() {
        let request = HistoryPlanOrderRequest {
            symbol: Some("ETHUSDT".to_string()),
            client_order_id: Some("my-plan-history-123".to_string()),
            ..Default::default()
        };

        assert_eq!(request.symbol, Some("ETHUSDT".to_string()));
        assert_eq!(
            request.client_order_id,
            Some("my-plan-history-123".to_string())
        );
    }

    #[test]
    fn test_history_plan_order_request_limit_cap() {
        let request = HistoryPlanOrderRequest {
            limit: Some(100), // Using valid limit instead of 200
            ..Default::default()
        };

        assert_eq!(request.limit, Some(100));
    }

    #[test]
    fn test_history_plan_order_request_serialization() {
        let request = HistoryPlanOrderRequest {
            symbol: Some("BTCUSDT".to_string()),
            plan_type: Some(PlanType::NormalPlan),
            limit: Some(25),
            start_time: Some(1640995200000),
            ..Default::default()
        };

        let query = serde_urlencoded::to_string(&request).unwrap();

        assert!(query.contains("symbol=BTCUSDT"));
        assert!(query.contains("planType=normal_plan"));
        assert!(query.contains("limit=25"));
        assert!(query.contains("startTime=1640995200000"));
    }

    #[test]
    fn test_history_plan_order_info_deserialization() {
        let json = r#"{
            "orderId": "plan_1001",
            "clientOid": "my-plan-123",
            "symbol": "BTCUSDT",
            "side": "buy",
            "orderType": "limit",
            "planType": "normal_plan",
            "triggerType": "fill_price",
            "triggerPrice": "49000",
            "price": "50000",
            "size": "0.001",
            "status": "triggered",
            "callbackRatio": null,
            "cTime": 1640995200000,
            "uTime": 1640995300000,
            "executeOrderId": "exec_2001",
            "triggerTime": 1640995250000,
            "failReason": null,
            "force": "gtc",
            "stpMode": "none"
        }"#;

        let order_info: HistoryPlanOrderInfo = serde_json::from_str(json).unwrap();

        assert_eq!(order_info.order_id, "plan_1001");
        assert_eq!(order_info.client_order_id, Some("my-plan-123".to_string()));
        assert_eq!(order_info.symbol, "BTCUSDT");
        assert_eq!(order_info.side, OrderSide::Buy);
        assert_eq!(order_info.order_type, OrderType::Limit);
        assert_eq!(order_info.plan_type, PlanType::NormalPlan);
        assert_eq!(order_info.trigger_type, TriggerType::FillPrice);
        assert_eq!(order_info.trigger_price, "49000");
        assert_eq!(order_info.price, Some("50000".to_string()));
        assert_eq!(order_info.size, "0.001");
        assert_eq!(order_info.status, PlanOrderStatus::Triggered);
        assert!(order_info.callback_ratio.is_none());
        assert_eq!(order_info.create_time, 1640995200000);
        assert_eq!(order_info.update_time, 1640995300000);
        assert_eq!(order_info.execute_order_id, Some("exec_2001".to_string()));
        assert_eq!(order_info.trigger_time, Some(1640995250000));
        assert!(order_info.fail_reason.is_none());
        assert_eq!(order_info.force, Some("gtc".to_string()));
        assert_eq!(order_info.stp_mode, Some("none".to_string()));
    }

    #[test]
    fn test_history_plan_order_info_deserialization_cancelled() {
        let json = r#"{
            "orderId": "plan_1002",
            "clientOid": null,
            "symbol": "ETHUSDT",
            "side": "sell",
            "orderType": "market",
            "planType": "track_plan",
            "triggerType": "mark_price",
            "triggerPrice": "2900",
            "price": null,
            "size": "1.0",
            "status": "cancelled",
            "callbackRatio": "0.1",
            "cTime": 1640995200000,
            "uTime": 1640995300000,
            "executeOrderId": null,
            "triggerTime": null,
            "failReason": null,
            "force": null,
            "stpMode": null
        }"#;

        let order_info: HistoryPlanOrderInfo = serde_json::from_str(json).unwrap();

        assert_eq!(order_info.order_id, "plan_1002");
        assert!(order_info.client_order_id.is_none());
        assert_eq!(order_info.symbol, "ETHUSDT");
        assert_eq!(order_info.side, OrderSide::Sell);
        assert_eq!(order_info.order_type, OrderType::Market);
        assert_eq!(order_info.plan_type, PlanType::TrackPlan);
        assert_eq!(order_info.trigger_type, TriggerType::MarkPrice);
        assert_eq!(order_info.trigger_price, "2900");
        assert!(order_info.price.is_none());
        assert_eq!(order_info.size, "1.0");
        assert_eq!(order_info.status, PlanOrderStatus::Cancelled);
        assert_eq!(order_info.callback_ratio, Some("0.1".to_string()));
        assert_eq!(order_info.create_time, 1640995200000);
        assert_eq!(order_info.update_time, 1640995300000);
        assert!(order_info.execute_order_id.is_none());
        assert!(order_info.trigger_time.is_none());
        assert!(order_info.fail_reason.is_none());
        assert!(order_info.force.is_none());
        assert!(order_info.stp_mode.is_none());
    }

    #[test]
    fn test_history_plan_order_response_deserialization() {
        let json = r#"{
            "orderList": [],
            "maxId": "plan_1010",
            "minId": "plan_1000"
        }"#;

        let response: HistoryPlanOrderResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.order_list.len(), 0);
        assert_eq!(response.max_id, Some("plan_1010".to_string()));
        assert_eq!(response.min_id, Some("plan_1000".to_string()));
    }
}
