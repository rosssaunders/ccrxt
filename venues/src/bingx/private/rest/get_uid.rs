use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::{EndpointType, RestResult};

/// Request to get account UID
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUidRequest {
    /// Timestamp of initiating the request, Unit: milliseconds
    /// This will be automatically set by the client
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,
}

/// Response from the get UID endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct GetUidResponse {
    /// User ID
    pub uid: String,
}

impl RestClient {
    /// Get account UID
    ///
    /// Retrieves the User ID (UID) for the current account.
    /// Rate limit: 5/s by UID
    ///
    /// # Arguments
    /// * `request` - The get UID request (can be empty for default parameters)
    ///
    /// # Returns
    /// A result containing the account UID or an error
    pub async fn get_uid(&self, request: &GetUidRequest) -> RestResult<GetUidResponse> {
        self.send_request(
            "/openApi/account/v1/uid",
            reqwest::Method::GET,
            Some(request),
            EndpointType::Account,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_uid_request_serialization() {
        let request = GetUidRequest {
            recv_window: Some(5000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("recvWindow=5000"));
    }

    #[test]
    fn test_get_uid_request_default() {
        let request = GetUidRequest::default();
        assert!(request.recv_window.is_none());
    }

    #[test]
    fn test_uid_response_deserialization() {
        let json = r#"{
            "uid": "123456789"
        }"#;

        let response: GetUidResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.uid, "123456789");
    }
}
