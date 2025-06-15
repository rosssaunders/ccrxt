//! Integration tests for Deribit public endpoints
//!
//! These tests verify the public API client functionality

#[cfg(test)]
mod tests {
    use crate::deribit::{
        AccountTier, EndpointType, GetComboDetailsRequest, PublicRestClient, RateLimiter,
    };
    use std::sync::Arc;

    // Helper function to create a test client
    fn create_test_client() -> PublicRestClient {
        let client = reqwest::Client::new();
        let rate_limiter = Arc::new(RateLimiter::new(AccountTier::Tier4));

        PublicRestClient::new("https://test.deribit.com", client, rate_limiter)
    }

    #[test]
    fn test_client_initialization() {
        let client = create_test_client();

        // Verify client properties
        assert_eq!(client.base_url, "https://test.deribit.com");
        // Client is properly initialized and usable
        assert!(format!("{:?}", client).contains("RestClient"));
    }

    #[test]
    fn test_rate_limiter_functionality() {
        let rate_limiter = RateLimiter::new(AccountTier::Tier3);

        // Test that we can check limits for different endpoint types
        tokio_test::block_on(async {
            // These should succeed as we haven't made any requests yet
            assert!(rate_limiter
                .check_limits(EndpointType::NonMatchingEngine)
                .await
                .is_ok());
            assert!(rate_limiter
                .check_limits(EndpointType::MatchingEngine)
                .await
                .is_ok());
            assert!(rate_limiter
                .check_limits(EndpointType::PublicGetInstruments)
                .await
                .is_ok());
        });
    }

    #[test]
    fn test_endpoint_type_classification() {
        // Test that get_combo_details is classified as NonMatchingEngine
        let endpoint_type = EndpointType::from_path("public/get_combo_details");
        assert_eq!(endpoint_type, EndpointType::NonMatchingEngine);

        // Test other public endpoints for consistency
        let instruments_type = EndpointType::from_path("public/get_instruments");
        assert_eq!(instruments_type, EndpointType::PublicGetInstruments);

        // Test private/trading endpoints
        let buy_type = EndpointType::from_path("private/buy");
        assert_eq!(buy_type, EndpointType::MatchingEngine);
    }

    #[test]
    fn test_request_parameters_serialization() {
        // Test get_combo_details request
        let combo_request = GetComboDetailsRequest {
            combo_id: "COMBO-TEST-123456".to_string(),
        };
        let json_value = serde_json::to_value(combo_request).unwrap();
        assert_eq!(json_value.get("combo_id").unwrap(), "COMBO-TEST-123456");
    }

    #[test]
    fn test_combo_states() {
        use crate::deribit::ComboState;

        // Test all combo states can be serialized/deserialized
        let states = vec![ComboState::Active, ComboState::Rfq, ComboState::Inactive];

        for state in states {
            let serialized = serde_json::to_value(&state).unwrap();
            let deserialized: ComboState = serde_json::from_value(serialized).unwrap();
            assert_eq!(state, deserialized);
        }
    }

    #[test]
    fn test_error_handling() {
        use crate::deribit::{ApiError, ErrorResponse, Errors};

        // Test error response deserialization
        let error_json = serde_json::json!({
            "jsonrpc": "2.0",
            "id": null,
            "error": {
                "code": 10004,
                "message": "combo_not_found",
                "data": null
            }
        });

        let error_response: ErrorResponse = serde_json::from_value(error_json).unwrap();
        assert_eq!(error_response.jsonrpc, "2.0");
        assert_eq!(error_response.error.code, 10004);
        assert_eq!(error_response.error.message, "combo_not_found");

        // Test error conversion
        let api_error = ApiError {
            code: 10004,
            message: "combo_not_found".to_string(),
            data: None,
        };
        let error = Errors::ApiError(api_error);
        assert!(error.to_string().contains("10004"));
        assert!(error.to_string().contains("combo_not_found"));
    }

    #[test]
    fn test_account_tier_properties() {
        // Test that account tiers have correct rate limits
        assert_eq!(AccountTier::Tier1.sustained_rate(), 30);
        assert_eq!(AccountTier::Tier2.sustained_rate(), 20);
        assert_eq!(AccountTier::Tier3.sustained_rate(), 10);
        assert_eq!(AccountTier::Tier4.sustained_rate(), 5);

        assert_eq!(AccountTier::Tier1.burst_limit(), 100);
        assert_eq!(AccountTier::Tier2.burst_limit(), 50);
        assert_eq!(AccountTier::Tier3.burst_limit(), 30);
        assert_eq!(AccountTier::Tier4.burst_limit(), 20);
    }

    #[test]
    fn test_endpoint_credit_costs() {
        // Test credit costs for different endpoint types
        assert_eq!(EndpointType::NonMatchingEngine.credit_cost(), 500);
        assert_eq!(EndpointType::MatchingEngine.credit_cost(), 0);
        assert_eq!(EndpointType::PublicGetInstruments.credit_cost(), 0);
    }

    #[test]
    fn test_combo_info_complete_structure() {
        use crate::deribit::{ComboInfo, ComboState};

        let combo_json = serde_json::json!({
            "creation_timestamp": 1610905028000i64,
            "id": "COMBO-INTEGRATION-TEST",
            "instrument_id": 123456,
            "legs": [
                {
                    "amount": 1,
                    "instrument_name": "BTC-PERPETUAL"
                },
                {
                    "amount": -1,
                    "instrument_name": "ETH-PERPETUAL"
                }
            ],
            "state": "active",
            "state_timestamp": 1610905028001i64
        });

        let combo_info: ComboInfo = serde_json::from_value(combo_json).unwrap();

        assert_eq!(combo_info.id, "COMBO-INTEGRATION-TEST");
        assert_eq!(combo_info.instrument_id, 123456);
        assert_eq!(combo_info.legs.len(), 2);
        assert_eq!(combo_info.state, ComboState::Active);

        // Test leg details
        let btc_leg = &combo_info.legs[0];
        let eth_leg = &combo_info.legs[1];

        assert_eq!(btc_leg.amount, 1);
        assert_eq!(btc_leg.instrument_name, "BTC-PERPETUAL");
        assert_eq!(eth_leg.amount, -1);
        assert_eq!(eth_leg.instrument_name, "ETH-PERPETUAL");
    }
}