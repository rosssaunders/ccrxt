#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_leverage_bracket_request_serialization() {
        let req = GetLeverageBracketRequest {
            symbol: Some("BTCUSDT".to_string()),
            recv_window: Some(5000),
            timestamp: Some(1234567890),
        };
        let serialized = serde_urlencoded::to_string(&req).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1234567890"));
    }

    #[test]
    fn test_get_leverage_bracket_request_default() {
        let req = GetLeverageBracketRequest::default();
        assert!(req.symbol.is_none());
        assert!(req.recv_window.is_none());
        assert!(req.timestamp.is_none());
    }

    #[test]
    fn test_leverage_bracket_response_deserialization() {
        let json = r#"
        [
            {
                "symbol": "BTCUSDT",
                "notionalCoef": 1.5,
                "brackets": [
                    {
                        "bracket": 1,
                        "initialLeverage": 125,
                        "notionalCap": 50000,
                        "notionalFloor": 0,
                        "maintMarginRatio": 0.004,
                        "cum": 0.0
                    },
                    {
                        "bracket": 2,
                        "initialLeverage": 100,
                        "notionalCap": 250000,
                        "notionalFloor": 50000,
                        "maintMarginRatio": 0.005,
                        "cum": 50.0
                    }
                ]
            }
        ]
        "#;
        let resp: Vec<LeverageBracketResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(resp.len(), 1);
        let r = &resp[0];
        assert_eq!(r.symbol, "BTCUSDT");
        assert_eq!(r.notional_coef, Some(1.5));
        assert_eq!(r.brackets.len(), 2);
        let b1 = &r.brackets[0];
        assert_eq!(b1.bracket, 1);
        assert_eq!(b1.initial_leverage, 125);
        assert_eq!(b1.notional_cap, 50000.0);
        assert_eq!(b1.notional_floor, 0.0);
        assert_eq!(b1.maint_margin_ratio, 0.004);
        assert_eq!(b1.cum, 0.0);
        let b2 = &r.brackets[1];
        assert_eq!(b2.bracket, 2);
        assert_eq!(b2.initial_leverage, 100);
        assert_eq!(b2.notional_cap, 250000.0);
        assert_eq!(b2.notional_floor, 50000.0);
        assert_eq!(b2.maint_margin_ratio, 0.005);
        assert_eq!(b2.cum, 50.0);
    }

    #[test]
    fn test_multiple_symbols_response_deserialization() {
        let json = r#"
        [
            {
                "symbol": "BTCUSDT",
                "brackets": [
                    {
                        "bracket": 1,
                        "initialLeverage": 125,
                        "notionalCap": 50000,
                        "notionalFloor": 0,
                        "maintMarginRatio": 0.004,
                        "cum": 0.0
                    }
                ]
            },
            {
                "symbol": "ETHUSDT",
                "brackets": [
                    {
                        "bracket": 1,
                        "initialLeverage": 100,
                        "notionalCap": 10000,
                        "notionalFloor": 0,
                        "maintMarginRatio": 0.005,
                        "cum": 0.0
                    }
                ]
            }
        ]
        "#;
        let resp: Vec<LeverageBracketResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(resp.len(), 2);
        assert_eq!(resp[0].symbol, "BTCUSDT");
        assert_eq!(resp[1].symbol, "ETHUSDT");
        assert_eq!(resp[0].brackets[0].initial_leverage, 125);
        assert_eq!(resp[1].brackets[0].initial_leverage, 100);
    }
}
use crate::binance::usdm::RestResult;
use serde::{Deserialize, Serialize};

use super::UsdmClient;

const LEVERAGE_BRACKET_ENDPOINT: &str = "/fapi/v1/leverageBracket";

/// Request parameters for the Notional and Leverage Brackets endpoint.
///
/// All fields are optional. See [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Notional-and-Leverage-Brackets
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLeverageBracketRequest {
    /// Trading symbol to query. Optional; if omitted, returns all symbols.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// The number of milliseconds after timestamp the request is valid for. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp (milliseconds since epoch). Required by API, but may be set automatically by client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,
}

/// Represents a single leverage bracket for a symbol.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeverageBracket {
    /// Bracket level (1-based index).
    pub bracket: u32,

    /// Maximum initial leverage for this bracket.
    pub initial_leverage: u32,

    /// Cap notional value for this bracket.
    pub notional_cap: f64,

    /// Notional threshold (minimum) for this bracket.
    pub notional_floor: f64,

    /// Maintenance margin ratio for this bracket.
    pub maint_margin_ratio: f64,

    /// Cumulative maintenance margin amount for this bracket.
    pub cum: f64,
}

/// Response for leverage bracket data for a symbol.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeverageBracketResponse {
    /// Trading symbol for these brackets.
    pub symbol: String,

    /// Notional coefficient for the symbol. Only present if user's symbol bracket is adjusted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notional_coef: Option<f64>,

    /// List of leverage brackets for this symbol.
    pub brackets: Vec<LeverageBracket>,
}

impl UsdmClient {
    /// Notional and Leverage Brackets (USER_DATA)
    ///
    /// Get notional and leverage bracket for symbols.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Notional-and-Leverage-Brackets
    ///
    /// Rate limit: 1 weight (with symbol), 40 weight (without symbol)
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`GetLeverageBracketRequest`])
    ///
    /// # Returns
    /// A list of [`LeverageBracketResponse`] objects with bracket information.
    pub async fn get_leverage_bracket(
        &self,
        params: GetLeverageBracketRequest,
    ) -> RestResult<Vec<LeverageBracketResponse>> {
        let weight = if params.symbol.is_some() { 1 } else { 40 };
        self.send_signed_request(
            LEVERAGE_BRACKET_ENDPOINT,
            reqwest::Method::GET,
            params,
            weight,
            false,
        )
        .await
    }
}
