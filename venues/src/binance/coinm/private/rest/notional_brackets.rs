use serde::{Deserialize, Serialize};

use crate::binance::{
    coinm::{RestResult, private::rest::client::RestClient},
    shared,
};

const LEVERAGE_BRACKET_ENDPOINT: &str = "/dapi/v1/leverageBracket";

/// Request parameters for notional and leverage brackets.
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotionalBracketRequest {
    /// Trading pair, e.g. BTCUSD_PERP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,

    /// This parameter cannot be used in combination with other parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Bracket information for notional brackets.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotionalBracket {
    /// Bracket
    pub bracket: u32,

    /// Initial leverage
    pub initial_leverage: u32,

    /// Max notional value of the bracket
    pub notional_cap: String,

    /// Min notional value of the bracket
    pub notional_floor: String,

    /// Maintenance margin ratio
    pub maint_margin_ratio: String,

    /// Auxiliary number for quick calculation
    pub cum: String,
}

/// Response for notional and leverage brackets.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotionalBracketResponse {
    /// Trading pair
    pub pair: String,

    /// Bracket information
    pub brackets: Vec<NotionalBracket>,
}

impl RestClient {
    /// Get notional and leverage brackets on Binance Coin-M Futures.
    ///
    /// See: <https://binance-docs.github.io/apidocs/delivery/en/>
    /// GET /dapi/v1/leverageBracket
    /// Weight: 1 if pair provided, 40 otherwise
    /// Requires API key and signature.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`NotionalBracketRequest`])
    ///
    /// # Returns
    /// A list of [`NotionalBracketResponse`] objects with bracket information.
    pub async fn get_notional_brackets(
        &self,
        params: NotionalBracketRequest,
    ) -> RestResult<Vec<NotionalBracketResponse>> {
        let weight = if params.pair.is_some() { 1 } else { 40 };
        shared::send_signed_request(
            self,
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
    fn test_notional_bracket_request_serialization() {
        let request = NotionalBracketRequest {
            pair: Some("BTCUSD_PERP".to_string()),
            recv_window: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "pair=BTCUSD_PERP");
    }

    #[test]
    fn test_notional_bracket_request_empty() {
        let request = NotionalBracketRequest {
            pair: None,
            recv_window: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_notional_bracket_request_with_recv_window() {
        let request = NotionalBracketRequest {
            pair: Some("ETHUSD_PERP".to_string()),
            recv_window: Some(5000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("pair=ETHUSD_PERP"));
        assert!(serialized.contains("recvWindow=5000"));
    }

    #[test]
    fn test_notional_bracket_response_deserialization() {
        let json = r#"[
            {
                "pair": "BTCUSD_PERP",
                "brackets": [
                    {
                        "bracket": 1,
                        "initialLeverage": 125,
                        "notionalCap": "50000",
                        "notionalFloor": "0",
                        "maintMarginRatio": "0.004",
                        "cum": "0"
                    },
                    {
                        "bracket": 2,
                        "initialLeverage": 100,
                        "notionalCap": "250000",
                        "notionalFloor": "50000",
                        "maintMarginRatio": "0.005",
                        "cum": "50"
                    }
                ]
            }
        ]"#;

        let response: Vec<NotionalBracketResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        assert_eq!(response[0].pair, "BTCUSD_PERP");
        assert_eq!(response[0].brackets.len(), 2);
        
        let bracket1 = &response[0].brackets[0];
        assert_eq!(bracket1.bracket, 1);
        assert_eq!(bracket1.initial_leverage, 125);
        assert_eq!(bracket1.notional_cap, "50000");
        assert_eq!(bracket1.notional_floor, "0");
        assert_eq!(bracket1.maint_margin_ratio, "0.004");
        assert_eq!(bracket1.cum, "0");
        
        let bracket2 = &response[0].brackets[1];
        assert_eq!(bracket2.bracket, 2);
        assert_eq!(bracket2.initial_leverage, 100);
        assert_eq!(bracket2.notional_cap, "250000");
        assert_eq!(bracket2.notional_floor, "50000");
        assert_eq!(bracket2.maint_margin_ratio, "0.005");
        assert_eq!(bracket2.cum, "50");
    }

    #[test]
    fn test_multiple_pairs_response() {
        let json = r#"[
            {
                "pair": "BTCUSD_PERP",
                "brackets": [
                    {
                        "bracket": 1,
                        "initialLeverage": 125,
                        "notionalCap": "50000",
                        "notionalFloor": "0",
                        "maintMarginRatio": "0.004",
                        "cum": "0"
                    }
                ]
            },
            {
                "pair": "ETHUSD_PERP",
                "brackets": [
                    {
                        "bracket": 1,
                        "initialLeverage": 100,
                        "notionalCap": "100000",
                        "notionalFloor": "0",
                        "maintMarginRatio": "0.005",
                        "cum": "0"
                    }
                ]
            }
        ]"#;

        let response: Vec<NotionalBracketResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 2);
        assert_eq!(response[0].pair, "BTCUSD_PERP");
        assert_eq!(response[1].pair, "ETHUSD_PERP");
        assert_eq!(response[0].brackets[0].initial_leverage, 125);
        assert_eq!(response[1].brackets[0].initial_leverage, 100);
    }
}
