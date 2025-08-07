//! Implements the /public/get_apr_history endpoint for Deribit.
//!
//! Retrieves historical APR data for specified currency. Only applicable to yield-generating tokens (`USDE`, `STETH`).

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult, enums::Currency};

const APR_HISTORY_ENDPOINT: &str = "public/get_apr_history";

/// Request parameters for the get_apr_history endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct GetAprHistoryRequest {
    /// Currency for which to retrieve APR history. Only `USDE` and `STETH` are supported.
    #[serde(rename = "currency")]
    pub currency: Currency,

    /// Number of days to retrieve (default 365, maximum 365).
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Used to receive APR history before given epoch day.
    #[serde(rename = "before", skip_serializing_if = "Option::is_none")]
    pub before: Option<u64>,
}

/// Represents a single APR history data point.
#[derive(Debug, Clone, Deserialize)]
pub struct AprHistoryData {
    /// The APR of the day.
    #[serde(rename = "apr")]
    pub apr: f64,

    /// The full epoch day.
    #[serde(rename = "day")]
    pub day: u64,
}

/// The result object for get_apr_history.
#[derive(Debug, Clone, Deserialize)]
pub struct GetAprHistoryResult {
    /// Continuation token for pagination.
    #[serde(rename = "continuation")]
    pub continuation: String,

    /// Array of APR history data points.
    #[serde(rename = "data")]
    pub data: Vec<AprHistoryData>,
}

/// Response for the get_apr_history endpoint.
pub type GetAprHistoryResponse = JsonRpcResult<GetAprHistoryResult>;

impl RestClient {
    /// Calls the /public/get_apr_history endpoint.
    ///
    /// Retrieves historical APR data for specified currency. Only applicable to yield-generating tokens (`USDE`, `STETH`).
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_apr_history)
    pub async fn get_apr_history(
        &self,
        params: GetAprHistoryRequest,
    ) -> RestResult<GetAprHistoryResponse> {
    self.send_post_request(
            APR_HISTORY_ENDPOINT,
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
        let req = GetAprHistoryRequest {
            currency: Currency::USDE,
            limit: Some(10),
            before: Some(1234567),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("USDE"));
        assert!(json.contains("limit"));
        assert!(json.contains("before"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
                "continuation": "abc123",
                "data": [
                    { "apr": 0.045, "day": 19000 },
                    { "apr": 0.046, "day": 19001 }
                ]
            }
        }"#;
        let resp: GetAprHistoryResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 1);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.continuation, "abc123");
        assert_eq!(resp.result.data.len(), 2);
        assert!((resp.result.data[0].apr - 0.045).abs() < 1e-8);
        assert_eq!(resp.result.data[1].day, 19001);
    }
}
