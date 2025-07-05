//! Get Order Info endpoint for Bitget Spot API
//!
//! This endpoint allows retrieving detailed information about a specific order.
//!
//! Reference: https://www.bitget.com/api-doc/spot/trade/Get-Order-Info
//! Endpoint: GET /api/v2/spot/trade/orderInfo
//! Rate limit: 20 times/1s (UID)

use serde::{Deserialize, Serialize};

use super::super::RestClient;
use crate::bitget::{OrderSide, OrderType, RestResult};

/// Request parameters for getting order information
#[derive(Debug, Clone, Serialize)]
pub struct GetOrderInfoRequest {
    /// Order ID (either orderId or clientOrderId is required)
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    /// Client order ID (either orderId or clientOrderId is required)
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
}

impl GetOrderInfoRequest {
    /// Create a request using order ID
    pub fn by_order_id(order_id: impl Into<String>) -> Self {
        Self {
            order_id: Some(order_id.into()),
            client_order_id: None,
        }
    }

    /// Create a request using client order ID
    pub fn by_client_order_id(client_order_id: impl Into<String>) -> Self {
        Self {
            order_id: None,
            client_order_id: Some(client_order_id.into()),
        }
    }
}

/// Order status as returned by the API
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    /// Pending match
    #[serde(rename = "live")]
    Live,
    /// Partially filled
    #[serde(rename = "partially_filled")]
    PartiallyFilled,
    /// All filled
    #[serde(rename = "filled")]
    Filled,
    /// The order is cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
}

/// Order source
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderSource {
    /// Normal order
    #[serde(rename = "normal")]
    Normal,
    /// Market order
    #[serde(rename = "market")]
    Market,
    /// Elite spot trade to buy (elite traders)
    #[serde(rename = "spot_trader_buy")]
    SpotTraderBuy,
    /// Copy trade to buy (followers)
    #[serde(rename = "spot_follower_buy")]
    SpotFollowerBuy,
    /// Elite spot trade to sell (elite traders)
    #[serde(rename = "spot_trader_sell")]
    SpotTraderSell,
    /// Copy trade to sell (followers)
    #[serde(rename = "spot_follower_sell")]
    SpotFollowerSell,
}

/// Entry point source
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EntryPointSource {
    /// Web client
    #[serde(rename = "WEB")]
    Web,
    /// Mobile app
    #[serde(rename = "APP")]
    App,
    /// API client
    #[serde(rename = "API")]
    Api,
    /// System client
    #[serde(rename = "SYS")]
    Sys,
    /// Android client
    #[serde(rename = "ANDROID")]
    Android,
    /// iOS client
    #[serde(rename = "IOS")]
    #[allow(non_camel_case_types)]
    iOS,
}

/// Fee details for an order
#[derive(Debug, Clone, Deserialize)]
pub struct FeeDetails {
    /// Fee coin
    #[serde(rename = "feeCoin")]
    #[allow(dead_code)]
    pub fee_coin: String,

    /// Fee amount (negative value)
    #[allow(dead_code)]
    pub fee: String,
}

/// Cancel reason
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CancelReason {
    /// Normal cancel
    #[serde(rename = "normal_cancel")]
    NormalCancel,
    /// Cancelled by STP
    #[serde(rename = "stp_cancel")]
    STPCancel,
}

/// Detailed order information response
#[derive(Debug, Clone, Deserialize)]
pub struct GetOrderInfoResponse {
    /// User ID
    #[serde(rename = "userId")]
    pub user_id: String,

    /// Trading pair name
    pub symbol: String,

    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: String,

    /// Customized ID
    #[serde(rename = "clientOid")]
    pub client_order_id: Option<String>,

    /// Order price
    pub price: String,

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
    #[serde(rename = "priceAvg")]
    pub price_avg: String,

    /// Filled quantity (base coin)
    #[serde(rename = "baseVolume")]
    pub base_volume: String,

    /// Total trading amount (quote coin)
    #[serde(rename = "quoteVolume")]
    pub quote_volume: String,

    /// Entry point source
    #[serde(rename = "enterPointSource")]
    pub entry_point_source: EntryPointSource,

    /// Transaction fee breakdown
    #[serde(rename = "feeDetail")]
    pub fee_detail: String,

    /// Order source
    #[serde(rename = "orderSource")]
    pub order_source: OrderSource,

    /// Cancel reason (if cancelled)
    #[serde(rename = "cancelReason")]
    pub cancel_reason: Option<String>,

    /// Creation time, Unix millisecond timestamp
    #[serde(rename = "cTime")]
    pub create_time: String,

    /// Update time, Unix millisecond timestamp
    #[serde(rename = "uTime")]
    pub update_time: String,
}

impl RestClient {
    /// Get detailed information about a specific order
    ///
    /// Retrieves comprehensive order information including fill status,
    /// fees, and timestamps.
    ///
    /// # Arguments
    /// * `request` - The order info request (by order ID or client order ID)
    ///
    /// # Rate Limit
    /// 20 requests per second per UID
    ///
    /// # Returns
    /// A result containing the order information or an error
    pub async fn get_order_info(
        &self,
        request: &GetOrderInfoRequest,
    ) -> RestResult<GetOrderInfoResponse> {
        let query_string = serde_urlencoded::to_string(&request)
            .map_err(|e| crate::bitget::Errors::Error(format!("Failed to encode query: {e}")))?;

        self.send_signed_request(
            "/api/v2/spot/trade/orderInfo",
            reqwest::Method::GET,
            Some(&query_string),
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
    fn test_get_order_info_request_by_order_id() {
        let request = GetOrderInfoRequest::by_order_id("1234567890");

        assert_eq!(request.order_id, Some("1234567890".to_string()));
        assert!(request.client_order_id.is_none());
    }

    #[test]
    fn test_get_order_info_request_by_client_order_id() {
        let request = GetOrderInfoRequest::by_client_order_id("my-order-123");

        assert!(request.order_id.is_none());
        assert_eq!(request.client_order_id, Some("my-order-123".to_string()));
    }

    #[test]
    fn test_get_order_info_request_serialization() {
        let request = GetOrderInfoRequest::by_order_id("1234567890");
        let query = serde_urlencoded::to_string(&request).unwrap();

        assert!(query.contains("orderId=1234567890"));
    }

    #[test]
    fn test_get_order_info_response_deserialization() {
        let json = r#"{
            "userId": "123456789",
            "symbol": "BTCUSDT",
            "orderId": "1001",
            "clientOid": "121211212122",
            "price": "50000",
            "size": "0.001",
            "orderType": "limit",
            "side": "buy",
            "status": "filled",
            "priceAvg": "50000.0",
            "baseVolume": "0.001",
            "quoteVolume": "50.0",
            "enterPointSource": "API",
            "feeDetail": "{}",
            "orderSource": "normal",
            "cancelReason": "",
            "cTime": "1695865232127",
            "uTime": "1695865233051"
        }"#;

        let response: GetOrderInfoResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.user_id, "123456789");
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.order_id, "1001");
        assert_eq!(response.client_order_id, Some("121211212122".to_string()));
        assert_eq!(response.price, "50000");
        assert_eq!(response.size, "0.001");
        assert_eq!(response.order_type, OrderType::Limit);
        assert_eq!(response.side, OrderSide::Buy);
        assert_eq!(response.status, OrderStatus::Filled);
        assert_eq!(response.entry_point_source, EntryPointSource::Api);
        assert_eq!(response.order_source, OrderSource::Normal);
    }
}
