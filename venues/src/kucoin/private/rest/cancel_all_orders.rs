use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::kucoin::{ResponseHeaders, RestResponse, Result};

use super::RestClient;

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
    /// Reference: https://docs.kucoin.com/#cancel-all-hf-orders-by-symbol
    pub async fn cancel_all_orders(
        &self,
        request: CancelAllOrdersRequest,
    ) -> Result<(CancelAllOrdersResponse, ResponseHeaders)> {
        let mut params = HashMap::new();
        
        if let Some(symbol) = request.symbol {
            params.insert("symbol".to_string(), symbol);
        }
        
        if let Some(trade_type) = request.trade_type {
            params.insert("tradeType".to_string(), trade_type);
        }

        let params_option = if params.is_empty() { None } else { Some(params) };

        let (response, headers): (RestResponse<CancelAllOrdersResponse>, ResponseHeaders) =
            self.delete("/api/v1/hf/orders", params_option).await?;

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
