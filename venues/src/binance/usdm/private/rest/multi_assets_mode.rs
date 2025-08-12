use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::{BooleanStatus, RestResult};

/// Endpoint path for changing multi-assets margin mode.
const CHANGE_MULTI_ASSETS_MODE_ENDPOINT: &str = "/fapi/v1/multiAssetsMargin";

/// Request parameters for the Change Multi-Assets Mode endpoint.
///
/// Changes the user's margin mode between Multi-Assets Mode and Single-Asset Mode.
/// See [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Change-Multi-Assets-Mode)
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMultiAssetsModeRequest {
    /// Multi-assets mode status.
    /// - `BooleanStatus::True`: Multi-assets mode enabled
    /// - `BooleanStatus::False`: Multi-assets mode disabled (single-asset mode)
    pub multi_assets_margin: BooleanStatus,

    /// Receiving window timeout in milliseconds (optional, max 60000).
    /// Securely stored and expected as Option<u64>.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp (milliseconds since epoch).
    /// Required by Binance API. Must be current server time.
    pub timestamp: u64,
}

/// Response confirming the multi-assets mode change.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMultiAssetsModeResponse {
    /// Response code (200 indicates success).
    pub code: i32,

    /// Response message from the API.
    pub msg: Cow<'static, str>,
}

impl UsdmClient {
    /// Change Multi-Assets Mode (TRADE)
    ///
    /// Change user's multi-assets margin mode between Multi-Assets Mode and Single-Asset Mode.
    /// This setting affects how margin is calculated and isolated across different assets.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Change-Multi-Assets-Mode)
    ///
    /// Rate limit: 1
    ///
    /// # Arguments
    /// * `request` - The request parameters (see [`ChangeMultiAssetsModeRequest`])
    ///
    /// # Returns
    /// [`ChangeMultiAssetsModeResponse`] confirming the multi-assets mode change.
    pub async fn change_multi_assets_mode(
        &self,
        request: ChangeMultiAssetsModeRequest,
    ) -> RestResult<ChangeMultiAssetsModeResponse> {
        self.send_post_signed_request(CHANGE_MULTI_ASSETS_MODE_ENDPOINT, request, 1, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_multi_assets_mode_request_serialization_enable() {
        let request = ChangeMultiAssetsModeRequest {
            multi_assets_margin: BooleanStatus::True,
            recv_window: None,
            timestamp: 1234567890,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("multiAssetsMargin=true"));
        assert!(!serialized.contains("recvWindow"));
        assert!(serialized.contains("timestamp=1234567890"));
    }

    #[test]
    fn test_change_multi_assets_mode_request_serialization_disable() {
        let request = ChangeMultiAssetsModeRequest {
            multi_assets_margin: BooleanStatus::False,
            recv_window: Some(5000),
            timestamp: 9876543210,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("multiAssetsMargin=false"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=9876543210"));
    }

    #[test]
    fn test_change_multi_assets_mode_response_deserialization() {
        let json = r#"{"code":200,"msg":"success"}"#;
        let response: ChangeMultiAssetsModeResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.code, 200);
        assert_eq!(response.msg, "success");
    }
}
