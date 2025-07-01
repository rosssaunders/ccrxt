//! Get Current Orders endpoint for Bitget Spot API
//!
//! This endpoint allows retrieving current unfilled orders.
//!
//! Reference: https://www.bitget.com/api-doc/spot/trade/Get-Unfilled-Orders
//! Endpoint: GET /api/v2/spot/trade/unfilled-orders
//! Rate limit: 20 times/1s (UID)

use serde::{Deserialize, Serialize};

use super::super::RestClient;
use crate::bitget::{OrderSide, OrderType, RestResult};
use super::get_order_info::{OrderStatus, OrderSource, EntryPointSource};

/// TPSL (Take Profit/Stop Loss) order type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TPSLType {
    /// Normal spot order
    #[serde(rename = "normal")]
    Normal,
    /// TPSL spot order
    #[serde(rename = "tpsl")]
    #[allow(clippy::upper_case_acronyms)]
    TPSL,
}

/// Request parameters for getting current unfilled orders
#[derive(Debug, Clone, Serialize)]
pub struct GetCurrentOrdersRequest {
    /// Trading pair (optional filter)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// The record start time for the query (Unix milliseconds)
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,

    /// The end time of the record for the query (Unix milliseconds)
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,

    /// Requests content on the page before this ID (older data)
    #[serde(rename = "idLessThan", skip_serializing_if = "Option::is_none")]
    pub id_less_than: Option<String>,

    /// Limit number (default 100, max 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Order ID filter
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    /// Order type (default normal)
    #[serde(rename = "tpslType", skip_serializing_if = "Option::is_none")]
    pub tpsl_type: Option<TPSLType>,

    /// Request time (Unix milliseconds)
    #[serde(rename = "requestTime", skip_serializing_if = "Option::is_none")]
    pub request_time: Option<i64>,

    /// Valid window period (Unix milliseconds)
    #[serde(rename = "receiveWindow", skip_serializing_if = "Option::is_none")]
    pub receive_window: Option<i64>,
}

impl GetCurrentOrdersRequest {
    /// Create a new request with default parameters
    pub fn new() -> Self {
        Self {
            symbol: None,
            start_time: None,
            end_time: None,
            id_less_than: None,
            limit: None,
            order_id: None,
            tpsl_type: None,
            request_time: None,
            receive_window: None,
        }
    }

    /// Filter by symbol
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }

    /// Set time range
    pub fn time_range(mut self, start_time: i64, end_time: i64) -> Self {
        self.start_time = Some(start_time);
        self.end_time = Some(end_time);
        self
    }

    /// Set pagination
    pub fn pagination(mut self, id_less_than: Option<String>, limit: u32) -> Self {
        self.id_less_than = id_less_than;
        self.limit = Some(limit.min(100)); // Enforce max limit
        self
    }

    /// Filter by order ID
    pub fn order_id(mut self, order_id: impl Into<String>) -> Self {
        self.order_id = Some(order_id.into());
        self
    }

    /// Filter by order type
    pub fn tpsl_type(mut self, tpsl_type: TPSLType) -> Self {
        self.tpsl_type = Some(tpsl_type);
        self
    }
}

impl Default for GetCurrentOrdersRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Information about a current order
#[derive(Debug, Clone, Deserialize)]
pub struct OrderInfo {
    /// User ID
    #[serde(rename = "userId")]
    pub user_id: String,

    /// Trading pair name
    pub symbol: String,

    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: String,

    /// Client order ID
    #[serde(rename = "clientOid")]
    pub client_order_id: Option<String>,

    /// Order price
    #[serde(rename = "priceAvg")]
    pub price_avg: String,

    /// Order amount
    pub size: String,

    /// Order type
    #[serde(rename = "orderType")]
    pub order_type: OrderType,

    /// Order direction
    pub side: OrderSide,

    /// Order status
    pub status: OrderStatus,

    /// Filled price
    #[serde(rename = "basePrice")]
    pub base_price: String,

    /// Filled volume in base coin
    #[serde(rename = "baseVolume")]
    pub base_volume: String,

    /// Filled volume in quote coin
    #[serde(rename = "quoteVolume")]
    pub quote_volume: String,

    /// Entry point source
    #[serde(rename = "enterPointSource")]
    pub entry_point_source: EntryPointSource,

    /// Order source
    #[serde(rename = "orderSource")]
    pub order_source: OrderSource,

    /// Take profit trigger price
    #[serde(rename = "presetTakeProfitPrice")]
    pub preset_take_profit_price: Option<String>,

    /// Take profit execute price
    #[serde(rename = "executeTakeProfitPrice")]
    pub execute_take_profit_price: Option<String>,

    /// Stop loss trigger price
    #[serde(rename = "presetStopLossPrice")]
    pub preset_stop_loss_price: Option<String>,

    /// Stop loss execute price
    #[serde(rename = "executeStopLossPrice")]
    pub execute_stop_loss_price: Option<String>,

    /// Creation time (Unix milliseconds)
    #[serde(rename = "cTime")]
    pub create_time: String,

    /// Update time (Unix milliseconds)
    #[serde(rename = "uTime")]
    pub update_time: String,

    /// TPSL order type
    #[serde(rename = "tpslType")]
    pub tpsl_type: String,

    /// Trigger price (only valid when tpslType is tpsl)
    #[serde(rename = "triggerPrice")]
    pub trigger_price: Option<String>,
}

