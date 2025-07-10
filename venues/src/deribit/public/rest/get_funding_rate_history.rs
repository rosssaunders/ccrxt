//! Implements the /public/get_funding_rate_history endpoint for Deribit.
//!
//! Retrieves historical funding rates for a given instrument.
use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult};

const FUNDING_RATE_HISTORY_ENDPOINT: &str = "public/get_funding_rate_history";

/// Request parameters for the get_funding_rate_history endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetFundingRateHistoryRequest {
    /// Instrument name (e.g., "BTC-PERPETUAL").
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,

    /// Start timestamp in milliseconds since epoch.
    #[serde(rename = "start_timestamp")]
    pub start_timestamp: u64,

    /// End timestamp in milliseconds since epoch.
    #[serde(rename = "end_timestamp")]
    pub end_timestamp: u64,

    /// Number of data points to retrieve (default 10, max 2000).
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
}

/// Represents a single funding rate history data point.
#[derive(Debug, Clone, Deserialize)]
pub struct FundingRateHistoryData {
    /// Timestamp in milliseconds since epoch.
    #[serde(rename = "timestamp")]
    pub timestamp: u64,

    /// 8-hour interest rate.
    #[serde(rename = "interest_8h")]
    pub interest_8h: f64,

    /// 1-hour interest rate.
    #[serde(rename = "interest_1h")]
    pub interest_1h: f64,

    /// Index price in base currency.
    #[serde(rename = "index_price")]
    pub index_price: f64,

    /// Previous index price in base currency.
    #[serde(rename = "prev_index_price")]
    pub prev_index_price: f64,
}

/// Array of funding rate history data points.
pub type GetFundingRateHistoryResult = Vec<FundingRateHistoryData>;

/// Response for the get_funding_rate_history endpoint.
pub type GetFundingRateHistoryResponse = JsonRpcResult<GetFundingRateHistoryResult>;

impl RestClient {
    /// Calls the /public/get_funding_rate_history endpoint.
    ///
    /// Retrieves historical funding rates for a given instrument.
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_funding_rate_history)
    pub async fn get_funding_rate_history(
        &self,
        params: GetFundingRateHistoryRequest,
    ) -> RestResult<GetFundingRateHistoryResponse> {
        self.send_request(
            FUNDING_RATE_HISTORY_ENDPOINT,
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
        let req = GetFundingRateHistoryRequest {
            instrument_name: "BTC-PERPETUAL".to_string(),
            start_timestamp: 1680307200000,
            end_timestamp: 1680310800000,
            count: Some(5),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("BTC-PERPETUAL"));
        assert!(json.contains("count"));
        assert!(json.contains("start_timestamp"));
        assert!(json.contains("end_timestamp"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            "id": 8,
            "jsonrpc": "2.0",
            "result": {
                "data": [
                    { 
                        "timestamp": 1680307200000, 
                        "interest_8h": 0.0001,
                        "interest_1h": 0.00005,
                        "index_price": 30000.0,
                        "prev_index_price": 29900.0
                    },
                    { 
                        "timestamp": 1680310800000, 
                        "interest_8h": 0.0002,
                        "interest_1h": 0.00006,
                        "index_price": 30100.0,
                        "prev_index_price": 30000.0
                    }
                ]
            }
        }"#;
        let resp: GetFundingRateHistoryResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 8);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.len(), 2);
        assert!((resp.result[0].interest_8h - 0.0001).abs() < 1e-8);
        assert_eq!(resp.result[1].timestamp, 1680310800000);
    }
}
