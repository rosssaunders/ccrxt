//! Position risk endpoints for Binance USDM REST API.
use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::RestResult;
use crate::binance::usdm::enums::*;

const POSITION_RISK_ENDPOINT: &str = "/fapi/v2/positionRisk";

/// Request parameters for the position risk endpoint.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionRiskRequest {
    /// Trading symbol (e.g., "BTCUSDT"). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<std::borrow::Cow<'static, str>>,
}

/// Position risk information for a symbol.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionRisk {
    /// Entry price for the position.
    pub entry_price: std::borrow::Cow<'static, str>,

    /// Margin type (crossed or isolated).
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

    /// Trading symbol.
    pub symbol: std::borrow::Cow<'static, str>,

    /// Unrealized profit for the position.
    pub un_realized_profit: std::borrow::Cow<'static, str>,

    /// Position side (long, short, both).
    pub position_side: PositionSide,
}

impl UsdmClient {
    /// Position Information V2
    ///
    /// Retrieves position risk information for a symbol or all positions on Binance USDM Futures.
    ///
    /// [docs]: https://binance-docs.github.io/apidocs/futures/en/#position-information-v2-user_data
    ///
    /// Rate limit: 2 requests per second
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
        self.send_signed_request(
            POSITION_RISK_ENDPOINT,
            reqwest::Method::GET,
            request,
            2,
            false,
        )
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
        };

        // The serialization should exclude api_key and api_secret
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=BTCUSDT");
        assert!(!serialized.contains("api_key"));
        assert!(!serialized.contains("api_secret"));
    }

    #[test]
    fn test_get_position_risk_request_no_symbol() {
        let request = GetPositionRiskRequest { symbol: None };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_position_risk_deserialization() {
        let json = r#"{
            "entryPrice": "45380.10",
            "marginType": "CROSSED",
            "isAutoAddMargin": true,
            "isolatedMargin": "0.00000000",
            "leverage": "20",
            "liquidationPrice": "43111.095",
            "markPrice": "45390.20",
            "maxNotionalValue": "500000",
            "positionAmt": "0.100",
            "symbol": "BTCUSDT",
            "unRealizedProfit": "1.01000000",
            "positionSide": "BOTH"
        }"#;

        let position_risk: PositionRisk = serde_json::from_str(json).unwrap();
        assert_eq!(position_risk.entry_price, "45380.10");
        assert!(matches!(position_risk.margin_type, MarginType::Cross));
        assert!(position_risk.is_auto_add_margin);
        assert_eq!(position_risk.isolated_margin, "0.00000000");
        assert_eq!(position_risk.leverage, "20");
        assert_eq!(position_risk.liquidation_price, "43111.095");
        assert_eq!(position_risk.mark_price, "45390.20");
        assert_eq!(position_risk.max_notional_value, "500000");
        assert_eq!(position_risk.position_amt, "0.100");
        assert_eq!(position_risk.symbol, "BTCUSDT");
        assert_eq!(position_risk.un_realized_profit, "1.01000000");
        assert!(matches!(position_risk.position_side, PositionSide::Both));
    }

    #[test]
    fn test_position_risk_isolated_margin() {
        let json = r#"{
            "entryPrice": "3070.50",
            "marginType": "ISOLATED",
            "isAutoAddMargin": false,
            "isolatedMargin": "100.00000000",
            "leverage": "10",
            "liquidationPrice": "2800.00",
            "markPrice": "3080.60",
            "maxNotionalValue": "100000",
            "positionAmt": "1.000",
            "symbol": "ETHUSDT",
            "unRealizedProfit": "10.10000000",
            "positionSide": "LONG"
        }"#;

        let position_risk: PositionRisk = serde_json::from_str(json).unwrap();
        assert!(matches!(position_risk.margin_type, MarginType::Isolated));
        assert!(!position_risk.is_auto_add_margin);
        assert_eq!(position_risk.isolated_margin, "100.00000000");
        assert!(matches!(position_risk.position_side, PositionSide::Long));
    }

    #[test]
    fn test_position_risk_short_position() {
        let json = r#"{
            "entryPrice": "45400.00",
            "marginType": "CROSSED",
            "isAutoAddMargin": true,
            "isolatedMargin": "0.00000000",
            "leverage": "5",
            "liquidationPrice": "54480.00",
            "markPrice": "45350.00",
            "maxNotionalValue": "1000000",
            "positionAmt": "-0.500",
            "symbol": "BTCUSDT",
            "unRealizedProfit": "25.00000000",
            "positionSide": "SHORT"
        }"#;

        let position_risk: PositionRisk = serde_json::from_str(json).unwrap();
        assert_eq!(position_risk.position_amt, "-0.500");
        assert!(matches!(position_risk.position_side, PositionSide::Short));
        assert_eq!(position_risk.un_realized_profit, "25.00000000");
    }

    #[test]
    fn test_position_risk_array_deserialization() {
        let json = r#"[
            {
                "entryPrice": "45380.10",
                "marginType": "CROSSED",
                "isAutoAddMargin": true,
                "isolatedMargin": "0.00000000",
                "leverage": "20",
                "liquidationPrice": "43111.095",
                "markPrice": "45390.20",
                "maxNotionalValue": "500000",
                "positionAmt": "0.100",
                "symbol": "BTCUSDT",
                "unRealizedProfit": "1.01000000",
                "positionSide": "BOTH"
            },
            {
                "entryPrice": "3070.50",
                "marginType": "CROSSED",
                "isAutoAddMargin": true,
                "isolatedMargin": "0.00000000",
                "leverage": "10",
                "liquidationPrice": "2763.45",
                "markPrice": "3080.60",
                "maxNotionalValue": "100000",
                "positionAmt": "1.000",
                "symbol": "ETHUSDT",
                "unRealizedProfit": "10.10000000",
                "positionSide": "BOTH"
            }
        ]"#;

        let positions: Vec<PositionRisk> = serde_json::from_str(json).unwrap();
        assert_eq!(positions.len(), 2);
        assert_eq!(positions[0].symbol, "BTCUSDT");
        assert_eq!(positions[1].symbol, "ETHUSDT");
    }

    #[test]
    fn test_position_risk_high_leverage() {
        let json = r#"{
            "entryPrice": "0.50000",
            "marginType": "ISOLATED",
            "isAutoAddMargin": false,
            "isolatedMargin": "10.00000000",
            "leverage": "125",
            "liquidationPrice": "0.49200",
            "markPrice": "0.50100",
            "maxNotionalValue": "50000",
            "positionAmt": "1000.000",
            "symbol": "DOGEUSDT",
            "unRealizedProfit": "100.00000000",
            "positionSide": "LONG"
        }"#;

        let position_risk: PositionRisk = serde_json::from_str(json).unwrap();
        assert_eq!(position_risk.leverage, "125");
        assert_eq!(position_risk.liquidation_price, "0.49200");
    }
}
