use super::client::RestClient;
use crate::cryptocom::rate_limit::EndpointType;
use crate::cryptocom::RestResult;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Request parameters for get conversion rate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetConversionRateRequest {
    /// Liquid staking token instrument name: CDCETH
    pub instrument_name: String,
}

/// Conversion rate response information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionRateResponse {
    /// CDCETH
    pub instrument_name: String,
    /// Conversion rate between staked token (ETH.staked) and liquid staking token (CDCETH)
    pub conversion_rate: String,
}

impl RestClient {
    /// Get conversion rate between staked token and liquid staking token
    ///
    /// Returns the current conversion rate for liquid staking tokens.
    ///
    /// See: <https://exchange-docs.crypto.com/exchange/index.html#public-staking-get-conversion-rate>
    ///
    /// Rate limit: 50 requests per second
    ///
    /// # Arguments
    /// * `instrument_name` - Liquid staking token instrument name: "CDCETH"
    ///
    /// # Returns
    /// Conversion rate information between staked and liquid staking tokens
    pub async fn get_conversion_rate(&self, instrument_name: &str) -> RestResult<Value> {
        let params_value = json!({
            "instrument_name": instrument_name
        });

        self.send_request(
            "public/staking/get-conversion-rate",
            reqwest::Method::POST,
            Some(&params_value),
            EndpointType::PublicStaking,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_get_conversion_rate_request_structure() {
        let request = GetConversionRateRequest {
            instrument_name: "CDCETH".to_string(),
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value["instrument_name"], "CDCETH");
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
            "instrument_name": "CDCETH",
            "conversion_rate": "1.0203"
        });

        let response: ConversionRateResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.instrument_name, "CDCETH");
        assert_eq!(response.conversion_rate, "1.0203");
    }

    #[test]
    fn test_conversion_rate_response_different_rates() {
        let rates = vec!["1.0000", "1.0203", "1.0500", "0.9800", "0.9999"];

        for rate in rates {
            let response_json = json!({
                "instrument_name": "CDCETH",
                "conversion_rate": rate
            });

            let response: ConversionRateResponse = serde_json::from_value(response_json).unwrap();
            assert_eq!(response.conversion_rate, rate);
            assert_eq!(response.instrument_name, "CDCETH");
        }
    }

    #[test]
    fn test_conversion_rate_response_serialization() {
        let response = ConversionRateResponse {
            instrument_name: "CDCETH".to_string(),
            conversion_rate: "1.0203".to_string(),
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: ConversionRateResponse = serde_json::from_str(&serialized).unwrap();

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
                "instrument_name": "CDCETH",
                "conversion_rate": rate
            });

            let response: ConversionRateResponse = serde_json::from_value(response_json).unwrap();
            assert_eq!(response.conversion_rate, rate);
        }
    }

    #[test]
    fn test_conversion_rate_endpoint_type() {
        // Verify that the endpoint uses the correct endpoint type for rate limiting
        let endpoint_type = EndpointType::from_path("public/staking/get-conversion-rate");
        assert_eq!(endpoint_type, EndpointType::PublicStaking);
    }
}
