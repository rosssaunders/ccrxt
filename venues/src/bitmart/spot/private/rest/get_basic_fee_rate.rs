use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bitmart::{RestResult, rate_limit::EndpointType};

const BASIC_FEE_RATE_ENDPOINT: &str = "/spot/v1/user_fee";

/// Request parameters for getting basic fee rate (no parameters required)
#[derive(Debug, Serialize, Default)]
pub struct GetBasicFeeRateRequest {}

/// Response for basic fee rate endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBasicFeeRateResponse {
    /// Rate type
    /// - `0` = Normal Users
    /// - `1` = VIP Users
    /// - `2` = Special VIP Users
    pub user_rate_type: i64,
    /// User Level
    pub level: String,
    /// Taker fee rate for Class-A pairs
    pub taker_fee_rate_a: String,
    /// Maker fee rate for Class-A pairs
    pub maker_fee_rate_a: String,
    /// Taker fee rate for Class-B pairs
    pub taker_fee_rate_b: String,
    /// Maker fee rate for Class-B pairs
    pub maker_fee_rate_b: String,
    /// Taker fee rate for Class-C pairs
    pub taker_fee_rate_c: String,
    /// Maker fee rate for Class-C pairs
    pub maker_fee_rate_c: String,
    /// Taker fee rate for Class-D pairs
    pub taker_fee_rate_d: String,
    /// Maker fee rate for Class-D pairs
    pub maker_fee_rate_d: String,
}

