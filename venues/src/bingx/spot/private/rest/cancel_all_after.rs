use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::spot::{
    EndpointType, RestResult,
    enums::{CancelAllAfterStatus, CancelAllAfterType},
};

/// Cancel all after endpoint URL
const CANCEL_ALL_AFTER_ENDPOINT: &str = "/openApi/spot/v1/trade/cancelAllAfter";

/// Request for cancel all after functionality
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllAfterRequest {
    /// Request type: ACTIVATE or CLOSE
    #[serde(rename = "type")]
    pub request_type: CancelAllAfterType,
    /// Activate countdown time (seconds), range: 10s-120s
    #[serde(rename = "timeOut")]
    pub timeout: u32,
    /// Request timestamp in milliseconds
    pub timestamp: u64,
    /// Request valid time window in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Response for cancel all after request
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllAfterResponse {
    /// Trigger time for deleting all pending orders
    pub trigger_time: i64,
    /// Status of the cancel all after operation
    pub status: CancelAllAfterStatus,
    /// Explanation note
    pub note: String,
}

impl RestClient {
    /// Set up cancel all after functionality
    ///
    /// After the countdown ends, cancel all current pending orders.
    /// This request can be continuously maintained to constantly extend the penalty time.
    ///
    /// # Arguments
    /// * `request` - The cancel all after request
    ///
    /// # Returns
    /// * `RestResult<CancelAllAfterResponse>` - The cancel all after response or error
    pub async fn cancel_all_after(
        &self,
        request: &CancelAllAfterRequest,
    ) -> RestResult<CancelAllAfterResponse> {
        self.send_post_signed_request(CANCEL_ALL_AFTER_ENDPOINT, request, EndpointType::Trading)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_all_after_request_serialization() {
        let request = CancelAllAfterRequest {
            request_type: CancelAllAfterType::Activate,
            timeout: 60,
            timestamp: 1658748648396,
            recv_window: Some(5000),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("ACTIVATE"));
        assert!(json.contains("\"timeOut\":60"));
        assert!(json.contains("1658748648396"));
    }

    #[test]
    fn test_cancel_all_after_request_close_serialization() {
        let request = CancelAllAfterRequest {
            request_type: CancelAllAfterType::Close,
            timeout: 0, // timeout is ignored for CLOSE
            timestamp: 1658748648396,
            recv_window: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("CLOSE"));
        assert!(json.contains("1658748648396"));
    }

    #[test]
    fn test_cancel_all_after_response_deserialization() {
        let json = r#"{
            "triggerTime": 1658748708396,
            "status": "ACTIVATED",
            "note": "Cancel all after activated successfully"
        }"#;

        let response: CancelAllAfterResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.trigger_time, 1658748708396);
        assert_eq!(response.status, CancelAllAfterStatus::Activated);
        assert!(response.note.contains("activated"));
    }

    #[test]
    fn test_cancel_all_after_response_closed_deserialization() {
        let json = r#"{
            "triggerTime": 0,
            "status": "CLOSED",
            "note": "Cancel all after closed successfully"
        }"#;

        let response: CancelAllAfterResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.trigger_time, 0);
        assert_eq!(response.status, CancelAllAfterStatus::Closed);
        assert!(response.note.contains("closed"));
    }
}
