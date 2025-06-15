//! Position endpoints for Binance Options Private API

use serde::{Deserialize, Serialize};

/// Request parameters for position information
#[derive(Debug, Clone, Serialize)]
pub struct PositionRequest {
    /// Option trading pair, e.g. BTC-200730-9000-C (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}



/// Position information response
#[derive(Debug, Clone, Deserialize)]
pub struct PositionResponse {
    /// Average entry price
    #[serde(rename = "entryPrice")]
    pub entry_price: String,
    /// Option trading pair
    pub symbol: String,
    /// Position direction
    pub side: String,
    /// Number of positions (positive numbers represent long positions, negative number represent short positions)
    pub quantity: String,
    /// Number of positions that can be reduced
    #[serde(rename = "reducibleQty")]
    pub reducible_qty: String,
    /// Current market value
    #[serde(rename = "markValue")]
    pub mark_value: String,
    /// Rate of return
    pub ror: String,
    /// Unrealized profit/loss
    #[serde(rename = "unrealizedPNL")]
    pub unrealized_pnl: String,
    /// Mark price
    #[serde(rename = "markPrice")]
    pub mark_price: String,
    /// Strike price
    #[serde(rename = "strikePrice")]
    pub strike_price: String,
    /// Position cost
    #[serde(rename = "positionCost")]
    pub position_cost: String,
    /// Exercise time
    #[serde(rename = "expiryDate")]
    pub expiry_date: u64,
    #[serde(rename = "priceScale")]
    pub price_scale: u32,
    #[serde(rename = "quantityScale")]
    pub quantity_scale: u32,
    #[serde(rename = "optionSide")]
    pub option_side: String,
    #[serde(rename = "quoteAsset")]
    pub quote_asset: String,
}

use crate::binance::options::{PrivateRestClient, RestResult};

impl PrivateRestClient {
    /// Get current position information
    ///
    /// # Arguments
    /// * `request` - Position request parameters (optional symbol and recv_window)
    ///
    /// # Returns
    /// List of position information
    ///
    /// # Weight
    /// 5
    pub async fn get_position(&self, request: PositionRequest) -> RestResult<Vec<PositionResponse>> {
        self.send_signed_request(
            "/eapi/v1/position",
            reqwest::Method::GET,
            request,
            5, // weight
            false, // not an order
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_request_creation() {
        let request = PositionRequest {
            symbol: None,
            recv_window: None,
        };
        assert!(request.symbol.is_none());

        let request_with_symbol = PositionRequest {
            symbol: Some("BTC-200730-9000-C".to_string()),
            recv_window: None,
        };
        assert_eq!(
            request_with_symbol.symbol,
            Some("BTC-200730-9000-C".to_string())
        );
    }
}