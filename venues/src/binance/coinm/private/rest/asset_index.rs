use serde::{Deserialize, Serialize};

use crate::binance::{
    coinm::{RestResult, private::rest::client::RestClient},
    shared,
};

/// Request parameters for asset index.
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AssetIndexRequest {
    /// Trading symbol (optional, all symbols returned if not provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// This parameter cannot be used in combination with other parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Base asset information in the asset index.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseAssetList {
    /// Base asset
    pub base_asset: String,

    /// Weight in percentage
    pub weight_in_percentage: String,

    /// Weight in quantity
    pub weight_in_quantity: String,
}

/// Response for asset index.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetIndexResponse {
    /// Symbol
    pub symbol: String,

    /// Time
    pub time: u64,

    /// Base asset list
    pub base_asset_list: Vec<BaseAssetList>,
}

impl RestClient {
    /// Get asset index on Binance Coin-M Futures.
    ///
    /// See: <https://binance-docs.github.io/apidocs/delivery/en/>
    /// GET /dapi/v1/assetIndex
    /// Weight: 10
    /// Requires API key.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`AssetIndexRequest`])
    ///
    /// # Returns
    /// A list of [`AssetIndexResponse`] objects with asset index information.
    pub async fn get_asset_index(
        &self,
        params: AssetIndexRequest,
    ) -> RestResult<Vec<AssetIndexResponse>> {
        shared::send_signed_request(
            self,
            "/dapi/v1/assetIndex",
            reqwest::Method::GET,
            params,
            10,
            false,
        )
        .await
    }
}
