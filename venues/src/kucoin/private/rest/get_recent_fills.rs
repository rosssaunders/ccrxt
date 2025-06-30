use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::kucoin::{ResponseHeaders, RestResponse, Result, TradeSide};

use super::RestClient;

/// Request for getting recent fills
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetRecentFillsRequest {
    /// Symbol filter (optional)
    pub symbol: Option<String>,
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

impl RestClient {
    /// Get recent fills
    ///
    /// Reference: https://docs.kucoin.com/#recent-fills
    pub async fn get_recent_fills(
        &self,
        request: GetRecentFillsRequest,
    ) -> Result<(Vec<Fill>, ResponseHeaders)> {
        let mut params = HashMap::new();

        if let Some(symbol) = request.symbol {
            params.insert("symbol".to_string(), symbol);
        }

        let (response, headers): (RestResponse<Vec<Fill>>, ResponseHeaders) =
            self.get("/api/v1/limit/fills", Some(params)).await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recent_fills_request_default() {
        let request = GetRecentFillsRequest::default();
        assert!(request.symbol.is_none());
    }

    #[test]
    fn test_recent_fills_request_creation() {
        let request = GetRecentFillsRequest {
            symbol: Some("ETH-USDT".to_string()),
        };
        assert_eq!(request.symbol, Some("ETH-USDT".to_string()));
    }
}
