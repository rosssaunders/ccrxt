use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

const CANCEL_ALL_ORDERS_ENDPOINT: &str = "/api/v1/hf/orders";

/// Request for cancelling all orders
#[derive(Debug, Clone, Serialize)]
pub struct CancelAllOrdersRequest {
    /// Symbol to cancel orders for (optional)
    pub symbol: Option<String>,

    /// Trade type (optional)
    #[serde(rename = "tradeType")]
    pub trade_type: Option<String>,
}

/// Cancel all orders response
#[derive(Debug, Clone, Deserialize)]
pub struct CancelAllOrdersResponse {
    /// List of cancelled order IDs
    #[serde(rename = "cancelledOrderIds")]
    pub cancelled_order_ids: Vec<String>,
}

impl RestClient {
    /// Cancel all orders (optionally filtered by symbol)
    ///
    /// [docs](https://docs.kucoin.com/#cancel-all-hf-orders-by-symbol)
    pub async fn cancel_all_orders(
        &self,
        request: CancelAllOrdersRequest,
    ) -> Result<(CancelAllOrdersResponse, ResponseHeaders)> {
        let (response, headers): (RestResponse<CancelAllOrdersResponse>, ResponseHeaders) = self
            .delete_with_request(CANCEL_ALL_ORDERS_ENDPOINT, &request)
            .await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_all_orders_request_creation() {
        let request = CancelAllOrdersRequest {
            symbol: Some("BTC-USDT".to_string()),
            trade_type: Some("TRADE".to_string()),
        };
        assert_eq!(request.symbol, Some("BTC-USDT".to_string()));
        assert_eq!(request.trade_type, Some("TRADE".to_string()));
    }

    #[test]
    fn test_cancel_all_orders_request_minimal() {
        let request = CancelAllOrdersRequest {
            symbol: None,
            trade_type: None,
        };
        assert!(request.symbol.is_none());
        assert!(request.trade_type.is_none());
    }
}
