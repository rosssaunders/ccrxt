use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::RestResult;

/// Endpoint path for getting download ID for futures transaction history.
const GET_INCOME_DOWNLOAD_ID_ENDPOINT: &str = "/fapi/v1/income/asyn";

/// Request parameters for the Get Download Id For Futures Transaction History endpoint.
///
/// Retrieves a download ID for futures transaction history. The time between `start_time` and `end_time` cannot be longer than 1 year.
///
/// All fields are required except `recv_window`, which is optional. See [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Transaction-History
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetIncomeDownloadIdRequest {
    /// Start time for filtering transactions (milliseconds since epoch).
    /// Must be less than or equal to `end_time`.
    pub start_time: u64,

    /// End time for filtering transactions (milliseconds since epoch).
    /// Must be greater than or equal to `start_time`.
    pub end_time: u64,

    /// Request timestamp (milliseconds since epoch).
    /// Used for request authentication and validity.
    pub timestamp: u64,

    /// Optional receive window for request validity (milliseconds).
    /// Default is 5000ms if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Response for the Get Download Id For Futures Transaction History endpoint.
///
/// Contains the download ID and average processing time information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIncomeDownloadIdResponse {
    /// Average time taken for data download in the past 30 days (in milliseconds).
    pub avg_cost_timestamp_of_last30d: u64,

    /// Download ID to be used for retrieving the download link.
    pub download_id: String,
}

impl UsdmClient {
    /// Get Download Id For Futures Transaction History (USER_DATA)
    ///
    /// Retrieves a download ID for futures transaction history.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Transaction-History
    ///
    /// Rate limit: 1000
    ///
    /// # Arguments
    /// * `request` - The request parameters for download ID
    ///
    /// # Returns
    /// Returns a struct containing the download ID and average processing time information.
    pub async fn get_income_download_id(
        &self,
        request: GetIncomeDownloadIdRequest,
    ) -> RestResult<GetIncomeDownloadIdResponse> {
        self.send_get_signed_request(GET_INCOME_DOWNLOAD_ID_ENDPOINT, request, 1000, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_income_download_id_request_serialization_full() {
        let request = GetIncomeDownloadIdRequest {
            start_time: 1625097600000,
            end_time: 1625184000000,
            timestamp: 1625184000001,
            recv_window: Some(5000),
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("startTime=1625097600000"));
        assert!(serialized.contains("endTime=1625184000000"));
        assert!(serialized.contains("timestamp=1625184000001"));
        assert!(serialized.contains("recvWindow=5000"));
    }

    #[test]
    fn test_get_income_download_id_request_serialization_minimal() {
        let request = GetIncomeDownloadIdRequest {
            start_time: 1625097600000,
            end_time: 1625184000000,
            timestamp: 1625184000001,
            recv_window: None,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("startTime=1625097600000"));
        assert!(serialized.contains("endTime=1625184000000"));
        assert!(serialized.contains("timestamp=1625184000001"));
        assert!(!serialized.contains("recvWindow"));
    }

    #[test]
    fn test_get_income_download_id_response_deserialization() {
        let json = r#"{
            "avgCostTimestampOfLast30d": 946684800000,
            "downloadId": "download123456"
        }"#;
        let response: GetIncomeDownloadIdResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.avg_cost_timestamp_of_last30d, 946684800000);
        assert_eq!(response.download_id, "download123456");
    }
}
