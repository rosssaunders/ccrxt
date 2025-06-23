//! Request and response structs for public/get_combo_ids endpoint
//!
//! Retrieves available combos. This method can be used to get the list of all
//! combos, or only the list of combos in the given state.

use super::RestClient;
use crate::deribit::{ComboState, Currency, EndpointType, RestResult};

use serde::{Deserialize, Serialize};

/// Request parameters for the public/get_combo_ids endpoint.
///
/// Retrieves available combos. This method can be used to get the list of all
/// combos, or only the list of combos in the given state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetComboIdsRequest {
    /// The currency symbol
    pub currency: Currency,

    /// Combo state, if not provided combos of all states are considered
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ComboState>,
}

/// Response for public/get_combo_ids endpoint following Deribit JSON-RPC 2.0 format.
#[derive(Debug, Clone, Deserialize)]
pub struct GetComboIdsResponse {
    /// The id that was sent in the request
    pub id: i64,

    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,

    /// Array of unique combo identifiers
    pub result: Vec<String>,
}

impl RestClient {
    /// Calls the public/get_combo_ids endpoint.
    ///
    /// Retrieves available combos. This method can be used to get the list of all
    /// combos, or only the list of combos in the given state.
    ///
    /// # Arguments
    /// * `params` - The request parameters including currency and optional state
    ///
    /// # Returns
    /// A result containing the response with combo IDs or an error
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_combo_ids)
    pub async fn get_combo_ids(&self, params: GetComboIdsRequest) -> RestResult<GetComboIdsResponse> {
        self.send_request(
            "public/get_combo_ids",
            reqwest::Method::GET,
            Some(&params),
            EndpointType::NonMatchingEngine,
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
    fn test_get_combo_ids_request_serialization() {
        let request = GetComboIdsRequest {
            currency: Currency::BTC,
            state: Some(ComboState::Active),
        };

        let json_value = serde_json::to_value(&request).unwrap();
        assert_eq!(json_value["currency"], "BTC");
        assert_eq!(json_value["state"], "active");
    }

    #[test]
    fn test_get_combo_ids_request_without_state() {
        let request = GetComboIdsRequest {
            currency: Currency::ETH,
            state: None,
        };

        let json_value = serde_json::to_value(&request).unwrap();
        assert_eq!(json_value["currency"], "ETH");
        assert!(!json_value.as_object().unwrap().contains_key("state"));
    }

    #[test]
    fn test_get_combo_ids_response_structure() {
        let response_json = json!({
            "id": 123,
            "jsonrpc": "2.0",
            "result": ["BTC-28JUN24-65000-C_BTC-28JUN24-70000-P", "ETH-28JUN24-3000-C_ETH-28JUN24-3500-P"]
        });

        let response: GetComboIdsResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.id, 123);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.len(), 2);
        assert_eq!(
            response.result[0],
            "BTC-28JUN24-65000-C_BTC-28JUN24-70000-P"
        );
        assert_eq!(response.result[1], "ETH-28JUN24-3000-C_ETH-28JUN24-3500-P");
    }

    #[test]
    fn test_all_currencies() {
        for currency in [
            Currency::BTC,
            Currency::ETH,
            Currency::USDC,
            Currency::USDT,
            Currency::EURR,
        ] {
            let request = GetComboIdsRequest {
                currency,
                state: None,
            };

            let json_value = serde_json::to_value(&request).unwrap();
            assert!(json_value["currency"].is_string());
        }
    }

    #[test]
    fn test_all_states() {
        for state in [ComboState::RFQ, ComboState::Active, ComboState::Inactive] {
            let request = GetComboIdsRequest {
                currency: Currency::BTC,
                state: Some(state),
            };

            let json_value = serde_json::to_value(&request).unwrap();
            assert!(json_value["state"].is_string());
        }
    }

    #[tokio::test]
    async fn test_endpoint_type_usage() {
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new("https://test.deribit.com", client, rate_limiter);

        // Test that we can create a request - this doesn't actually call the API
        let _request = GetComboIdsRequest {
            currency: Currency::BTC,
            state: Some(ComboState::Active),
        };

        // Test that rate limiting works for this endpoint type
        let result = rest_client
            .rate_limiter
            .check_limits(EndpointType::NonMatchingEngine)
            .await;
        assert!(result.is_ok());
    }
}
