use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result, TradeSide};

const FILLS_ENDPOINT: &str = "/api/v1/fills";

/// Request for getting fills (trade history)
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetFillsRequest {
    /// Trading symbol filter (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Order ID filter (optional)
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    /// Trade side filter (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<TradeSide>,

    /// Trade type filter (optional)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub trade_type: Option<String>,

    /// Start time filter (optional, milliseconds)
    #[serde(rename = "startAt", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,

    /// End time filter (optional, milliseconds)
    #[serde(rename = "endAt", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
}

/// Fill (trade) information
#[derive(Debug, Clone, Deserialize)]
pub struct Fill {
    /// Trade ID
    #[serde(rename = "tradeId")]
    pub trade_id: String,

    /// Symbol
    pub symbol: String,

    /// Counter order ID
    #[serde(rename = "counterOrderId")]
    pub counter_order_id: String,

    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: String,

    /// Side
    pub side: TradeSide,

    /// Liquidity (taker/maker)
    pub liquidity: String,

    /// Force taker flag
    #[serde(rename = "forceTaker")]
    pub force_taker: bool,

    /// Price
    pub price: String,

    /// Size
    pub size: String,

    /// Funds
    pub funds: String,

    /// Fee
    pub fee: String,

    /// Fee rate
    #[serde(rename = "feeRate")]
    pub fee_rate: String,

    /// Fee currency
    #[serde(rename = "feeCurrency")]
    pub fee_currency: String,

    /// Stop type
    pub stop: String,

    /// Trade type
    #[serde(rename = "type")]
    pub trade_type: String,

    /// Creation time
    #[serde(rename = "createdAt")]
    pub created_at: i64,

    /// Trade time
    #[serde(rename = "tradeTime")]
    pub trade_time: i64,
}

/// Response wrapper for fills
#[derive(Debug, Clone, Deserialize)]
pub struct FillsResponse {
    /// Current page
    #[serde(rename = "currentPage")]
    pub current_page: i32,

    /// Page size
    #[serde(rename = "pageSize")]
    pub page_size: i32,

    /// Total number of records
    #[serde(rename = "totalNum")]
    pub total_num: i32,

    /// Total pages
    #[serde(rename = "totalPage")]
    pub total_page: i32,

    /// Fill items
    pub items: Vec<Fill>,
}

impl RestClient {
    /// Get fills (trade history)
    ///
    /// Reference: https://docs.kucoin.com/#list-fills
    pub async fn get_fills(
        &self,
        request: GetFillsRequest,
    ) -> Result<(FillsResponse, ResponseHeaders)> {
        let (response, headers): (RestResponse<FillsResponse>, ResponseHeaders) =
            self.get_with_request(FILLS_ENDPOINT, &request).await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fills_request_default() {
        let request = GetFillsRequest::default();
        assert!(request.symbol.is_none());
        assert!(request.order_id.is_none());
    }

    #[test]
    fn test_fills_request_creation() {
        let request = GetFillsRequest {
            symbol: Some("BTC-USDT".to_string()),
            order_id: Some("test_order".to_string()),
            ..Default::default()
        };
        assert_eq!(request.symbol, Some("BTC-USDT".to_string()));
        assert_eq!(request.order_id, Some("test_order".to_string()));
    }
}
