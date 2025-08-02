use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::{RestResult, enums::*};

/// Endpoint path for Position Information V2.
const POSITION_RISK_ENDPOINT: &str = "/fapi/v2/positionRisk";

/// Request parameters for the Position Information V2 endpoint.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionRiskRequest {
    /// Trading symbol (e.g., "BTCUSDT"). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<std::borrow::Cow<'static, str>>,

    /// The number of milliseconds the request is valid for. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp (milliseconds since epoch).
    pub timestamp: u64,
}

/// Position risk information for a symbol.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionRisk {
    /// Entry price for the position.
    pub entry_price: std::borrow::Cow<'static, str>,

    /// Break-even price for the position.
    pub break_even_price: std::borrow::Cow<'static, str>,

    /// Margin type (cross or isolated).
    pub margin_type: MarginType,

    /// Whether auto add margin is enabled.
    pub is_auto_add_margin: bool,

    /// Isolated margin amount.
    pub isolated_margin: std::borrow::Cow<'static, str>,

    /// Leverage used for the position.
    pub leverage: std::borrow::Cow<'static, str>,

    /// Liquidation price for the position.
    pub liquidation_price: std::borrow::Cow<'static, str>,

    /// Mark price for the symbol.
    pub mark_price: std::borrow::Cow<'static, str>,

    /// Maximum notional value allowed.
    pub max_notional_value: std::borrow::Cow<'static, str>,

    /// Position amount (positive for long, negative for short).
    pub position_amt: std::borrow::Cow<'static, str>,

    /// Notional value of the position.
    pub notional: std::borrow::Cow<'static, str>,

    /// Isolated wallet amount.
    pub isolated_wallet: std::borrow::Cow<'static, str>,

    /// Trading symbol.
    pub symbol: std::borrow::Cow<'static, str>,

    /// Unrealized profit for the position.
    pub un_realized_profit: std::borrow::Cow<'static, str>,

    /// Position side (long, short, both).
    pub position_side: PositionSide,

    /// Last update time (milliseconds since epoch).
    pub update_time: u64,
}

impl UsdmClient {
    /// Position Information V2
    ///
    /// Retrieves position risk information for a symbol or all positions on Binance USDM Futures.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Position-Information-V2
    ///
    /// Rate limit: 5 requests per minute
    ///
    /// # Arguments
    /// * `request` - The position risk request parameters
    ///
    /// # Returns
    /// Returns a vector of `PositionRisk` containing position details.
    pub async fn get_position_risk(
        &self,
        request: GetPositionRiskRequest,
    ) -> RestResult<Vec<PositionRisk>> {
        self.send_get_signed_request(POSITION_RISK_ENDPOINT, request, 5, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_position_risk_request_serialization() {
        let request = GetPositionRiskRequest {
            symbol: Some("BTCUSDT".into()),
            recv_window: Some(5000),
            timestamp: 1234567890,
        };
        let serialized = serde_urlencoded::to_string(&request).expect("serialization failed");
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1234567890"));
    }

    #[test]
    fn test_get_position_risk_request_no_symbol() {
        let request = GetPositionRiskRequest {
            symbol: None,
            recv_window: None,
            timestamp: 1234567890,
        };
        let serialized = serde_urlencoded::to_string(&request).expect("serialization failed");
        assert_eq!(serialized, "timestamp=1234567890");
    }

    #[test]
    fn test_position_risk_deserialization() {
        let json = r#"{
            "entryPrice": "45380.10",
            "breakEvenPrice": "45300.00",
            "marginType": "CROSS",
            "isAutoAddMargin": true,
            "isolatedMargin": "0.00000000",
            "leverage": "20",
            "liquidationPrice": "43111.095",
            "markPrice": "45390.20",
            "maxNotionalValue": "500000",
            "positionAmt": "0.100",
            "notional": "4539.02",
            "isolatedWallet": "0.00000000",
            "symbol": "BTCUSDT",
            "unRealizedProfit": "1.01000000",
            "positionSide": "BOTH",
            "updateTime": 1655217461579
        }"#;

        let position_risk: PositionRisk =
            serde_json::from_str(json).expect("deserialization failed");
        assert_eq!(position_risk.entry_price, "45380.10");
        assert_eq!(position_risk.break_even_price, "45300.00");
        assert_eq!(position_risk.margin_type, MarginType::Cross);
        assert!(position_risk.is_auto_add_margin);
        assert_eq!(position_risk.isolated_margin, "0.00000000");
        assert_eq!(position_risk.leverage, "20");
        assert_eq!(position_risk.liquidation_price, "43111.095");
        assert_eq!(position_risk.mark_price, "45390.20");
        assert_eq!(position_risk.max_notional_value, "500000");
        assert_eq!(position_risk.position_amt, "0.100");
        assert_eq!(position_risk.notional, "4539.02");
        assert_eq!(position_risk.isolated_wallet, "0.00000000");
        assert_eq!(position_risk.symbol, "BTCUSDT");
        assert_eq!(position_risk.un_realized_profit, "1.01000000");
        assert_eq!(position_risk.position_side, PositionSide::Both);
        assert_eq!(position_risk.update_time, 1655217461579);
    }

    #[test]
    fn test_position_risk_isolated_margin() {
        let json = r#"{
            "entryPrice": "3070.50",
            "breakEvenPrice": "3070.00",
            "marginType": "ISOLATED",
            "isAutoAddMargin": false,
            "isolatedMargin": "100.00000000",
            "leverage": "10",
            "liquidationPrice": "2800.00",
            "markPrice": "3080.60",
            "maxNotionalValue": "100000",
            "positionAmt": "1.000",
            "notional": "3080.60",
            "isolatedWallet": "100.00000000",
            "symbol": "ETHUSDT",
            "unRealizedProfit": "10.10000000",
            "positionSide": "LONG",
            "updateTime": 1655217461579
        }"#;

        let position_risk: PositionRisk =
            serde_json::from_str(json).expect("deserialization failed");
        assert_eq!(position_risk.margin_type, MarginType::Isolated);
        assert!(!position_risk.is_auto_add_margin);
        assert_eq!(position_risk.isolated_margin, "100.00000000");
        assert_eq!(position_risk.position_side, PositionSide::Long);
        assert_eq!(position_risk.isolated_wallet, "100.00000000");
    }

