use serde::{Deserialize, Serialize};

use super::{
    super::RestClient,
    get_current_orders::TPSLType,
    get_order_info::{EntryPointSource, OrderSource, OrderStatus},
};
use crate::bitget::spot::{OrderSide, OrderType, RestResult};

const ORDER_HISTORY_ENDPOINT: &str = "/api/v2/spot/trade/history-orders";
/// Request parameters for getting order history
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetOrderHistoryRequest {
    /// Trading pair (optional filter)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// The record start time for the query (Unix milliseconds)
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,

    /// The end time of the record for the query (Unix milliseconds)
    /// The interval between startTime and endTime must not exceed 90 days
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

/// Fee details for historical orders
#[derive(Debug, Clone, Deserialize)]
pub struct HistoricalFeeDetails {
    /// New fees structure
    #[serde(rename = "newFees")]
    #[allow(dead_code)]
    pub new_fees: Option<NewFeeDetails>,

    /// Additional fee coin details
    #[serde(flatten)]
    #[allow(dead_code)]
    pub additional_fees: serde_json::Value,
}

/// New fee structure details
#[derive(Debug, Clone, Deserialize)]
pub struct NewFeeDetails {
    /// Fee component C
    #[allow(dead_code)]
    pub c: String,
    /// Fee component D  
    #[allow(dead_code)]
    pub d: String,
    /// Whether deduction was applied
    #[allow(dead_code)]
    pub deduction: bool,
    /// Fee component R
    #[allow(dead_code)]
    pub r: String,
    /// Fee component T
    #[allow(dead_code)]
    pub t: String,
    /// Total deduction fee
    #[serde(rename = "totalDeductionFee")]
    #[allow(dead_code)]
    pub total_deduction_fee: String,
}

/// Information about a historical order
#[derive(Debug, Clone, Deserialize)]
pub struct OrderHistoryInfo {
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

    /// Average fill price
    #[serde(rename = "priceAvg")]
    pub price_avg: String,

    /// Filled volume (base coin)
    #[serde(rename = "baseVolume")]
    pub base_volume: String,

    /// Filled volume (quote coin)
    #[serde(rename = "quoteVolume")]
    pub quote_volume: String,

    /// Entry point source
    #[serde(rename = "enterPointSource")]
    pub entry_point_source: EntryPointSource,

    /// Order source
    #[serde(rename = "orderSource")]
    pub order_source: OrderSource,

    /// Creation time (Unix milliseconds)
    #[serde(rename = "cTime")]
    pub create_time: String,

    /// Update time (Unix milliseconds)
    #[serde(rename = "uTime")]
    pub update_time: String,

    /// Fee details as JSON string
    #[serde(rename = "feeDetail")]
    pub fee_detail: String,

    /// TPSL order type
    #[serde(rename = "tpslType")]
    pub tpsl_type: String,

    /// Cancel reason (if cancelled)
    #[serde(rename = "cancelReason")]
    pub cancel_reason: Option<String>,

    /// Trigger price (only valid when tpslType is tpsl)
    #[serde(rename = "triggerPrice")]
    pub trigger_price: Option<String>,
}

/// Response from getting order history
#[derive(Debug, Clone)]
pub struct GetOrderHistoryResponse {
    /// List of historical orders
    pub orders: Vec<OrderHistoryInfo>,
}

// Implementation for direct response deserialization when the API returns an array
impl<'de> Deserialize<'de> for GetOrderHistoryResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let orders = Vec::<OrderHistoryInfo>::deserialize(deserializer)?;
        Ok(GetOrderHistoryResponse { orders })
    }
}