/// Response from getting current orders
#[derive(Debug, Clone)]
pub struct GetCurrentOrdersResponse {
    /// List of current orders
    pub orders: Vec<OrderInfo>,
}

// Implementation for direct response deserialization when the API returns an array
impl<'de> Deserialize<'de> for GetCurrentOrdersResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let orders = Vec::<OrderInfo>::deserialize(deserializer)?;
        Ok(GetCurrentOrdersResponse { orders })
    }
}

impl RestClient {
    /// Get current unfilled orders
    ///
    /// Retrieves all unfilled orders for the authenticated account with optional filters.
    ///
    /// # Arguments
    /// * `request` - The request parameters with optional filters
    ///
    /// # Rate Limit
    /// 20 requests per second per UID
    ///
    /// # Returns
    /// A result containing the current orders or an error
    pub async fn get_current_orders(
        &self,
        request: GetCurrentOrdersRequest,
    ) -> RestResult<GetCurrentOrdersResponse> {
        // Only create query string if there are parameters to serialize
        let has_params = request.symbol.is_some()
            || request.start_time.is_some()
            || request.end_time.is_some()
            || request.id_less_than.is_some()
            || request.limit.is_some()
            || request.order_id.is_some()
            || request.tpsl_type.is_some()
            || request.request_time.is_some()
            || request.receive_window.is_some();

        let query_string = if has_params {
            Some(serde_urlencoded::to_string(&request).map_err(|e| {
                crate::bitget::Errors::Error(format!("Failed to encode query: {e}"))
            })?)
        } else {
            None
        };

        self.send_signed_request(
            "/api/v2/spot/trade/unfilled-orders",
            reqwest::Method::GET,
            query_string.as_deref(),
            None,  // No body for GET request
            20,    // 20 requests per second rate limit
            false, // Not an order placement endpoint
            None,  // No order-specific rate limit
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_orders_request_default() {
        let request = GetCurrentOrdersRequest::new();
        
        assert!(request.symbol.is_none());
        assert!(request.start_time.is_none());
        assert!(request.end_time.is_none());
        assert!(request.limit.is_none());
    }

    #[test]
    fn test_get_current_orders_request_builder() {
        let request = GetCurrentOrdersRequest::new()
            .symbol("BTCUSDT")
            .time_range(1659036670000, 1659076670000)
            .pagination(Some("12345".to_string()), 50)
            .tpsl_type(TPSLType::Normal);

        assert_eq!(request.symbol, Some("BTCUSDT".to_string()));
        assert_eq!(request.start_time, Some(1659036670000));
        assert_eq!(request.end_time, Some(1659076670000));
        assert_eq!(request.id_less_than, Some("12345".to_string()));
        assert_eq!(request.limit, Some(50));
        assert_eq!(request.tpsl_type, Some(TPSLType::Normal));
    }

    #[test]
    fn test_get_current_orders_request_limit_enforcement() {
        let request = GetCurrentOrdersRequest::new().pagination(None, 200);
        
        // Should be capped at 100
        assert_eq!(request.limit, Some(100));
    }

    #[test]
    fn test_order_info_deserialization() {
        let json = r#"{
            "userId": "123456789",
            "symbol": "BTCUSDT",
            "orderId": "2222222",
            "clientOid": "xxxxxxx",
            "priceAvg": "34829.12",
            "size": "1",
            "orderType": "limit",
            "side": "buy",
            "status": "live",
            "basePrice": "0",
            "baseVolume": "0",
            "quoteVolume": "0",
            "enterPointSource": "WEB",
            "orderSource": "normal",
            "presetTakeProfitPrice": "70000",
            "executeTakeProfitPrice": "",
            "presetStopLossPrice": "10000",
            "executeStopLossPrice": "",
            "cTime": "1622697148",
            "uTime": "1622697148",
            "tpslType": "normal",
            "triggerPrice": null
        }"#;

        let order: OrderInfo = serde_json::from_str(json).unwrap();

        assert_eq!(order.user_id, "123456789");
        assert_eq!(order.symbol, "BTCUSDT");
        assert_eq!(order.order_id, "2222222");
        assert_eq!(order.client_order_id, Some("xxxxxxx".to_string()));
        assert_eq!(order.order_type, OrderType::Limit);
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.status, OrderStatus::Live);
        assert_eq!(order.tpsl_type, "normal");
    }

    #[test]
    fn test_get_current_orders_response_deserialization() {
        let json = r#"[
            {
                "userId": "123456789",
                "symbol": "BTCUSDT",
                "orderId": "2222222",
                "clientOid": "xxxxxxx",
                "priceAvg": "34829.12",
                "size": "1",
                "orderType": "limit",
                "side": "buy",
                "status": "live",
                "basePrice": "0",
                "baseVolume": "0",
                "quoteVolume": "0",
                "enterPointSource": "WEB",
                "orderSource": "normal",
                "presetTakeProfitPrice": "70000",
                "executeTakeProfitPrice": "",
                "presetStopLossPrice": "10000",
                "executeStopLossPrice": "",
                "cTime": "1622697148",
                "uTime": "1622697148",
                "tpslType": "normal",
                "triggerPrice": null
            }
        ]"#;

        let response: GetCurrentOrdersResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.orders.len(), 1);
        assert_eq!(response.orders[0].symbol, "BTCUSDT");
        assert_eq!(response.orders[0].order_id, "2222222");
    }
}