    #[test]
    fn test_position_risk_short_position() {
        let json = r#"{
            "entryPrice": "45400.00",
            "breakEvenPrice": "45400.00",
            "marginType": "CROSS",
            "isAutoAddMargin": true,
            "isolatedMargin": "0.00000000",
            "leverage": "5",
            "liquidationPrice": "54480.00",
            "markPrice": "45350.00",
            "maxNotionalValue": "1000000",
            "positionAmt": "-0.500",
            "notional": "-22675.00",
            "isolatedWallet": "0.00000000",
            "symbol": "BTCUSDT",
            "unRealizedProfit": "25.00000000",
            "positionSide": "SHORT",
            "updateTime": 1655217461579
        }"#;

        let position_risk: PositionRisk =
            serde_json::from_str(json).expect("deserialization failed");
        assert_eq!(position_risk.position_amt, "-0.500");
        assert_eq!(position_risk.position_side, PositionSide::Short);
        assert_eq!(position_risk.un_realized_profit, "25.00000000");
        assert_eq!(position_risk.notional, "-22675.00");
    }

    #[test]
    fn test_position_risk_array_deserialization() {
        let json = r#"[
            {
                "entryPrice": "45380.10",
                "breakEvenPrice": "45300.00",
                "marginType": "CROSS",
                "isAutoAddMargin": true,
                "isolatedMargin": "0.00000000",
                "leverage": "20",
                "liquidationPrice": "43111.095",
                "markPrice": "45390.20",
                "maxNotionalValue": "500000",
                "positionAmt": "0.100",
                "notional": "4539.02",
                "isolatedWallet": "0.00000000",
                "symbol": "BTCUSDT",
                "unRealizedProfit": "1.01000000",
                "positionSide": "BOTH",
                "updateTime": 1655217461579
            },
            {
                "entryPrice": "3070.50",
                "breakEvenPrice": "3070.00",
                "marginType": "CROSS",
                "isAutoAddMargin": true,
                "isolatedMargin": "0.00000000",
                "leverage": "10",
                "liquidationPrice": "2763.45",
                "markPrice": "3080.60",
                "maxNotionalValue": "100000",
                "positionAmt": "1.000",
                "notional": "3080.60",
                "isolatedWallet": "0.00000000",
                "symbol": "ETHUSDT",
                "unRealizedProfit": "10.10000000",
                "positionSide": "BOTH",
                "updateTime": 1655217461579
            }
        ]"#;

        let positions: Vec<PositionRisk> =
            serde_json::from_str(json).expect("array deserialization failed");
        assert_eq!(positions.len(), 2);
        assert_eq!(positions[0].symbol, "BTCUSDT");
        assert_eq!(positions[1].symbol, "ETHUSDT");
        assert_eq!(positions[0].break_even_price, "45300.00");
        assert_eq!(positions[1].break_even_price, "3070.00");
    }

    #[test]
    fn test_position_risk_high_leverage() {
        let json = r#"{
            "entryPrice": "0.50000",
            "breakEvenPrice": "0.50000",
            "marginType": "ISOLATED",
            "isAutoAddMargin": false,
            "isolatedMargin": "10.00000000",
            "leverage": "125",
            "liquidationPrice": "0.49200",
            "markPrice": "0.50100",
            "maxNotionalValue": "50000",
            "positionAmt": "1000.000",
            "notional": "501.00",
            "isolatedWallet": "10.00000000",
            "symbol": "DOGEUSDT",
            "unRealizedProfit": "100.00000000",
            "positionSide": "LONG",
            "updateTime": 1655217461579
        }"#;

        let position_risk: PositionRisk =
            serde_json::from_str(json).expect("deserialization failed");
        assert_eq!(position_risk.leverage, "125");
        assert_eq!(position_risk.liquidation_price, "0.49200");
        assert_eq!(position_risk.break_even_price, "0.50000");
    }
}
