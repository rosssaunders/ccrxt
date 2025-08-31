//! Implements the /public/get_funding_chart_data endpoint for Deribit.
//!
//! Retrieves funding chart data for a given instrument name.
use serde::{Deserialize, Serialize};

use crate::deribit::{EndpointType, JsonRpcResult, PublicRestClient, RestResult};

const FUNDING_CHART_DATA_ENDPOINT: &str = "public/get_funding_chart_data";

/// Request parameters for the get_funding_chart_data endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetFundingChartDataRequest {
    /// Instrument name (e.g., "BTC-PERPETUAL").
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,

    /// Specifies time period. 8h - 8 hours, 24h - 24 hours, 1m - 1 month
    #[serde(rename = "length")]
    pub length: String,
}

/// Represents a single funding chart data point.
#[derive(Debug, Clone, Deserialize)]
pub struct FundingChartDataPoint {
    /// Timestamp in milliseconds since epoch.
    #[serde(rename = "timestamp")]
    pub timestamp: u64,

    /// 8-hour interest rate.
    #[serde(rename = "interest_8h")]
    pub interest_8h: f64,

    /// Index price at this timestamp.
    #[serde(rename = "index_price")]
    pub index_price: f64,
}

/// The result object for get_funding_chart_data.
#[derive(Debug, Clone, Deserialize)]
pub struct GetFundingChartDataResult {
    /// Array of funding chart data points.
    #[serde(rename = "data")]
    pub data: Vec<FundingChartDataPoint>,
}

/// Response for the get_funding_chart_data endpoint.
pub type GetFundingChartDataResponse = JsonRpcResult<GetFundingChartDataResult>;

impl PublicRestClient {
    /// Calls the /public/get_funding_chart_data endpoint.
    ///
    /// Retrieves funding chart data for a given instrument name.
    ///
    /// [docs](https://docs.deribit.com/#public-get_funding_chart_data)
    pub async fn get_funding_chart_data(
        &self,
        params: GetFundingChartDataRequest,
    ) -> RestResult<GetFundingChartDataResponse> {
        self.send_post_request(
            FUNDING_CHART_DATA_ENDPOINT,
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
            length: "8h".to_string(),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("BTC-PERPETUAL"));
        assert!(json.contains("8h"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            "id": 7,
            "jsonrpc": "2.0",
            "result": {
                "data": [
                    { "timestamp": 1680307200000, "interest_8h": 0.0001, "index_price": 65000.0 },
                    { "timestamp": 1680310800000, "interest_8h": 0.0002, "index_price": 65100.0 }
                ]
            }
        }"#;
        let resp: GetFundingChartDataResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 7);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.data.len(), 2);
        assert!((resp.result.data[0].interest_8h - 0.0001).abs() < 1e-8);
        assert!((resp.result.data[0].index_price - 65000.0).abs() < 1e-8);
        assert_eq!(resp.result.data[1].timestamp, 1680310800000);
    }
}
