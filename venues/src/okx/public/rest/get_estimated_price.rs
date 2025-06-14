use serde::{Deserialize, Serialize};
use super::client::RestClient;
use crate::okx::{EndpointType, RestResult};

/// Request parameters for getting estimated delivery/exercise price
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEstimatedPriceRequest {
    /// Instrument ID (e.g., "BTC-USD-200214")
    #[serde(rename = "instId")]
    pub inst_id: String,
}

/// Response for getting estimated delivery/exercise price
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEstimatedPriceResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Estimated price data
    pub data: Vec<EstimatedPriceData>,
}

/// Estimated delivery/exercise price data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstimatedPriceData {
    /// Instrument type
    #[serde(rename = "instType")]
    pub inst_type: String,
    /// Instrument ID
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
    /// See: https://www.okx.com/docs-v5/en/#rest-api-public-data-get-estimated-delivery-exercise-price
    ///
    /// Rate limit: 10 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The estimated price request parameters
    ///
    /// # Returns
    /// Response containing the estimated delivery/exercise price data
    pub async fn get_estimated_price(
        &self,
        request: GetEstimatedPriceRequest,
    ) -> RestResult<GetEstimatedPriceResponse> {
        self.send_request(
            "api/v5/public/estimated-price",
            reqwest::Method::GET,
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

    #[test]
    fn test_get_estimated_price_request_structure() {
        let request = GetEstimatedPriceRequest {
            inst_id: "BTC-USD-200214".to_string(),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("instId").and_then(|v| v.as_str()), Some("BTC-USD-200214"));
    }

    #[test]
    fn test_estimated_price_data_structure() {
        let estimated_price_json = json!({
            "instType": "FUTURES",
            "instId": "BTC-USD-200214",
            "settlePx": "50000.5",
            "ts": "1597026383085"
        });

        let estimated_price_data: EstimatedPriceData = serde_json::from_value(estimated_price_json).unwrap();
        assert_eq!(estimated_price_data.inst_type, "FUTURES");
        assert_eq!(estimated_price_data.inst_id, "BTC-USD-200214");
        assert_eq!(estimated_price_data.settle_px, "50000.5");
        assert_eq!(estimated_price_data.ts, "1597026383085");
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

        let response: GetEstimatedPriceResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data.first().unwrap().inst_id, "BTC-USD-200214");
    }
}