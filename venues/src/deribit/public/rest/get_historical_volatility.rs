//! Implements the /public/get_historical_volatility endpoint for Deribit.
//!
//! Retrieves historical volatility data for a given currency.
use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult, enums::Currency};

const HISTORICAL_VOLATILITY_ENDPOINT: &str = "public/get_historical_volatility";

/// Request parameters for the get_historical_volatility endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct GetHistoricalVolatilityRequest {
    /// Currency for which to retrieve historical volatility.
    #[serde(rename = "currency")]
    pub currency: Currency,

    /// Number of data points to retrieve (default 365, max 365).
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,

    /// End timestamp in milliseconds since epoch (optional).
    #[serde(rename = "end_timestamp", skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<u64>,
}

/// Data structure for a single historical volatility data point.
#[derive(Debug, Clone, Deserialize)]
pub struct HistoricalVolatilityDataPoint {
    /// Timestamp in milliseconds since epoch.
    pub timestamp: u64,

    /// Volatility value.
    pub value: f64,
}

/// Helper type for the array of [timestamp, value] pairs returned by the API.
pub type RawVolatilityDataPoint = (u64, f64);

/// The result object for get_historical_volatility.
/// The result for get_historical_volatility is a direct array of data points
pub type GetHistoricalVolatilityResult = Vec<RawVolatilityDataPoint>;

/// Response for the get_historical_volatility endpoint.
pub type GetHistoricalVolatilityResponse = JsonRpcResult<GetHistoricalVolatilityResult>;

impl RestClient {
    /// Calls the /public/get_historical_volatility endpoint.
    ///
    /// Retrieves historical volatility data for a given currency.
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_historical_volatility)
    pub async fn get_historical_volatility(
        &self,
        params: GetHistoricalVolatilityRequest,
    ) -> RestResult<GetHistoricalVolatilityResponse> {
        self.send_post_request(
            HISTORICAL_VOLATILITY_ENDPOINT,
            Some(&params),
            EndpointType::NonMatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_serialize_request() {
        let req = GetHistoricalVolatilityRequest {
            currency: Currency::BTC,
            count: Some(10),
            end_timestamp: Some(1680310800000),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("BTC"));
        assert!(json.contains("count"));
        assert!(json.contains("end_timestamp"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            "id": 10,
            "jsonrpc": "2.0",
            "result": [
                [1680307200000, 0.65],
                [1680310800000, 0.66]
            ]
        }"#;
        let resp: GetHistoricalVolatilityResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 10);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.len(), 2);
        assert!((resp.result[0].1 - 0.65).abs() < 1e-8);
        assert_eq!(resp.result[1].0, 1680310800000);
    }
}
