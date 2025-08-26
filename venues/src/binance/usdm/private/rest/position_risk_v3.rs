// No top-of-file comments per project instructions.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::binance::usdm::PrivateRestClient as RestClient;
use crate::binance::usdm::{RestResult, enums::*};

/// Endpoint path for Position Risk V3.
const POSITION_RISK_V3_ENDPOINT: &str = "/fapi/v3/positionRisk";

/// Request parameters for the Position Risk V3 endpoint.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionRiskV3Request {
    /// Trading symbol (e.g., "BTCUSDT"). Optional.
    /// If not provided, returns all symbols with open positions or orders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Cow<'static, str>>,

    /// The number of milliseconds after timestamp the request is valid for. Optional.
    /// If omitted, default is 5000ms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp (milliseconds since epoch). Required.
    pub timestamp: u64,
}

/// Position risk information for a single symbol.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionRiskV3 {
    /// Trading symbol (e.g., "BTCUSDT").
    pub symbol: String,

    /// Position side (e.g., LONG, SHORT, BOTH).
    pub position_side: PositionSide,

    /// Position amount (positive for long, negative for short).
    pub position_amt: String,

    /// Entry price for the position.
    pub entry_price: String,

    /// Break-even price for the position.
    pub break_even_price: String,

    /// Mark price for the symbol.
    pub mark_price: String,

    /// Unrealized profit for the position.
    pub un_realized_profit: String,

    /// Liquidation price for the position.
    pub liquidation_price: String,

    /// Isolated margin for the position.
    pub isolated_margin: String,

    /// Notional value of the position.
    pub notional: String,

    /// Margin asset for the position (e.g., "USDT").
    pub margin_asset: String,

    /// Isolated wallet balance for the position.
    pub isolated_wallet: String,

    /// Initial margin required with current mark price.
    pub initial_margin: String,

    /// Maintenance margin required.
    pub maint_margin: String,

    /// Initial margin required for positions with current mark price.
    pub position_initial_margin: String,

    /// Initial margin required for open orders with current mark price.
    pub open_order_initial_margin: String,

    /// ADL quantile indicator.
    #[serde(rename = "adl")]
    pub adl_quantile: u8,

    /// Bids notional (ignored).
    pub bid_notional: String,

    /// Asks notional (ignored).
    pub ask_notional: String,

    /// Update time (milliseconds since epoch).
    pub update_time: Option<u64>,

    /// Leverage used for the position.
    pub leverage: String,

    /// Maximum notional value allowed for the position.
    pub max_notional_value: String,

    /// Margin type for the position (isolated or cross).
    pub margin_type: MarginType,

    /// Whether auto add margin is enabled.
    pub is_auto_add_margin: bool,
}

impl RestClient {
    /// Position Information V3
    ///
    /// Get current position information (only symbols with position or open orders will be returned).
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Position-Information-V3)
    ///
    /// Rate limit: 5
    ///
    /// # Arguments
    /// * `request` - The request parameters for Position Risk V3. See [`GetPositionRiskV3Request`].
    ///
    /// # Returns
    /// Returns a vector of position risk information for each symbol. See [`PositionRiskV3`].
    pub async fn get_position_risk_v3(
        &self,
        request: GetPositionRiskV3Request,
    ) -> RestResult<Vec<PositionRiskV3>> {
        self.send_get_signed_request(POSITION_RISK_V3_ENDPOINT, request, 5, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_risk_v3_response_deserialization() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "positionSide": "LONG",
                "positionAmt": "0.1",
                "entryPrice": "50000.0",
                "breakEvenPrice": "50025.0",
                "markPrice": "51000.0",
                "unRealizedProfit": "100.0",
                "liquidationPrice": "45000.0",
                "isolatedMargin": "510.0",
                "notional": "5100.0",
                "marginAsset": "USDT",
                "isolatedWallet": "510.0",
                "initialMargin": "510.0",
                "maintMargin": "51.0",
                "positionInitialMargin": "510.0",
                "openOrderInitialMargin": "0.0",
                "adl": 2,
                "bidNotional": "0.0",
                "askNotional": "0.0",
                "updateTime": 1720736417660,
                "leverage": "10",
                "maxNotionalValue": "1000000",
                "marginType": "ISOLATED",
                "isAutoAddMargin": false
            }
        ]"#;

        let result: Vec<PositionRiskV3> = serde_json::from_str(json).unwrap();
        assert_eq!(result.len(), 1);
        let pos = &result[0];
        assert_eq!(pos.symbol, "BTCUSDT");
        assert_eq!(pos.position_side, PositionSide::Long);
        assert_eq!(pos.margin_type, MarginType::Isolated);
        assert_eq!(pos.adl_quantile, 2);
        assert_eq!(pos.update_time, Some(1720736417660));
        assert_eq!(pos.margin_asset, "USDT");
        assert_eq!(pos.leverage, "10");
        assert_eq!(pos.max_notional_value, "1000000");
    }

    #[test]
    fn test_get_position_risk_v3_request_serialization() {
        let request = GetPositionRiskV3Request {
            symbol: Some(Cow::Borrowed("BTCUSDT")),
            recv_window: Some(10000),
            timestamp: 1720736417660,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("recvWindow=10000"));
        assert!(serialized.contains("timestamp=1720736417660"));
    }
}
