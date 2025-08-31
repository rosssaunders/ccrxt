//! BitMart submit batch order REST API endpoint
//!
//! This module implements the BitMart batch order API endpoint for placing multiple orders in a single request.

use serde::{Deserialize, Serialize};

use crate::bitmart::{
    OrderSide, OrderType, RestResult, StpMode, rate_limit::EndpointType,
    spot::private_client::RestClient,
};

const SUBMIT_BATCH_ORDER_ENDPOINT: &str = "/spot/v4/batch_orders";

/// Parameters for a single order in a batch
#[derive(Debug, Serialize)]
pub struct BatchOrderParam {
    /// Order side (buy/sell)
    pub side: OrderSide,
    /// Order type (limit/market/limit_maker/ioc)
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// Client-defined OrderId (optional, max 32 characters)
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    /// Self-trade prevention mode (default: none)
    #[serde(rename = "stpMode", skip_serializing_if = "Option::is_none")]
    pub stp_mode: Option<StpMode>,
    /// Order size (required for limit/limit_maker/ioc and market sell orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// Price (required for limit/limit_maker/ioc orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Notional amount (required for market buy orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notional: Option<String>,
}

/// Request parameters for submitting batch orders
#[derive(Debug, Serialize)]
pub struct SubmitBatchOrderRequest {
    /// Trading pair (e.g. BTC_USDT)
    pub symbol: String,
    /// Order parameters list (max 10 orders)
    #[serde(rename = "orderParams")]
    pub order_params: Vec<BatchOrderParam>,
    /// Trade time limit in milliseconds, allowed range (0,60000], default: 5000
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Response for submitting batch orders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitBatchOrderResponse {
    /// Code indicating success or failure
    pub code: i32,
    /// Response message
    pub msg: String,
    /// Response data containing order IDs
    pub data: BatchOrderData,
}

/// Data portion of batch order response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOrderData {
    /// List of order IDs for successfully placed orders
    #[serde(rename = "orderIds")]
    pub order_ids: Vec<String>,
}

impl RestClient {
    /// New Batch Order (v4)
    ///
    /// Places multiple orders in a single request. Maximum 10 orders per batch.
    ///
    /// [docs](https://developer-pro.bitmart.com/en/spot/#new-batch-orderv4-signed)
    ///
    /// Rate limit: varies by endpoint type
    ///
    /// # Arguments
    /// * `request` - The batch order request parameters
    ///
    /// # Returns
    /// Batch order submission response with order IDs
    pub async fn submit_batch_order(
        &self,
        request: SubmitBatchOrderRequest,
    ) -> RestResult<SubmitBatchOrderResponse> {
        if request.order_params.len() > 10 {
            return Err(crate::bitmart::Errors::Error(
                "Maximum 10 orders allowed per batch".to_string(),
            ));
        }

        self.send_post_signed_request(
            SUBMIT_BATCH_ORDER_ENDPOINT,
            request,
            EndpointType::SpotTrading,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_order_request() {
        let order_param = BatchOrderParam {
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            client_order_id: Some("test_order_123".to_string()),
            stp_mode: Some(StpMode::None),
            size: Some("0.001".to_string()),
            price: Some("50000.00".to_string()),
            notional: None,
        };

        let request = SubmitBatchOrderRequest {
            symbol: "BTC_USDT".to_string(),
            order_params: vec![order_param],
            recv_window: Some(5000),
        };

        assert_eq!(request.symbol, "BTC_USDT");
        assert_eq!(request.order_params.len(), 1);
        assert_eq!(request.recv_window, Some(5000));
        assert_eq!(request.order_params[0].side, OrderSide::Buy);
        assert_eq!(request.order_params[0].order_type, OrderType::Limit);
    }

    #[test]
    fn test_batch_order_param_market_buy() {
        let order_param = BatchOrderParam {
            side: OrderSide::Buy,
            order_type: OrderType::Market,
            client_order_id: None,
            stp_mode: None,
            size: None,
            price: None,
            notional: Some("100.00".to_string()),
        };

        assert_eq!(order_param.side, OrderSide::Buy);
        assert_eq!(order_param.order_type, OrderType::Market);
        assert!(order_param.size.is_none());
        assert!(order_param.price.is_none());
        assert_eq!(order_param.notional, Some("100.00".to_string()));
    }

    #[test]
    fn test_batch_order_param_market_sell() {
        let order_param = BatchOrderParam {
            side: OrderSide::Sell,
            order_type: OrderType::Market,
            client_order_id: None,
            stp_mode: None,
            size: Some("0.01".to_string()),
            price: None,
            notional: None,
        };

        assert_eq!(order_param.side, OrderSide::Sell);
        assert_eq!(order_param.order_type, OrderType::Market);
        assert_eq!(order_param.size, Some("0.01".to_string()));
        assert!(order_param.price.is_none());
        assert!(order_param.notional.is_none());
    }
}