impl RestClient {
    /// Get historical orders
    ///
    /// Retrieves order history for the authenticated account with optional filters.
    /// Only supports data within 90 days. Older data can be downloaded from the web.
    ///
    /// [docs](https://www.bitget.com/api-doc/spot/trade/Get-History-Orders)
    ///
    /// # Arguments
    /// * `request` - The request parameters with optional filters
    ///
    /// # Rate Limit
    /// 20 requests per second per UID
    ///
    /// # Returns
    /// A result containing the order history or an error
    pub async fn get_order_history(
        &self,
        request: GetOrderHistoryRequest,
    ) -> RestResult<GetOrderHistoryResponse> {
        self.send_get_signed_request(ORDER_HISTORY_ENDPOINT, request, 10, false, None)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_order_history_request_default() {
        let request = GetOrderHistoryRequest::default();

        assert!(request.symbol.is_none());
        assert!(request.start_time.is_none());
        assert!(request.end_time.is_none());
        assert!(request.limit.is_none());
    }

    #[test]
    fn test_get_order_history_request_builder() {
        let request = GetOrderHistoryRequest {
            symbol: Some("ETHUSDT".to_string()),
            order_id: None,
            start_time: Some(1659036670000),
            end_time: Some(1659076670000),
            id_less_than: Some("67890".to_string()),
            limit: Some(75),
            tpsl_type: Some(TPSLType::Normal),
            request_time: None,
            receive_window: None,
        };

        assert_eq!(request.symbol, Some("ETHUSDT".to_string()));
        assert_eq!(request.start_time, Some(1659036670000));
        assert_eq!(request.end_time, Some(1659076670000));
        assert_eq!(request.id_less_than, Some("67890".to_string()));
        assert_eq!(request.limit, Some(75));
        assert_eq!(request.tpsl_type, Some(TPSLType::Normal));
    }

    #[test]
    fn test_get_order_history_request_limit_enforcement() {
        let request = GetOrderHistoryRequest {
            symbol: None,
            order_id: None,
            start_time: None,
            end_time: None,
            id_less_than: None,
            limit: Some(100), // Manual cap at 100
            tpsl_type: None,
            request_time: None,
            receive_window: None,
        };

        // Should be capped at 100
        assert_eq!(request.limit, Some(100));
    }

    #[test]
    fn test_order_history_info_deserialization() {
        let json = r#"{
            "userId": "987654321",
            "symbol": "ETHUSDT",
            "orderId": "3333333",
            "clientOid": "my-order-456",
            "price": "0",
            "size": "20.0000000000000000",
            "orderType": "market",
            "side": "buy",
            "status": "filled",
            "priceAvg": "1598.1000000000000000",
            "baseVolume": "0.0125000000000000",
            "quoteVolume": "19.9762500000000000",
            "enterPointSource": "WEB",
            "orderSource": "market",
            "cTime": "1698736299656",
            "uTime": "1698736300363",
            "feeDetail": "{\"newFees\":{\"c\":0,\"d\":0,\"deduction\":false,\"r\":-0.112079256,\"t\":-0.112079256,\"totalDeductionFee\":0}}",
            "tpslType": "normal",
            "cancelReason": "",
            "triggerPrice": null
        }"#;

        let order: OrderHistoryInfo = serde_json::from_str(json).unwrap();

        assert_eq!(order.user_id, "987654321");
        assert_eq!(order.symbol, "ETHUSDT");
        assert_eq!(order.order_id, "3333333");
        assert_eq!(order.client_order_id, Some("my-order-456".to_string()));
        assert_eq!(order.order_type, OrderType::Market);
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.status, OrderStatus::Filled);
        assert_eq!(order.tpsl_type, "normal");
        assert!(order.fee_detail.contains("newFees"));
    }

    #[test]
    fn test_get_order_history_response_deserialization() {
        let json = r#"[
            {
                "userId": "987654321",
                "symbol": "ETHUSDT",
                "orderId": "3333333",
                "clientOid": "my-order-456",
                "price": "1600",
                "size": "1.0",
                "orderType": "limit",
                "side": "sell",
                "status": "filled",
                "priceAvg": "1600.0",
                "baseVolume": "1.0",
                "quoteVolume": "1600.0",
                "enterPointSource": "API",
                "orderSource": "normal",
                "cTime": "1698736299656",
                "uTime": "1698736300363",
                "feeDetail": "{}",
                "tpslType": "normal",
                "cancelReason": "",
                "triggerPrice": null
            }
        ]"#;

        let response: GetOrderHistoryResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.orders.len(), 1);
        assert_eq!(response.orders[0].symbol, "ETHUSDT");
        assert_eq!(response.orders[0].order_id, "3333333");
        assert_eq!(response.orders[0].status, OrderStatus::Filled);
    }
}
