//! Request and response structs for public/get_combo_details endpoint
//!
//! Retrieves information about a combo

use super::client::RestClient;
use crate::deribit::{EndpointType, RestResult, ComboInfo};
use serde::{Deserialize, Serialize};

/// Request parameters for the public/get_combo_details endpoint.
///
/// Retrieves information about a combo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetComboDetailsRequest {
    /// Combo ID. Required.
    pub combo_id: String,
}

/// Response for public/get_combo_details endpoint following JSON-RPC 2.0 format.
#[derive(Debug, Clone, Deserialize)]
pub struct GetComboDetailsResponse {
    /// The id that was sent in the request
    pub id: i64,
    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,
    /// Result data containing combo details
    pub result: ComboInfo,
}

impl RestClient {
    /// Calls the public/get_combo_details endpoint.
    ///
    /// Retrieves information about a combo
    ///
    /// # Arguments
    /// * `params` - The request parameters containing the combo_id
    ///
    /// # Returns
    /// Response containing combo details
    ///
    /// [Official API docs](https://docs.deribit.com/)
    pub async fn get_combo_details(
        &self,
        params: GetComboDetailsRequest,
    ) -> RestResult<GetComboDetailsResponse> {
        self.send_request(
            "public/get_combo_details",
            reqwest::Method::GET,
            Some(&params),
            EndpointType::NonMatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_get_combo_details_request_structure() {
        let request = GetComboDetailsRequest {
            combo_id: "COMBO-123456".to_string(),
        };

        let json_value = serde_json::to_value(&request).expect("Failed to serialize request");
        assert_eq!(json_value.get("combo_id").unwrap(), "COMBO-123456");
    }

    #[test]
    fn test_get_combo_details_request_serialization() {
        let request = GetComboDetailsRequest {
            combo_id: "COMBO-789012".to_string(),
        };

        let serialized = serde_json::to_string(&request).expect("Failed to serialize request");
        let deserialized: GetComboDetailsRequest = 
            serde_json::from_str(&serialized).expect("Failed to deserialize request");
        
        assert_eq!(request.combo_id, deserialized.combo_id);
    }

    #[test]
    fn test_get_combo_details_response_structure() {
        let response_json = json!({
            "id": 123,
            "jsonrpc": "2.0",
            "result": {
                "creation_timestamp": 1610905028000i64,
                "id": "COMBO-123456",
                "instrument_id": 78901,
                "legs": [
                    {
                        "amount": 1,
                        "instrument_name": "BTC-PERPETUAL"
                    },
                    {
                        "amount": -2,
                        "instrument_name": "ETH-PERPETUAL"
                    }
                ],
                "state": "active",
                "state_timestamp": 1610905028001i64
            }
        });

        let response: GetComboDetailsResponse = serde_json::from_value(response_json)
            .expect("Failed to deserialize response");

        assert_eq!(response.id, 123);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.id, "COMBO-123456");
        assert_eq!(response.result.instrument_id, 78901);
        assert_eq!(response.result.legs.len(), 2);
        assert_eq!(response.result.legs[0].amount, 1);
        assert_eq!(response.result.legs[0].instrument_name, "BTC-PERPETUAL");
        assert_eq!(response.result.legs[1].amount, -2);
        assert_eq!(response.result.legs[1].instrument_name, "ETH-PERPETUAL");
        assert_eq!(response.result.state, crate::deribit::ComboState::Active);
    }

    #[test]
    fn test_combo_state_serialization() {
        use crate::deribit::ComboState;
        
        let active = ComboState::Active;
        let rfq = ComboState::Rfq;
        let inactive = ComboState::Inactive;

        assert_eq!(serde_json::to_string(&active).unwrap(), "\"active\"");
        assert_eq!(serde_json::to_string(&rfq).unwrap(), "\"rfq\"");
        assert_eq!(serde_json::to_string(&inactive).unwrap(), "\"inactive\"");

        // Test deserialization
        let active_deser: ComboState = serde_json::from_str("\"active\"").unwrap();
        let rfq_deser: ComboState = serde_json::from_str("\"rfq\"").unwrap();
        let inactive_deser: ComboState = serde_json::from_str("\"inactive\"").unwrap();

        assert_eq!(active_deser, ComboState::Active);
        assert_eq!(rfq_deser, ComboState::Rfq);
        assert_eq!(inactive_deser, ComboState::Inactive);
    }

    #[test]
    fn test_combo_leg_structure() {
        use crate::deribit::ComboLeg;

        let leg_json = json!({
            "amount": -5,
            "instrument_name": "BTC-25DEC20-20000-C"
        });

        let leg: ComboLeg = serde_json::from_value(leg_json)
            .expect("Failed to deserialize combo leg");

        assert_eq!(leg.amount, -5);
        assert_eq!(leg.instrument_name, "BTC-25DEC20-20000-C");
    }
}