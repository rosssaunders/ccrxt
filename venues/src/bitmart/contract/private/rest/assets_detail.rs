use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bitmart::RestResult;

/// Request parameters for the BitMart "Get Contract Assets" endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetContractAssetsRequest;

/// Response from the BitMart "Get Contract Assets" endpoint.
///
/// Contains the result code, message, asset data, and trace ID.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetContractAssetsResponse {
    /// API response code (e.g., 1000 for success)
    #[serde(rename = "code")]
    pub code: i32,

    /// API response message (e.g., "Ok")
    #[serde(rename = "message")]
    pub message: Cow<'static, str>,

    /// List of contract asset details for each currency
    #[serde(rename = "data")]
    pub data: Option<Vec<ContractAsset>>,

    /// Trace ID for debugging and support
    #[serde(rename = "trace")]
    pub trace: Option<Cow<'static, str>>,
}

/// Details of a single contract asset for a given currency.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractAsset {
    /// Currency code (e.g., "USDT", "BTC", "ETH")
    #[serde(rename = "currency")]
    pub currency: Cow<'static, str>,

    /// Position margin (as a string, e.g., "100")
    #[serde(rename = "position_deposit")]
    pub position_deposit: Cow<'static, str>,

    /// Transaction freeze amount (as a string)
    #[serde(rename = "frozen_balance")]
    pub frozen_balance: Cow<'static, str>,

    /// Available amount (as a string)
    #[serde(rename = "available_balance")]
    pub available_balance: Cow<'static, str>,

    /// Total equity (as a string)
    #[serde(rename = "equity")]
    pub equity: Cow<'static, str>,

    /// Unrealized P&L (as a string)
    #[serde(rename = "unrealized")]
    pub unrealized: Cow<'static, str>,
}

impl RestClient {
    /// Get Contract Assets (KEYED)
    ///
    /// Retrieves user contract asset details for BitMart Futures accounts.
    ///
    /// [docs](https://developer-pro.bitmart.com/en/futuresv2/#get-contract-assets-keyed)
    ///
    /// # Rate Limit
    /// See BitMart API documentation for detailed rate limits.
    ///
    /// # Arguments
    /// * `_req` - Request parameters (none required for this endpoint)
    ///
    /// # Returns
    /// * [`GetContractAssetsResponse`] - The contract asset details for the user
    pub async fn get_contract_assets(
        &self,
        _req: &GetContractAssetsRequest,
    ) -> RestResult<GetContractAssetsResponse> {
        self.send_signed_request("/contract/private/assets-detail")
            .await
    }
}
