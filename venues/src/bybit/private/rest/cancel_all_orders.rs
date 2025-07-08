use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bybit::{EndpointType, RestResult, enums::*};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrdersRequest {
    pub category: Category,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_filter: Option<OrderFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_order_type: Option<StopOrderType>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelledOrder {
    pub order_id: String,
    pub order_link_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CancelAllOrdersData {
    pub list: Vec<CancelledOrder>,
    pub success: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CancelAllOrdersResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: CancelAllOrdersData,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

impl RestClient {
    /// Cancel all orders
    ///
    /// Cancel all open orders by symbol/baseCoin/settleCoin.
    ///
    /// # Arguments
    /// * `request` - The cancel all orders request parameters
    ///
    /// # Returns
    /// A result containing the cancel all orders response or an error
    pub async fn cancel_all_orders(
        &self,
        request: CancelAllOrdersRequest,
    ) -> RestResult<CancelAllOrdersResponse> {
        self.send_signed_request(
            "/v5/order/cancel-all",
            reqwest::Method::POST,
            request,
            EndpointType::Trade,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_all_orders_request_by_symbol() {
        let request = CancelAllOrdersRequest {
            category: Category::Linear,
            symbol: Some("BTCUSDT".to_string()),
            base_coin: None,
            settle_coin: None,
            order_filter: None,
            stop_order_type: None,
        };

        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.symbol, Some("BTCUSDT".to_string()));
        assert!(request.base_coin.is_none());
        assert!(request.settle_coin.is_none());
    }

    #[test]
    fn test_cancel_all_orders_request_by_base_coin() {
        let request = CancelAllOrdersRequest {
            category: Category::Spot,
            symbol: None,
            base_coin: Some("BTC".to_string()),
            settle_coin: None,
            order_filter: Some(OrderFilter::Order),
            stop_order_type: None,
        };

        assert_eq!(request.category, Category::Spot);
        assert!(request.symbol.is_none());
        assert_eq!(request.base_coin, Some("BTC".to_string()));
        assert_eq!(request.order_filter, Some(OrderFilter::Order));
    }

    #[test]
    fn test_cancel_all_orders_request_serialization() {
        let request = CancelAllOrdersRequest {
            category: Category::Linear,
            symbol: None,
            base_coin: None,
            settle_coin: Some("USDT".to_string()),
            order_filter: None,
            stop_order_type: Some(StopOrderType::Stop),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"category\":\"linear\""));
        assert!(json.contains("\"settleCoin\":\"USDT\""));
        assert!(json.contains("\"stopOrderType\":\"Stop\""));
        assert!(!json.contains("symbol")); // Should be skipped when None
    }
}
