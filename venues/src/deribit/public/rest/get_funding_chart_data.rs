//! Implements the /public/get_funding_chart_data endpoint for Deribit.
//!
//! Retrieves funding chart data for a given instrument name.

use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, RestResult};

const FUNDING_CHART_DATA_ENDPOINT: &str = "public/get_funding_chart_data";

/// Request parameters for the get_funding_chart_data endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetFundingChartDataRequest {
    /// Instrument name (e.g., "BTC-PERPETUAL").
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
}

/// Represents a single funding chart data point.
#[derive(Debug, Clone, Deserialize)]
pub struct FundingChartDataPoint {
    /// Timestamp in milliseconds since epoch.
    #[serde(rename = "timestamp")]
    pub timestamp: u64,

    /// Funding rate value at this timestamp.
    #[serde(rename = "funding_rate")]
    pub funding_rate: f64,
}

/// The result object for get_funding_chart_data.
#[derive(Debug, Clone, Deserialize)]
pub struct GetFundingChartDataResult {
    /// Array of funding chart data points.
    #[serde(rename = "data")]
    pub data: Vec<FundingChartDataPoint>,
}

/// Response for the get_funding_chart_data endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct GetFundingChartDataResponse {
    /// The id that was sent in the request.
    #[serde(rename = "id")]
    pub id: u64,

    /// The JSON-RPC version (2.0).
    #[serde(rename = "jsonrpc")]
    pub jsonrpc: String,

    /// The result object containing funding chart data.
    #[serde(rename = "result")]
    pub result: GetFundingChartDataResult,
}

impl RestClient {
    /// Calls the /public/get_funding_chart_data endpoint.
    ///
    /// Retrieves funding chart data for a given instrument name.
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_funding_chart_data)
    pub async fn get_funding_chart_data(
        &self,
        params: GetFundingChartDataRequest,
    ) -> RestResult<GetFundingChartDataResponse> {
        self.send_request(
            FUNDING_CHART_DATA_ENDPOINT,
            Method::POST,
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
        let req = GetFundingChartDataRequest {
            instrument_name: "BTC-PERPETUAL".to_string(),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("BTC-PERPETUAL"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            "id": 7,
            "jsonrpc": "2.0",
            "result": {
                "data": [
                    { "timestamp": 1680307200000, "funding_rate": 0.0001 },
                    { "timestamp": 1680310800000, "funding_rate": 0.0002 }
                ]
            }
        }"#;
        let resp: GetFundingChartDataResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 7);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.data.len(), 2);
        assert!((resp.result.data[0].funding_rate - 0.0001).abs() < 1e-8);
        assert_eq!(resp.result.data[1].timestamp, 1680310800000);
    }
}
