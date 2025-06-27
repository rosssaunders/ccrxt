use crate::binance::coinm::RestResult;
use crate::binance::coinm::enums::IncomeType;
use crate::binance::coinm::private::rest::client::RestClient;
use crate::binance::shared;
use serde::{Deserialize, Serialize};

/// Request parameters for income history.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct IncomeHistoryRequest {
    /// Trading symbol, e.g. BTCUSD_PERP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Income type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub income_type: Option<IncomeType>,

    /// Timestamp in ms to get income starting from INCLUSIVE
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// Timestamp in ms to get income ending from INCLUSIVE
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Default 100; max 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// This parameter cannot be used in combination with other parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Response for income history.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncomeHistoryEntry {
    /// Symbol
    pub symbol: String,

    /// Income type
    pub income_type: IncomeType,

    /// Income amount
    pub income: String,

    /// Asset
    pub asset: String,

    /// Income info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,

    /// Time
    pub time: u64,

    /// Transaction id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tran_id: Option<u64>,

    /// Trade id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,
}

impl RestClient {
    /// Get income history on Binance Coin-M Futures.
    ///
    /// See: <https://binance-docs.github.io/apidocs/delivery/en/>
    /// GET /dapi/v1/income
    /// Weight: 20
    /// Requires API key and signature.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`IncomeHistoryRequest`])
    ///
    /// # Returns
    /// A list of [`IncomeHistoryEntry`] objects with income history details.
    pub async fn get_income_history(
        &self,
        params: IncomeHistoryRequest,
    ) -> RestResult<Vec<IncomeHistoryEntry>> {
        let weight = 20;
        shared::send_signed_request(
            self,
            "/dapi/v1/income",
            reqwest::Method::GET,
            params,
            weight,
            false,
        )
        .await
    }
}
