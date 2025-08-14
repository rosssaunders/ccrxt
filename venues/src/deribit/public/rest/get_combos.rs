//! Request and response structs for public/get_combos endpoint
//!
//! Retrieves information about active combos

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult, enums::Currency};

const COMBOS_ENDPOINT: &str = "public/get_combos";

/// Request parameters for the public/get_combos endpoint.
///
/// Retrieves information about active combos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCombosRequest {
    /// The currency symbol or "any" for all
    pub currency: Currency,
}

/// A leg in a combo
#[derive(Debug, Clone, Deserialize)]
pub struct ComboLeg {
    /// Size multiplier of a leg. A negative value indicates that the trades on given leg
    /// are in opposite direction to the combo trades they originate from
    pub amount: i32,

    /// Unique instrument identifier
    pub instrument_name: String,
}

/// Combo information
#[derive(Debug, Clone, Deserialize)]
pub struct ComboInfo {
    /// The timestamp (milliseconds since the Unix epoch)
    pub creation_timestamp: i64,

    /// Unique combo identifier
    pub id: String,

    /// Instrument ID
    pub instrument_id: i32,

    /// Legs of the combo
    pub legs: Vec<ComboLeg>,

    /// Combo state: "rfq", "active", "inactive"
    pub state: String,

    /// The timestamp (milliseconds since the Unix epoch)
    pub state_timestamp: i64,
}

/// Response for public/get_combo_ids endpoint following Deribit JSON-RPC 2.0 format.
pub type GetCombosResponse = JsonRpcResult<Vec<ComboInfo>>;

impl RestClient {
    /// Calls the public/get_combos endpoint.
    ///
    /// Retrieves information about active combos
    ///
    /// # Arguments
    /// * `params` - The request parameters including currency
    ///
    /// # Returns
    /// A result containing the response with combo information or an error
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_combos)
    pub async fn get_combos(&self, params: GetCombosRequest) -> RestResult<GetCombosResponse> {
        self.send_post_request(
            COMBOS_ENDPOINT,
            Some(&params),
            EndpointType::PublicGetCombos,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use serde_json::json;

    use super::*;
    use crate::deribit::{AccountTier, RateLimiter};

    #[test]
    fn test_get_combos_request_serialization() {
        let request = GetCombosRequest {
            currency: Currency::BTC,
        };

        let json_value = serde_json::to_value(&request).unwrap();
        assert_eq!(json_value["currency"], "BTC");
    }

    #[test]
    fn test_get_combos_request_with_any_currency() {
        let request = GetCombosRequest {
            currency: Currency::Any,
        };

        let json_value = serde_json::to_value(&request).unwrap();
        assert_eq!(json_value["currency"], "any");
    }

    #[test]
    fn test_get_combos_response_structure() {
        let response_json = json!({
            "id": 123,
            "jsonrpc": "2.0",
            "result": [
                {
                    "creation_timestamp": 1640995200000i64,
                    "id": "BTC-28JUN24-65000-C_BTC-28JUN24-70000-P",
                    "instrument_id": 123456,
                    "legs": [
                        {
                            "amount": 1,
                            "instrument_name": "BTC-28JUN24-65000-C"
                        },
                        {
                            "amount": -1,
                            "instrument_name": "BTC-28JUN24-70000-P"
                        }
                    ],
                    "state": "active",
                    "state_timestamp": 1640995200000i64
                }
            ]
        });

        let response: GetCombosResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.id, 123);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.len(), 1);

        let combo = &response.result[0];
        assert_eq!(combo.creation_timestamp, 1640995200000);
        assert_eq!(combo.id, "BTC-28JUN24-65000-C_BTC-28JUN24-70000-P");
        assert_eq!(combo.instrument_id, 123456);
        assert_eq!(combo.state, "active");
        assert_eq!(combo.state_timestamp, 1640995200000);
        assert_eq!(combo.legs.len(), 2);

        let leg1 = &combo.legs[0];
        assert_eq!(leg1.amount, 1);
        assert_eq!(leg1.instrument_name, "BTC-28JUN24-65000-C");

        let leg2 = &combo.legs[1];
        assert_eq!(leg2.amount, -1);
        assert_eq!(leg2.instrument_name, "BTC-28JUN24-70000-P");
    }

    #[test]
    fn test_all_supported_currencies() {
        for currency in [
            Currency::BTC,
            Currency::ETH,
            Currency::USDC,
            Currency::USDT,
            Currency::EURR,
            Currency::Any,
        ] {
            let request = GetCombosRequest { currency };

            let json_value = serde_json::to_value(&request).unwrap();
            assert!(json_value["currency"].is_string());
        }
    }

    #[tokio::test]
    async fn test_endpoint_type_usage() {
        let http_client = Arc::new(rest::native::NativeHttpClient::default());
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new("https://test.deribit.com", http_client, rate_limiter);

        // Test that we can create a request - this doesn't actually call the API
        let _request = GetCombosRequest {
            currency: Currency::BTC,
        };

        // Test that rate limiting works for this endpoint type
        let result = rest_client
            .rate_limiter
            .check_limits(EndpointType::NonMatchingEngine)
            .await;
        assert!(result.is_ok());
    }
}
