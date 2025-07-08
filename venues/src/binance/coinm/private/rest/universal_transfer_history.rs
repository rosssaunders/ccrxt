use serde::{Deserialize, Serialize};

use crate::binance::{
    coinm::{RestResult, enums::TransferType, private::rest::client::RestClient},
    shared,
};

/// Request parameters for universal transfer history.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UniversalTransferHistoryRequest {
    /// Transfer type
    #[serde(rename = "type")]
    pub transfer_type: TransferType,

    /// Start time in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Current page number, default 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<u32>,

    /// Page size, default 10, max 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,

    /// From symbol (for isolated margin)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_symbol: Option<String>,

    /// To symbol (for isolated margin)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_symbol: Option<String>,

    /// This parameter cannot be used in combination with other parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Transfer record in the history.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UniversalTransferHistoryRow {
    /// Asset
    pub asset: String,

    /// Amount
    pub amount: String,

    /// Transfer type
    #[serde(rename = "type")]
    pub transfer_type: String,

    /// Status (CONFIRMED, FAILED, PENDING)
    pub status: String,

    /// Transaction id
    pub tran_id: u64,

    /// Timestamp
    pub timestamp: u64,
}

/// Response for universal transfer history.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UniversalTransferHistoryResponse {
    /// Total records
    pub total: u32,

    /// Transfer records
    pub rows: Vec<UniversalTransferHistoryRow>,
}

impl RestClient {
    /// Get universal transfer history on Binance.
    ///
    /// See: <https://binance-docs.github.io/apidocs/spot/en/>
    /// GET /sapi/v1/asset/transfer
    /// Weight: 1
    /// Requires API key and signature.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`UniversalTransferHistoryRequest`])
    ///
    /// # Returns
    /// A [`UniversalTransferHistoryResponse`] object with transfer history.
    pub async fn get_universal_transfer_history(
        &self,
        params: UniversalTransferHistoryRequest,
    ) -> RestResult<UniversalTransferHistoryResponse> {
        let weight = 1;
        shared::send_signed_request(
            self,
            "/sapi/v1/asset/transfer",
            reqwest::Method::GET,
            params,
            weight,
            false,
        )
        .await
    }
}
