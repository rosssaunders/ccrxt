// Position Risk endpoint implementation for GET /dapi/v1/positionRisk
// See: https://binance-docs.github.io/apidocs/delivery/en/#position-information-user_data

use serde::{Deserialize, Serialize};
use crate::binance::coinm::private::rest::client::RestClient;
use crate::binance::coinm::RestResult;

/// Request parameters for position risk (GET /dapi/v1/positionRisk).
#[derive(Debug, Clone, Serialize, Default)]
pub struct PositionRiskRequest {
    /// Margin asset. Optional.
    #[serde(rename = "marginAsset", skip_serializing_if = "Option::is_none")]
    pub margin_asset: Option<String>,

    /// Trading pair. Optional.
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,

    /// The value cannot be greater than 60000. Optional.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp (milliseconds since epoch).
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Response for position risk (GET /dapi/v1/positionRisk).
#[derive(Debug, Clone, Deserialize)]
pub struct PositionRisk {
    /// Trading symbol (e.g., "BTCUSD_201225").
    pub symbol: String,
    /// Position amount.
    #[serde(rename = "positionAmt")]
    pub position_amt: String,
    /// Entry price.
    #[serde(rename = "entryPrice")]
    pub entry_price: String,
    /// Break-even price.
    #[serde(rename = "breakEvenPrice")]
    pub break_even_price: String,
    /// Mark price.
    #[serde(rename = "markPrice")]
    pub mark_price: String,
    /// Unrealized profit.
    #[serde(rename = "unRealizedProfit")]
    pub unrealized_profit: String,
    /// Liquidation price.
    #[serde(rename = "liquidationPrice")]
    pub liquidation_price: String,
    /// Leverage.
    pub leverage: String,
    /// Maximum quantity of base asset.
    #[serde(rename = "maxQty")]
    pub max_qty: String,
    /// Margin type (e.g., "cross").
    #[serde(rename = "marginType")]
    pub margin_type: String,
    /// Isolated margin.
    #[serde(rename = "isolatedMargin")]
    pub isolated_margin: String,
    /// Is auto add margin enabled.
    #[serde(rename = "isAutoAddMargin")]
    pub is_auto_add_margin: String,
    /// Position side ("BOTH", "LONG", "SHORT").
    #[serde(rename = "positionSide")]
    pub position_side: String,
    /// Update time (milliseconds since epoch).
    #[serde(rename = "updateTime")]
    pub update_time: u64,
}

impl RestClient {
    /// Get current account position risk information on Binance Coin-M Futures.
    ///
    /// See: https://binance-docs.github.io/apidocs/delivery/en/#position-information-user_data
    /// GET /dapi/v1/positionRisk
    /// Weight: 1
    /// Requires API key and signature.
    pub async fn get_position_risk(
        &self,
        params: PositionRiskRequest,
    ) -> RestResult<Vec<PositionRisk>> {
        let weight = 1;
        self.send_signed_request(
            "/dapi/v1/positionRisk",
            reqwest::Method::GET,
            params,
            weight,
            false,
        )
        .await
    }
}
