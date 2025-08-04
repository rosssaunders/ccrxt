use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::okx::{EndpointType, InstrumentType, RestResult};

const PUBLIC_ESTIMATED_PRICE_ENDPOINT: &str = "api/v5/public/estimated-price";

/// Request parameters for getting estimated delivery/exercise price
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEstimatedPriceRequest {
    /// Instrument ID, e.g. "BTC-USD-200214"
    /// Only applicable to FUTURES/OPTION
    #[serde(rename = "instId")]
    pub inst_id: String,
}

/// Estimated price data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstimatedPriceData {
    /// Instrument type (FUTURES/OPTION)
    #[serde(rename = "instType")]
    pub inst_type: InstrumentType,

    /// Instrument ID, e.g. "BTC-USD-200214"
    #[serde(rename = "instId")]
    pub inst_id: String,

    /// Estimated delivery/exercise price
    #[serde(rename = "settlePx")]
    pub settle_px: String,

    /// Data return time, Unix timestamp format in milliseconds
    pub ts: String,
}

impl RestClient {
    /// Get estimated delivery/exercise price
    ///
    /// Retrieve the estimated delivery price which will only have a return value one
    /// hour before the delivery/exercise.
    ///
    /// [docs]: https://www.okx.com/docs-v5/en/#rest-api-public-rest-api-get-estimated-delivery-exercise-price
    ///
    /// Rate limit: 10 requests per 2 seconds
    /// Rate limit rule: IP + Instrument ID
    ///
    /// # Arguments
    /// * `request` - The estimated price request parameters
    ///
    /// # Returns
    /// Response containing the estimated delivery/exercise price
    pub async fn get_estimated_price(
        &self,
        request: GetEstimatedPriceRequest,
    ) -> RestResult<Vec<EstimatedPriceData>> {
        self.send_get_request(
            PUBLIC_ESTIMATED_PRICE_ENDPOINT,
            Some(&request),
            EndpointType::PublicMarketData,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use crate::okx::response::OkxApiResponse;

    #[test]
    fn test_get_estimated_price_request_structure() {
        let request = GetEstimatedPriceRequest {
            inst_id: "BTC-USD-200214".to_string(),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instId").and_then(|v| v.as_str()),
            Some("BTC-USD-200214")
        );
    }

    #[test]
    fn test_estimated_price_data_structure() {
        let price_data_json = json!({
            "instType": "FUTURES",
            "instId": "BTC-USD-200214",
            "settlePx": "50000.5",
            "ts": "1597026383085"
        });

        let price_data: EstimatedPriceData = serde_json::from_value(price_data_json).unwrap();
        assert_eq!(price_data.inst_type, InstrumentType::Futures);
        assert_eq!(price_data.inst_id, "BTC-USD-200214");
        assert_eq!(price_data.settle_px, "50000.5");
        assert_eq!(price_data.ts, "1597026383085");
    }

    #[test]
    fn test_estimated_price_data_option_structure() {
        let price_data_json = json!({
            "instType": "OPTION",
            "instId": "BTC-USD-200214-50000-C",
            "settlePx": "48500.25",
            "ts": "1597026383085"
        });

        let price_data: EstimatedPriceData = serde_json::from_value(price_data_json).unwrap();
        assert_eq!(price_data.inst_type, InstrumentType::Option);
        assert_eq!(price_data.inst_id, "BTC-USD-200214-50000-C");
        assert_eq!(price_data.settle_px, "48500.25");
        assert_eq!(price_data.ts, "1597026383085");
    }

    #[test]
    fn test_get_estimated_price_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instType": "FUTURES",
                    "instId": "BTC-USD-200214",
                    "settlePx": "50000.5",
                    "ts": "1597026383085"
                }
            ]
        });

        let response: OkxApiResponse<EstimatedPriceData> = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.msg, "");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data.first().unwrap().inst_id, "BTC-USD-200214");
        assert_eq!(response.data.first().unwrap().settle_px, "50000.5");
    }

    #[test]
    fn test_get_estimated_price_response_empty_data() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": []
        });

        let response: OkxApiResponse<EstimatedPriceData> = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.msg, "");
        assert_eq!(response.data.len(), 0);
    }

    #[test]
    fn test_get_estimated_price_response_error_case() {
        let response_json = json!({
            "code": "51000",
            "msg": "Parameter {instId} can not be empty",
            "data": []
        });

        let response: OkxApiResponse<EstimatedPriceData> = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "51000");
        assert_eq!(response.msg, "Parameter {instId} can not be empty");
        assert_eq!(response.data.len(), 0);
    }

    #[test]
    fn test_estimated_price_serialization_roundtrip() {
        let original = GetEstimatedPriceRequest {
            inst_id: "BTC-USD-200214".to_string(),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: GetEstimatedPriceRequest = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.inst_id, deserialized.inst_id);
    }

    #[test]
    fn test_estimated_price_data_serialization_roundtrip() {
        let original = EstimatedPriceData {
            inst_type: InstrumentType::Futures,
            inst_id: "BTC-USD-200214".to_string(),
            settle_px: "50000.5".to_string(),
            ts: "1597026383085".to_string(),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: EstimatedPriceData = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.inst_type, deserialized.inst_type);
        assert_eq!(original.inst_id, deserialized.inst_id);
        assert_eq!(original.settle_px, deserialized.settle_px);
        assert_eq!(original.ts, deserialized.ts);
    }

    #[test]
    fn test_estimated_price_with_decimal_values() {
        let price_data_json = json!({
            "instType": "OPTION",
            "instId": "BTC-USD-200214-45000-P",
            "settlePx": "0.123456789",
            "ts": "1597026383085"
        });

        let price_data: EstimatedPriceData = serde_json::from_value(price_data_json).unwrap();
        assert_eq!(price_data.settle_px, "0.123456789");
    }

    #[tokio::test]
    async fn test_get_estimated_price_method_compilation() {
        // This test ensures the get_estimated_price method compiles and is accessible
        // without needing to make an actual HTTP request
        use crate::okx::RateLimiter;
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new();
        let rest_client = super::RestClient::new("https://www.okx.com", client, rate_limiter);

        // Verify the method exists and is properly typed
        let _ = super::RestClient::get_estimated_price;
        let _ = &rest_client;

        // This proves the method signature is correct without calling it
        println!("get_estimated_price method is accessible and properly typed");
    }
}
