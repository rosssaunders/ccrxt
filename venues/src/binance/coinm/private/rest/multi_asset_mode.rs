use serde::{Deserialize, Serialize};

use crate::binance::{
    coinm::{RestResult, private::rest::client::RestClient},
    shared,
};

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
            "/dapi/v1/multiAssetsMargin",
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
            "/dapi/v1/multiAssetsMargin",
            reqwest::Method::GET,
            params,
            weight,
            false,
        )
        .await
    }
}
