use serde::{Deserialize, Serialize};

use crate::binance::{
    coinm::{RestResult, enums::DownloadStatus, private::rest::client::RestClient},
    shared,
};

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

/// Request parameters for getting download link.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDownloadLinkRequest {
    /// Download ID
    pub download_id: String,

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

/// Response for getting download link.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDownloadLinkResponse {
    /// Download ID
    pub download_id: String,

    /// Download status
    pub status: DownloadStatus,

    /// Download URL (only available when status is completed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// The link will expire after this timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_timestamp: Option<u64>,
}

impl RestClient {
    /// Get download ID for transaction history on Binance Coin-M Futures.
    ///
    /// See: <https://binance-docs.github.io/apidocs/delivery/en/>
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
        shared::send_signed_request(
            self,
            "/dapi/v1/income/asyn",
            reqwest::Method::GET,
            params,
            weight,
            false,
        )
        .await
    }

    /// Get download link for transaction history on Binance Coin-M Futures.
    ///
    /// See: <https://binance-docs.github.io/apidocs/delivery/en/>
    /// GET /dapi/v1/income/asyn/id
    /// Weight: 5
    /// Requires API key and signature.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`GetDownloadLinkRequest`])
    ///
    /// # Returns
    /// A [`GetDownloadLinkResponse`] object with download link details.
    pub async fn get_download_link_for_transaction_history(
        &self,
        params: GetDownloadLinkRequest,
    ) -> RestResult<GetDownloadLinkResponse> {
        let weight = 5;
        shared::send_signed_request(
            self,
            "/dapi/v1/income/asyn/id",
            reqwest::Method::GET,
            params,
            weight,
            false,
        )
        .await
    }
}
