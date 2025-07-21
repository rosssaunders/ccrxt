use crate::binance::usdm::RestResult;
use serde::{Deserialize, Serialize};

use super::UsdmClient;

const LEVERAGE_BRACKET_ENDPOINT: &str = "/fapi/v1/leverageBracket";

/// Request parameters for getting leverage bracket.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLeverageBracketRequest {
    /// Trading symbol (optional for getting all symbols' brackets).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Receive window (optional timeout for request).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Individual leverage bracket data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeverageBracket {
    /// Bracket level (1-based index).
    pub bracket: u32,

    /// Initial leverage for this bracket.
    pub initial_leverage: u32,

    /// Notional cap (maximum notional value for this bracket).
    pub notional_cap: String,

    /// Notional floor (minimum notional value for this bracket).
    pub notional_floor: String,

    /// Maintenance margin ratio for this bracket.
    pub maint_margin_ratio: String,

    /// Cumulative maintenance margin amount.
    pub cum: String,
}

/// Response wrapper for leverage bracket data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeverageBracketResponse {
    /// Trading symbol for these brackets.
    pub symbol: String,

    /// List of leverage brackets for this symbol.
    pub brackets: Vec<LeverageBracket>,
}

impl UsdmClient {
    /// Notional and Leverage Brackets (USER_DATA)
    ///
    /// Get notional and leverage bracket for symbols.
    /// The weight for this request is 1 for a single symbol, 40 when symbol is omitted.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_leverage_bracket_request_serialization() {
        let request = GetLeverageBracketRequest {
            symbol: Some("BTCUSDT".to_string()),
            recv_window: Some(5000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("recvWindow=5000"));
    }

    #[test]
    fn test_get_leverage_bracket_request_default() {
        let request = GetLeverageBracketRequest::default();
        assert!(request.symbol.is_none());
        assert!(request.recv_window.is_none());
    }

    #[test]
    fn test_leverage_bracket_response_deserialization() {
        let json = r#"
        [
            {
                "symbol": "BTCUSDT",
                "brackets": [
                    {
                        "bracket": 1,
                        "initialLeverage": 125,
                        "notionalCap": "50000",
                        "notionalFloor": "0",
                        "maintMarginRatio": "0.004",
                        "cum": "0.0"
                    },
                    {
                        "bracket": 2,
                        "initialLeverage": 100,
                        "notionalCap": "250000",
                        "notionalFloor": "50000",
                        "maintMarginRatio": "0.005",
                        "cum": "50.0"
                    }
                ]
            }
        ]
        "#;

        let response: Vec<LeverageBracketResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        assert_eq!(response[0].symbol, "BTCUSDT");
        assert_eq!(response[0].brackets.len(), 2);

        let bracket1 = &response[0].brackets[0];
        assert_eq!(bracket1.bracket, 1);
        assert_eq!(bracket1.initial_leverage, 125);
        assert_eq!(bracket1.notional_cap, "50000");
        assert_eq!(bracket1.notional_floor, "0");
        assert_eq!(bracket1.maint_margin_ratio, "0.004");
        assert_eq!(bracket1.cum, "0.0");

        let bracket2 = &response[0].brackets[1];
        assert_eq!(bracket2.bracket, 2);
        assert_eq!(bracket2.initial_leverage, 100);
        assert_eq!(bracket2.notional_cap, "250000");
        assert_eq!(bracket2.notional_floor, "50000");
        assert_eq!(bracket2.maint_margin_ratio, "0.005");
        assert_eq!(bracket2.cum, "50.0");
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
                        "notionalCap": "50000",
                        "notionalFloor": "0",
                        "maintMarginRatio": "0.004",
                        "cum": "0.0"
                    }
                ]
            },
            {
                "symbol": "ETHUSDT",
                "brackets": [
                    {
                        "bracket": 1,
                        "initialLeverage": 100,
                        "notionalCap": "10000",
                        "notionalFloor": "0",
                        "maintMarginRatio": "0.005",
                        "cum": "0.0"
                    }
                ]
            }
        ]
        "#;

        let response: Vec<LeverageBracketResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 2);
        assert_eq!(response[0].symbol, "BTCUSDT");
        assert_eq!(response[1].symbol, "ETHUSDT");
        assert_eq!(response[0].brackets[0].initial_leverage, 125);
        assert_eq!(response[1].brackets[0].initial_leverage, 100);
    }
}
