use serde::{Deserialize, Serialize};

use crate::binance::usdm::{RestResult, private_client::UsdmClient};

/// Endpoint path for getting current multi-assets mode status.
const MULTI_ASSETS_MARGIN_STATUS_ENDPOINT: &str = "/fapi/v1/multiAssetsMargin";

/// Request parameters for the Get Current Multi-Assets Mode endpoint.
///
/// This struct is used to query the user's multi-assets mode status.
/// All fields are sent as query parameters.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetMultiAssetsMarginStatusRequest {
    /// Receiving window time in milliseconds (optional, default 5000ms).
    /// Used to specify the number of milliseconds after timestamp the request is valid for.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds since epoch.
    /// Required by Binance API for all signed requests.
    pub timestamp: u64,
}

/// Response from the Get Current Multi-Assets Mode endpoint.
///
/// Indicates whether multi-assets mode is enabled for the user.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiAssetsMarginStatusResponse {
    /// Whether multi-assets mode is enabled.
    /// true: Multi-Assets Mode; false: Single-Asset Mode.
    pub multi_assets_margin: bool,
}

impl UsdmClient {
    /// Get Current Multi-Assets Mode
    ///
    /// Check user's Multi-Assets mode (Multi-Assets Mode or Single-Asset Mode) on every symbol.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Get-Current-Multi-Assets-Mode)
    ///
    /// Rate limit: 30
    ///
    /// # Arguments
    /// * `request` - The request parameters for the endpoint
    ///
    /// # Returns
    /// Returns [`MultiAssetsMarginStatusResponse`] indicating whether multi-assets mode is enabled.
    pub async fn get_multi_assets_margin_status(
        &self,
        request: GetMultiAssetsMarginStatusRequest,
    ) -> RestResult<MultiAssetsMarginStatusResponse> {
        self.send_get_signed_request(MULTI_ASSETS_MARGIN_STATUS_ENDPOINT, request, 30, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_get_multi_assets_margin_status_request_default() {
        let request = GetMultiAssetsMarginStatusRequest::default();
        assert!(request.recv_window.is_none());
        // timestamp defaults to 0
        assert_eq!(request.timestamp, 0);
    }

    #[test]
    fn test_get_multi_assets_margin_status_request_serialization() {
        let request = GetMultiAssetsMarginStatusRequest {
            recv_window: Some(5000),
            timestamp: 1234567890123,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1234567890123"));
    }

    #[test]
    fn test_multi_assets_margin_status_response_deserialization_true() {
        let json = r#"{ "multiAssetsMargin": true }"#;
        let response: MultiAssetsMarginStatusResponse = serde_json::from_str(json).unwrap();
        assert!(response.multi_assets_margin);
    }

    #[test]
    fn test_multi_assets_margin_status_response_deserialization_false() {
        let json = r#"{ "multiAssetsMargin": false }"#;
        let response: MultiAssetsMarginStatusResponse = serde_json::from_str(json).unwrap();
        assert!(!response.multi_assets_margin);
    }
}
