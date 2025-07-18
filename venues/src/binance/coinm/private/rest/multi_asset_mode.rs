use serde::{Deserialize, Serialize};

use crate::binance::{
    coinm::{RestResult, private::rest::client::RestClient},
    shared,
};

const MULTI_ASSETS_MARGIN_ENDPOINT: &str = "/dapi/v1/multiAssetsMargin";

/// Request parameters for changing multi-asset mode.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMultiAssetModeRequest {
    /// "true" to enable multi-asset mode, "false" to disable
    pub multi_assets_margin: bool,

    /// This parameter cannot be used in combination with other parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Request parameters for getting multi-asset mode.
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetMultiAssetModeRequest {
    /// This parameter cannot be used in combination with other parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Response for changing multi-asset mode.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMultiAssetModeResponse {
    /// Code (200 for success)
    pub code: u32,

    /// Message
    pub msg: String,
}

/// Response for getting multi-asset mode.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMultiAssetModeResponse {
    /// Multi-asset mode enabled
    pub multi_assets_margin: bool,
}

impl RestClient {
    /// Change multi-asset mode on Binance Coin-M Futures.
    ///
    /// See: <https://binance-docs.github.io/apidocs/delivery/en/>
    /// POST /dapi/v1/multiAssetsMargin
    /// Weight: 1
    /// Requires API key and signature.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`ChangeMultiAssetModeRequest`])
    ///
    /// # Returns
    /// A [`ChangeMultiAssetModeResponse`] object with the operation result.
    pub async fn change_multi_asset_mode(
        &self,
        params: ChangeMultiAssetModeRequest,
    ) -> RestResult<ChangeMultiAssetModeResponse> {
        let weight = 1;
        shared::send_signed_request(
            self,
            MULTI_ASSETS_MARGIN_ENDPOINT,
            reqwest::Method::POST,
            params,
            weight,
            false,
        )
        .await
    }

    /// Get current multi-asset mode on Binance Coin-M Futures.
    ///
    /// See: <https://binance-docs.github.io/apidocs/delivery/en/>
    /// GET /dapi/v1/multiAssetsMargin
    /// Weight: 30
    /// Requires API key and signature.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`GetMultiAssetModeRequest`])
    ///
    /// # Returns
    /// A [`GetMultiAssetModeResponse`] object with the current multi-asset mode.
    pub async fn get_multi_asset_mode(
        &self,
        params: GetMultiAssetModeRequest,
    ) -> RestResult<GetMultiAssetModeResponse> {
        let weight = 30;
        shared::send_signed_request(
            self,
            MULTI_ASSETS_MARGIN_ENDPOINT,
            reqwest::Method::GET,
            params,
            weight,
            false,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_multi_asset_mode_request_serialization() {
        let request = ChangeMultiAssetModeRequest {
            multi_assets_margin: true,
            recv_window: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "multiAssetsMargin=true");
    }

    #[test]
    fn test_change_multi_asset_mode_request_with_recv_window() {
        let request = ChangeMultiAssetModeRequest {
            multi_assets_margin: false,
            recv_window: Some(5000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("multiAssetsMargin=false"));
        assert!(serialized.contains("recvWindow=5000"));
    }

    #[test]
    fn test_get_multi_asset_mode_request_serialization() {
        let request = GetMultiAssetModeRequest {
            recv_window: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_get_multi_asset_mode_request_with_recv_window() {
        let request = GetMultiAssetModeRequest {
            recv_window: Some(3000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "recvWindow=3000");
    }

    #[test]
    fn test_change_multi_asset_mode_response_deserialization() {
        let json = r#"{
            "code": 200,
            "msg": "success"
        }"#;

        let response: ChangeMultiAssetModeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, 200);
        assert_eq!(response.msg, "success");
    }

    #[test]
    fn test_get_multi_asset_mode_response_deserialization() {
        let json = r#"{
            "multiAssetsMargin": true
        }"#;

        let response: GetMultiAssetModeResponse = serde_json::from_str(json).unwrap();
        assert!(response.multi_assets_margin);
    }

    #[test]
    fn test_get_multi_asset_mode_response_false() {
        let json = r#"{
            "multiAssetsMargin": false
        }"#;

        let response: GetMultiAssetModeResponse = serde_json::from_str(json).unwrap();
        assert!(!response.multi_assets_margin);
    }
}
