//! Request and response structs for public/get_combo_details endpoint
//!
//! Retrieves information about a combo

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use super::get_combos::ComboInfo;
use crate::deribit::{EndpointType, RestResult};

/// Request parameters for the public/get_combo_details endpoint.
///
/// Retrieves information about a combo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetComboDetailsRequest {
    /// Combo ID
    pub combo_id: String,
}

/// Response for public/get_combo_details endpoint following Deribit JSON-RPC 2.0 format.
#[derive(Debug, Clone, Deserialize)]
pub struct GetComboDetailsResponse {
    /// The id that was sent in the request
    pub id: i64,

    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,

    /// Combo information
    pub result: ComboInfo,
}

impl RestClient {
    /// Calls the public/get_combo_details endpoint.
    ///
    /// Retrieves information about a combo
    ///
    /// # Arguments
    /// * `params` - The request parameters including combo_id
    ///
    /// # Returns
    /// A result containing the response with combo information or an error
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_combo_details)
    pub async fn get_combo_details(&self, params: GetComboDetailsRequest) -> RestResult<GetComboDetailsResponse> {
        self.send_request(
            "public/get_combo_details",
            reqwest::Method::GET,
            Some(&params),
            EndpointType::PublicGetComboDetails,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::deribit::{AccountTier, RateLimiter};

    #[test]
    fn test_get_combo_details_request_serialization() {
        let request = GetComboDetailsRequest {
            combo_id: "BTC-28JUN24-65000-C_BTC-28JUN24-70000-P".to_string(),
        };

        let json_value = serde_json::to_value(&request).unwrap();
        assert_eq!(json_value["combo_id"], "BTC-28JUN24-65000-C_BTC-28JUN24-70000-P");
    }

    #[test]
    fn test_get_combo_details_response_structure() {
        let response_json = json!({
            "id": 123,
            "jsonrpc": "2.0",
            "result": {
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
        });

        let response: GetComboDetailsResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.id, 123);
        assert_eq!(response.jsonrpc, "2.0");
        
        let combo = &response.result;
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
    fn test_combo_details_response_with_different_states() {
        for state in ["rfq", "active", "inactive"] {
            let response_json = json!({
                "id": 456,
                "jsonrpc": "2.0",
                "result": {
                    "creation_timestamp": 1640995200000i64,
                    "id": "ETH-28JUN24-3000-C_ETH-28JUN24-3500-P",
                    "instrument_id": 789012,
                    "legs": [
                        {
                            "amount": 2,
                            "instrument_name": "ETH-28JUN24-3000-C"
                        }
                    ],
                    "state": state,
                    "state_timestamp": 1640995300000i64
                }
            });

            let response: GetComboDetailsResponse = serde_json::from_value(response_json).unwrap();
            assert_eq!(response.result.state, state);
        }
    }

    #[tokio::test]
    async fn test_endpoint_type_usage() {
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new("https://test.deribit.com", client, rate_limiter);

        // Test that we can create a request - this doesn't actually call the API
        let _request = GetComboDetailsRequest {
            combo_id: "BTC-28JUN24-65000-C_BTC-28JUN24-70000-P".to_string(),
        };

        // Test that rate limiting works for this endpoint type
        let result = rest_client
            .rate_limiter
            .check_limits(EndpointType::PublicGetComboDetails)
            .await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_request_with_different_combo_ids() {
        let combo_ids = [
            "BTC-28JUN24-65000-C_BTC-28JUN24-70000-P",
            "ETH-28JUN24-3000-C_ETH-28JUN24-3500-P",
            "SOL-28JUN24-100-C_SOL-28JUN24-150-P",
        ];

        for combo_id in combo_ids {
            let request = GetComboDetailsRequest {
                combo_id: combo_id.to_string(),
            };

            let json_value = serde_json::to_value(&request).unwrap();
            assert_eq!(json_value["combo_id"], combo_id);
        }
    }
}