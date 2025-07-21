use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::RestResult;

const MULTI_ASSETS_MARGIN_STATUS_ENDPOINT: &str = "/fapi/v1/multiAssetsMargin";

/// Request parameters for getting multi-assets margin status.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetMultiAssetsMarginStatusRequest {
    /// Receiving window time (optional, default 5000ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Response from multi-assets margin status endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiAssetsMarginStatusResponse {
    /// Whether multi-assets mode is enabled.
    pub multi_assets_margin: bool,
}

impl UsdmClient {
    /// Get Multi-Assets Mode Status
    ///
    /// Check current multi-assets mode status.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Get-Current-Multi-Assets-Mode
    ///
    /// Rate limit: 30
    ///
    /// # Arguments
    /// * `request` - The request parameters
    ///
    /// # Returns
    /// Multi-assets margin status response
    pub async fn get_multi_assets_margin_status(
        &self,
        request: GetMultiAssetsMarginStatusRequest,
    ) -> RestResult<MultiAssetsMarginStatusResponse> {
        self.send_signed_request(
            MULTI_ASSETS_MARGIN_STATUS_ENDPOINT,
            reqwest::Method::GET,
            request,
            30,
            false,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_multi_assets_margin_status_request_default() {
        let request = GetMultiAssetsMarginStatusRequest::default();
        assert!(request.recv_window.is_none());
    }

    #[test]
    fn test_get_multi_assets_margin_status_request_serialization() {
        let request = GetMultiAssetsMarginStatusRequest {
            recv_window: Some(5000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("recvWindow=5000"));
    }

    #[test]
    fn test_multi_assets_margin_status_response_deserialization() {
        let json = r#"
        {
            "multiAssetsMargin": true
        }
        "#;

        let response: MultiAssetsMarginStatusResponse = serde_json::from_str(json).unwrap();
        assert!(response.multi_assets_margin);
    }

    #[test]
    fn test_multi_assets_margin_status_response_disabled() {
        let json = r#"
        {
            "multiAssetsMargin": false
        }
        "#;

        let response: MultiAssetsMarginStatusResponse = serde_json::from_str(json).unwrap();
        assert!(!response.multi_assets_margin);
    }
}
