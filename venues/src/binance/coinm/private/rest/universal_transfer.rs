use crate::binance::coinm::RestResult;
use crate::binance::coinm::enums::TransferType;
use crate::binance::coinm::private::rest::client::RestClient;
use crate::binance::shared;
use serde::{Deserialize, Serialize};

/// Request parameters for universal transfer.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UniversalTransferRequest {
    /// Transfer type
    #[serde(rename = "type")]
    pub transfer_type: TransferType,

    /// Asset to transfer
    pub asset: String,

    /// Amount to transfer
    pub amount: String,

    /// From symbol (required for isolated margin transfers)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_symbol: Option<String>,

    /// To symbol (required for isolated margin transfers)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_symbol: Option<String>,

    /// This parameter cannot be used in combination with other parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Response for universal transfer.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UniversalTransferResponse {
    /// Transaction id
    pub tran_id: u64,
}

impl RestClient {
    /// Perform universal transfer on Binance.
    ///
    /// See: <https://binance-docs.github.io/apidocs/spot/en/>
    /// POST /sapi/v1/asset/transfer
    /// Weight: 900
    /// Requires API key and signature.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`UniversalTransferRequest`])
    ///
    /// # Returns
    /// A [`UniversalTransferResponse`] object with transaction details.
    pub async fn universal_transfer(
        &self,
        params: UniversalTransferRequest,
    ) -> RestResult<UniversalTransferResponse> {
        let weight = 900;
        shared::send_signed_request(
            self,
            "/sapi/v1/asset/transfer",
            reqwest::Method::POST,
            params,
            weight,
            false,
        )
        .await
    }
}