impl RestClient {
    /// Get basic fee rate
    ///
    /// For querying the base rate of the current user
    ///
    /// See: https://raw.githubusercontent.com/rosssaunders/coincise/refs/heads/main/docs/bitmart/spot/funding_account.md
    ///
    /// Rate limit: 12 times/2 sec per API key
    ///
    /// # Arguments
    /// * `request` - The request parameters (empty struct)
    ///
    /// # Returns
    /// Basic fee rate information
    pub async fn get_basic_fee_rate(
        &self,
        request: GetBasicFeeRateRequest,
    ) -> RestResult<GetBasicFeeRateResponse> {
        self.send_request(
            BASIC_FEE_RATE_ENDPOINT,
            reqwest::Method::GET,
            Some(&request),
            EndpointType::FundingAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_basic_fee_rate_request_default() {
        let request = GetBasicFeeRateRequest::default();
        // This is an empty struct, just verify it can be created
        let _serialized = serde_json::to_string(&request).unwrap();
    }

    #[test]
    fn test_get_basic_fee_rate_response_structure() {
        let response = GetBasicFeeRateResponse {
            user_rate_type: 0,
            level: "LV1".to_string(),
            taker_fee_rate_a: "0.001".to_string(),
            maker_fee_rate_a: "0.001".to_string(),
            taker_fee_rate_b: "0.0025".to_string(),
            maker_fee_rate_b: "0.0025".to_string(),
            taker_fee_rate_c: "0.004".to_string(),
            maker_fee_rate_c: "0.004".to_string(),
            taker_fee_rate_d: "0.006".to_string(),
            maker_fee_rate_d: "0.006".to_string(),
        };

        assert_eq!(response.user_rate_type, 0);
        assert_eq!(response.level, "LV1");
        assert_eq!(response.taker_fee_rate_a, "0.001");
        assert_eq!(response.maker_fee_rate_a, "0.001");
        assert_eq!(response.taker_fee_rate_b, "0.0025");
        assert_eq!(response.maker_fee_rate_b, "0.0025");
        assert_eq!(response.taker_fee_rate_c, "0.004");
        assert_eq!(response.maker_fee_rate_c, "0.004");
        assert_eq!(response.taker_fee_rate_d, "0.006");
        assert_eq!(response.maker_fee_rate_d, "0.006");
    }

    #[test]
    fn test_vip_user_response() {
        let response = GetBasicFeeRateResponse {
            user_rate_type: 1, // VIP User
            level: "VIP1".to_string(),
            taker_fee_rate_a: "0.0008".to_string(),
            maker_fee_rate_a: "0.0008".to_string(),
            taker_fee_rate_b: "0.002".to_string(),
            maker_fee_rate_b: "0.002".to_string(),
            taker_fee_rate_c: "0.0035".to_string(),
            maker_fee_rate_c: "0.0035".to_string(),
            taker_fee_rate_d: "0.005".to_string(),
            maker_fee_rate_d: "0.005".to_string(),
        };

        assert_eq!(response.user_rate_type, 1);
        assert_eq!(response.level, "VIP1");
        assert_eq!(response.taker_fee_rate_a, "0.0008");
        assert_eq!(response.maker_fee_rate_a, "0.0008");
    }

    #[test]
    fn test_special_vip_user_response() {
        let response = GetBasicFeeRateResponse {
            user_rate_type: 2, // Special VIP User
            level: "VIP5".to_string(),
            taker_fee_rate_a: "0.0005".to_string(),
            maker_fee_rate_a: "0.0005".to_string(),
            taker_fee_rate_b: "0.0015".to_string(),
            maker_fee_rate_b: "0.0015".to_string(),
            taker_fee_rate_c: "0.003".to_string(),
            maker_fee_rate_c: "0.003".to_string(),
            taker_fee_rate_d: "0.004".to_string(),
            maker_fee_rate_d: "0.004".to_string(),
        };

        assert_eq!(response.user_rate_type, 2);
        assert_eq!(response.level, "VIP5");
        assert_eq!(response.taker_fee_rate_a, "0.0005");
        assert_eq!(response.maker_fee_rate_a, "0.0005");
    }

    #[test]
    fn test_fee_rate_serialization_roundtrip() {
        let response = GetBasicFeeRateResponse {
            user_rate_type: 0,
            level: "LV2".to_string(),
            taker_fee_rate_a: "0.0009".to_string(),
            maker_fee_rate_a: "0.0009".to_string(),
            taker_fee_rate_b: "0.0024".to_string(),
            maker_fee_rate_b: "0.0024".to_string(),
            taker_fee_rate_c: "0.0039".to_string(),
            maker_fee_rate_c: "0.0039".to_string(),
            taker_fee_rate_d: "0.0059".to_string(),
            maker_fee_rate_d: "0.0059".to_string(),
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: GetBasicFeeRateResponse = serde_json::from_str(&serialized).unwrap();

        assert_eq!(response.user_rate_type, deserialized.user_rate_type);
        assert_eq!(response.level, deserialized.level);
        assert_eq!(response.taker_fee_rate_a, deserialized.taker_fee_rate_a);
        assert_eq!(response.maker_fee_rate_a, deserialized.maker_fee_rate_a);
        assert_eq!(response.taker_fee_rate_b, deserialized.taker_fee_rate_b);
        assert_eq!(response.maker_fee_rate_b, deserialized.maker_fee_rate_b);
        assert_eq!(response.taker_fee_rate_c, deserialized.taker_fee_rate_c);
        assert_eq!(response.maker_fee_rate_c, deserialized.maker_fee_rate_c);
        assert_eq!(response.taker_fee_rate_d, deserialized.taker_fee_rate_d);
        assert_eq!(response.maker_fee_rate_d, deserialized.maker_fee_rate_d);
    }

    #[test]
    fn test_response_json_parsing() {
        let json = r#"{
            "user_rate_type": 0,
            "level": "LV1",
            "taker_fee_rate_a": "0.001",
            "maker_fee_rate_a": "0.001",
            "taker_fee_rate_b": "0.0025",
            "maker_fee_rate_b": "0.0025",
            "taker_fee_rate_c": "0.004",
            "maker_fee_rate_c": "0.004",
            "taker_fee_rate_d": "0.006",
            "maker_fee_rate_d": "0.006"
        }"#;

        let response: GetBasicFeeRateResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.user_rate_type, 0);
        assert_eq!(response.level, "LV1");
        assert_eq!(response.taker_fee_rate_a, "0.001");
        assert_eq!(response.maker_fee_rate_a, "0.001");
        assert_eq!(response.taker_fee_rate_b, "0.0025");
        assert_eq!(response.maker_fee_rate_b, "0.0025");
        assert_eq!(response.taker_fee_rate_c, "0.004");
        assert_eq!(response.maker_fee_rate_c, "0.004");
        assert_eq!(response.taker_fee_rate_d, "0.006");
        assert_eq!(response.maker_fee_rate_d, "0.006");
    }
}
