//! Implements the /public/get_historical_volatility endpoint for Deribit.
//!
//! Retrieves historical volatility data for a given currency.

use super::RestClient;
use crate::deribit::enums::Currency;
use crate::deribit::{EndpointType, RestResult};

use reqwest::Method;
use serde::{Deserialize, Serialize};

const HISTORICAL_VOLATILITY_ENDPOINT: &str = "get_historical_volatility";

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

/// Represents a single historical volatility data point.
#[derive(Debug, Clone, Deserialize)]
pub struct HistoricalVolatilityData {
    /// Volatility value (annualized, as a percentage).
    #[serde(rename = "volatility")]
    pub volatility: f64,

    /// Timestamp in milliseconds since epoch.
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// The result object for get_historical_volatility.
#[derive(Debug, Clone, Deserialize)]
pub struct GetHistoricalVolatilityResult {
    /// Array of historical volatility data points.
    #[serde(rename = "data")]
    pub data: Vec<HistoricalVolatilityData>,
}

/// Response for the get_historical_volatility endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct GetHistoricalVolatilityResponse {
    /// The id that was sent in the request.
    #[serde(rename = "id")]
    pub id: u64,

    /// The JSON-RPC version (2.0).
    #[serde(rename = "jsonrpc")]
    pub jsonrpc: String,

    /// The result object containing historical volatility data.
    #[serde(rename = "result")]
    pub result: GetHistoricalVolatilityResult,
}

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
        self.send_request(
            HISTORICAL_VOLATILITY_ENDPOINT,
            Method::POST,
            Some(&params),
            EndpointType::NonMatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

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
            "result": {
                "data": [
                    { "volatility": 0.65, "timestamp": 1680307200000 },
                    { "volatility": 0.66, "timestamp": 1680310800000 }
                ]
            }
        }"#;
        let resp: GetHistoricalVolatilityResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 10);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.data.len(), 2);
        assert!((resp.result.data[0].volatility - 0.65).abs() < 1e-8);
        assert_eq!(resp.result.data[1].timestamp, 1680310800000);
    }
}
