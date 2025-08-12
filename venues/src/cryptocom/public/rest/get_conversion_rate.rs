use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::cryptocom::{ApiResult, RestResult, rate_limit::EndpointType};

/// Endpoint path for the get-conversion-rate API
const CONVERSION_RATE_ENDPOINT: &str = "public/staking/get-conversion-rate";

/// Request parameters for get conversion rate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetConversionRateRequest {
    /// Liquid staking token instrument name: CDCETH
    pub instrument_name: String,
}

/// Conversion rate response information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionRateResult {
    /// CDCETH
    pub instrument_name: String,

    /// Conversion rate between staked token (ETH.staked) and liquid staking token (CDCETH)
    pub conversion_rate: String,
}

/// Response wrapper for get-conversion-rate endpoint
pub type ConversionRateResponse = ApiResult<ConversionRateResult>;

impl RestClient {
    /// Get conversion rate between staked token and liquid staking token
    ///
    /// Returns the current conversion rate for liquid staking tokens.
    ///
    /// See: <https://exchange-docs.crypto.com/exchange/v1/rest-ws/index.html#public-staking-get-conversion-rate>
    ///
    /// Rate limit: 50 requests per second
    ///
    /// # Arguments
    /// * `params` - Request parameters including instrument_name
    ///
    /// # Returns
    /// Conversion rate information between staked and liquid staking tokens
    pub async fn get_conversion_rate(
        &self,
        params: GetConversionRateRequest,
    ) -> RestResult<ConversionRateResponse> {
        self.send_post_request(
            CONVERSION_RATE_ENDPOINT,
            Some(&params),
            EndpointType::PublicStaking,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_get_conversion_rate_request_structure() {
        let request = GetConversionRateRequest {
            instrument_name: "CDCETH".to_string(),
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value.get("instrument_name").unwrap(), "CDCETH");
    }

    #[test]
    fn test_get_conversion_rate_request_serialization() {
        let request = GetConversionRateRequest {
            instrument_name: "CDCETH".to_string(),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: GetConversionRateRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.instrument_name, "CDCETH");
    }

    #[test]
    fn test_conversion_rate_response_structure() {
        let response_json = json!({
            "code": 0,
            "result": {
                "instrument_name": "CDCETH",
                "conversion_rate": "1.0203"
            },
            "id": -1
        });

        let response: ConversionRateResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.result.instrument_name, "CDCETH");
        assert_eq!(response.result.conversion_rate, "1.0203");
    }

    #[test]
    fn test_conversion_rate_response_different_rates() {
        let rates = vec!["1.0000", "1.0203", "1.0500", "0.9800", "0.9999"];

        for rate in rates {
            let response_json = json!({
                "code": 0,
                "result": {
                    "instrument_name": "CDCETH",
                    "conversion_rate": rate
                },
                "id": -1
            });

            let response: ConversionRateResponse = serde_json::from_value(response_json).unwrap();
            assert_eq!(response.result.conversion_rate, rate);
            assert_eq!(response.result.instrument_name, "CDCETH");
        }
    }

    #[test]
    fn test_conversion_rate_response_serialization() {
        let response = ConversionRateResult {
            instrument_name: "CDCETH".to_string(),
            conversion_rate: "1.0203".to_string(),
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: ConversionRateResult = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.instrument_name, "CDCETH");
        assert_eq!(deserialized.conversion_rate, "1.0203");
    }

    #[test]
    fn test_conversion_rate_precision() {
        // Test various precision levels for conversion rates
        let precise_rates = vec![
            "1.02030000",
            "1.0203",
            "1.020300000000",
            "0.98010000",
            "1.0000000000",
        ];

        for rate in precise_rates {
            let response_json = json!({
                "code": 0,
                "result": {
                    "instrument_name": "CDCETH",
                    "conversion_rate": rate
                },
                "id": -1
            });

            let response: ConversionRateResponse = serde_json::from_value(response_json).unwrap();
            assert_eq!(response.result.conversion_rate, rate);
        }
    }

    #[test]
    fn test_conversion_rate_endpoint_type() {
        // Verify that the endpoint uses the correct endpoint type for rate limiting
        let endpoint_type = EndpointType::from_path("public/staking/get-conversion-rate");
        assert_eq!(endpoint_type, EndpointType::PublicStaking);
    }
}
