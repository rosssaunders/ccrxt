//! Request and response structs for public/get_combos endpoint
//!
//! Retrieves information about active combos

use super::client::RestClient;
use crate::deribit::{ComboState, Currency, EndpointType, RestResult};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Request parameters for the public/get_combos endpoint.
///
/// Retrieves information about active combos.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCombosRequest {
    /// The currency symbol or "any" for all. Required.
    #[serde(rename = "currency")]
    pub currency: Currency,
}

/// Response for public/get_combos endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct GetCombosResponse {
    /// The id that was sent in the request
    #[serde(rename = "id")]
    pub id: i32,

    /// The JSON-RPC version (2.0)
    #[serde(rename = "jsonrpc")]
    pub jsonrpc: String,

    /// Array of combo objects
    #[serde(rename = "result")]
    pub result: Vec<Combo>,
}

/// Combo object containing information about a trading combo
#[derive(Debug, Clone, Deserialize)]
pub struct Combo {
    /// The timestamp (milliseconds since the Unix epoch)
    #[serde(rename = "creation_timestamp")]
    pub creation_timestamp: i64,

    /// Unique combo identifier
    #[serde(rename = "id")]
    pub id: String,

    /// Instrument ID
    #[serde(rename = "instrument_id")]
    pub instrument_id: i32,

    /// Array of legs that make up the combo
    #[serde(rename = "legs")]
    pub legs: Vec<ComboLeg>,

    /// Combo state: "rfq", "active", "inactive"
    #[serde(rename = "state")]
    pub state: ComboState,

    /// The timestamp (milliseconds since the Unix epoch)
    #[serde(rename = "state_timestamp")]
    pub state_timestamp: i64,
}

/// A leg of a combo trade
#[derive(Debug, Clone, Deserialize)]
pub struct ComboLeg {
    /// Size multiplier of a leg. A negative value indicates that the trades on given leg 
    /// are in opposite direction to the combo trades they originate from
    #[serde(rename = "amount")]
    pub amount: i32,

    /// Unique instrument identifier
    #[serde(rename = "instrument_name")]
    pub instrument_name: Cow<'static, str>,
}

impl RestClient {
    /// Calls the public/get_combos endpoint.
    ///
    /// Retrieves information about active combos.
    ///
    /// [Official API docs](https://docs.deribit.com/v2/#public-get_combos)
    pub async fn get_combos(
        &self,
        request: GetCombosRequest,
    ) -> RestResult<GetCombosResponse> {
        self.send_request(
            "public/get_combos",
            reqwest::Method::POST,
            Some(&request),
            EndpointType::PublicGetCombos,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_get_combos_request_serialization() {
        let request = GetCombosRequest {
            currency: Currency::Btc,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("currency").unwrap(), "BTC");
    }

    #[test]
    fn test_get_combos_request_any_currency() {
        let request = GetCombosRequest {
            currency: Currency::Any,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("currency").unwrap(), "any");
    }

    #[test]
    fn test_get_combos_response_deserialization() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": [
                {
                    "creation_timestamp": 1640995200000_i64,
                    "id": "combo_123",
                    "instrument_id": 456,
                    "legs": [
                        {
                            "amount": 1,
                            "instrument_name": "BTC-PERPETUAL"
                        },
                        {
                            "amount": -1,
                            "instrument_name": "BTC-25DEC21"
                        }
                    ],
                    "state": "active",
                    "state_timestamp": 1640995200000_i64
                }
            ]
        });

        let response: GetCombosResponse = serde_json::from_value(response_json).unwrap();
        
        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.len(), 1);
        
        let combo = &response.result[0];
        assert_eq!(combo.id, "combo_123");
        assert_eq!(combo.instrument_id, 456);
        assert_eq!(combo.state, ComboState::Active);
        assert_eq!(combo.legs.len(), 2);
        assert_eq!(combo.legs[0].amount, 1);
        assert_eq!(combo.legs[0].instrument_name, "BTC-PERPETUAL");
        assert_eq!(combo.legs[1].amount, -1);
        assert_eq!(combo.legs[1].instrument_name, "BTC-25DEC21");
    }

    #[test]
    fn test_combo_state_deserialization() {
        let test_cases = vec![
            ("\"rfq\"", ComboState::Rfq),
            ("\"active\"", ComboState::Active),
            ("\"inactive\"", ComboState::Inactive),
        ];

        for (json_str, expected_state) in test_cases {
            let state: ComboState = serde_json::from_str(json_str).unwrap();
            assert_eq!(state, expected_state);
        }
    }
}