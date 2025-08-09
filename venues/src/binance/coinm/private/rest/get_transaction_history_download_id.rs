use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, private::rest::client::RestClient};

const INCOME_ASYN_ENDPOINT: &str = "/dapi/v1/income/asyn";

/// Request parameters for getting download ID for transaction history.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDownloadIdRequest {
    /// Start time in milliseconds
    pub start_time: u64,

    /// End time in milliseconds
    pub end_time: u64,

    /// This parameter cannot be used in combination with other parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Response for getting download ID.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDownloadIdResponse {
    /// Average time in seconds for processing the file
    pub avg_cost_timestamp: String,

    /// Download ID
    pub download_id: String,
}

impl RestClient {
    /// Get download ID for transaction history on Binance Coin-M Futures.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/coin-margined-futures/account/rest-api/Get-Futures-Transaction-History-Download-Link-by-Id
    ///
    /// GET /dapi/v1/income/asyn
    /// Weight: 5
    /// Requires API key and signature.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`GetDownloadIdRequest`])
    ///
    /// # Returns
    /// A [`GetDownloadIdResponse`] object with download ID details.
    pub async fn get_download_id_for_transaction_history(
        &self,
        params: GetDownloadIdRequest,
    ) -> RestResult<GetDownloadIdResponse> {
        let weight = 5;
        self.send_get_signed_request(INCOME_ASYN_ENDPOINT, params, weight, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_download_id_request_serialization() {
        let request = GetDownloadIdRequest {
            start_time: 1625097600000,
            end_time: 1625184000000,
            recv_window: None,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("startTime=1625097600000"));
        assert!(serialized.contains("endTime=1625184000000"));
    }

    #[test]
    fn test_get_download_id_request_serialization_with_recv_window() {
        let request = GetDownloadIdRequest {
            start_time: 1625097600000,
            end_time: 1625184000000,
            recv_window: Some(5000),
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("startTime=1625097600000"));
        assert!(serialized.contains("endTime=1625184000000"));
        assert!(serialized.contains("recvWindow=5000"));
    }

    #[test]
    fn test_get_download_id_response_deserialization() {
        let json = r#"{
            "avgCostTimestamp": "300",
            "downloadId": "123456"
        }"#;
        let response: GetDownloadIdResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.avg_cost_timestamp, "300");
        assert_eq!(response.download_id, "123456");
    }
}
