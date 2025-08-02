use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::options::{OptionsBillType, RestResult};

const GET_ACCOUNT_FUNDING_FLOW_ENDPOINT: &str = "/eapi/v1/bill";

/// Request parameters for querying account funding flows
#[derive(Debug, Clone, Serialize)]
pub struct AccountFundingFlowRequest {
    /// Asset type (currently only "USDT" is supported)
    #[serde(rename = "currency")]
    pub currency: String,

    /// Return records with ID >= this value (latest data by default)
    #[serde(rename = "recordId", skip_serializing_if = "Option::is_none")]
    pub record_id: Option<u64>,

    /// Start time
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Number of result sets returned (default: 100, max: 1000)
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Request timeout window in milliseconds (max 60000)
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Account funding flow record
#[derive(Debug, Clone, Deserialize)]
pub struct FundingFlowRecord {
    /// Record ID
    #[serde(rename = "id")]
    pub id: u64,

    /// Asset type
    #[serde(rename = "asset")]
    pub asset: String,

    /// Amount (positive = inflow, negative = outflow)
    #[serde(rename = "amount")]
    pub amount: Decimal,

    /// Transaction type
    #[serde(rename = "type")]
    pub transaction_type: OptionsBillType,

    /// Creation time
    #[serde(rename = "createDate")]
    pub create_date: u64,
}

impl RestClient {
    /// Query account funding flows
    ///
    /// Returns account funding flow records including fees, contract trades, and transfers.
    ///
    /// [docs]: (https://developers.binance.com/docs/derivatives/option/account/Account-Funding-Flow)
    /// Method: GET /eapi/v1/bill
    /// Weight: 1
    /// Requires: API key and signature
    pub async fn get_account_funding_flow(
        &self,
        params: AccountFundingFlowRequest,
    ) -> RestResult<Vec<FundingFlowRecord>> {
        self.send_get_signed_request(
            GET_ACCOUNT_FUNDING_FLOW_ENDPOINT,
            params,
            1,
            false,
        )
        .await
    }
}
