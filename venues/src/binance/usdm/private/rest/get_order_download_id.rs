use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::RestResult;

const GET_ORDER_DOWNLOAD_ID_ENDPOINT: &str = "/fapi/v1/order/asyn";

/// Request for getting order download ID.
///
/// Parameters for retrieving a download ID for order history.
/// The time between start_time and end_time cannot be longer than 1 year.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderDownloadIdRequest {
    /// Start time for filtering orders (milliseconds since epoch).
    pub start_time: u64,

    /// End time for filtering orders (milliseconds since epoch).
    pub end_time: u64,

    /// Receive window for request validity (optional, default 5000ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Response from order download ID endpoint.
///
/// Contains the download ID and average processing time information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderDownloadIdResponse {
    /// Average time taken for data download in the past 30 days.
    pub avg_cost_timestamp: String,

    /// Download ID to be used for retrieving the download link.
    pub download_id: String,
}

impl UsdmClient {
    /// Get Download Id For Order History (USER_DATA)
    ///
    /// Get download id for order history
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Get-Download-Id-For-Order-History
    ///
    /// Rate limit: 5
    ///
    /// # Arguments
    /// * `request` - The download ID request parameters
    ///
    /// # Returns
    /// Download ID and estimated processing time information
    pub async fn get_order_download_id(
        &self,
        request: GetOrderDownloadIdRequest,
    ) -> RestResult<GetOrderDownloadIdResponse> {
        self.send_signed_request(
            GET_ORDER_DOWNLOAD_ID_ENDPOINT,
            Method::GET,
            request,
            5,
            false,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_order_download_id_request_serialization() {
        let request = GetOrderDownloadIdRequest {
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
    fn test_get_order_download_id_request_serialization_minimal() {
        let request = GetOrderDownloadIdRequest {
            start_time: 1625097600000,
            end_time: 1625184000000,
            recv_window: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("startTime=1625097600000"));
        assert!(serialized.contains("endTime=1625184000000"));
        assert!(!serialized.contains("recvWindow"));
    }

    #[test]
    fn test_get_order_download_id_response_deserialization() {
        let json = r#"{
            "avgCostTimestamp": "946684800000",
            "downloadId": "download123456"
        }"#;

        let response: GetOrderDownloadIdResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.avg_cost_timestamp, "946684800000");
        assert_eq!(response.download_id, "download123456");
    }
}
