use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::kucoin::{ResponseHeaders, RestResponse, Result};

/// Endpoint URL for cancel all orders
pub const CANCEL_ALL_ORDERS_ENDPOINT: &str = "/api/v1/orders";

#[derive(Debug, Clone, Default, Serialize)]
pub struct CancelAllOrdersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrdersResponse {
    pub cancelled_order_ids: Vec<String>,
}

impl super::RestClient {
    /// Cancel all orders
    pub async fn cancel_all_orders(
        &self,
        request: CancelAllOrdersRequest,
    ) -> Result<(RestResponse<CancelAllOrdersResponse>, ResponseHeaders)> {
        let endpoint = CANCEL_ALL_ORDERS_ENDPOINT;
        let mut params = HashMap::new();
        if let Some(symbol) = request.symbol {
            params.insert("symbol".to_string(), symbol);
        }
        let params = if params.is_empty() {
            None
        } else {
            Some(params)
        };
        self.delete(endpoint, params).await
    }
}
