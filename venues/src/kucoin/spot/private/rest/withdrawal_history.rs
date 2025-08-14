use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

const WITHDRAWAL_HISTORY_ENDPOINT: &str = "/api/v1/withdrawals";

/// Request for getting withdrawal history
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetWithdrawalsRequest {
    /// Currency filter (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Start time filter (optional, milliseconds)
    #[serde(skip_serializing_if = "Option::is_none", rename = "startAt")]
    pub start_time: Option<i64>,

    /// End time filter (optional, milliseconds)
    #[serde(skip_serializing_if = "Option::is_none", rename = "endAt")]
    pub end_time: Option<i64>,

    /// Status filter (optional): PROCESSING, WALLET_PROCESSING, SUCCESS, FAILURE
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Withdrawal record
#[derive(Debug, Clone, Deserialize)]
pub struct Withdrawal {
    /// Withdrawal ID
    pub id: String,

    /// Withdrawal address
    pub address: String,

    /// Address memo/tag
    pub memo: Option<String>,

    /// Currency
    pub currency: String,

    /// Chain
    pub chain: String,

    /// Amount
    pub amount: String,

    /// Fee
    pub fee: String,

    /// Wallet transaction ID
    #[serde(rename = "walletTxId")]
    pub wallet_tx_id: Option<String>,

    /// Is internal transfer
    #[serde(rename = "isInner")]
    pub is_inner: bool,

    /// Status
    pub status: String,

    /// Remark
    pub remark: Option<String>,

    /// Creation time
    #[serde(rename = "createdAt")]
    pub created_at: i64,

    /// Update time
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
}

/// Response wrapper for withdrawals
#[derive(Debug, Clone, Deserialize)]
pub struct WithdrawalsResponse {
    /// Current page
    #[serde(rename = "currentPage")]
    pub current_page: i32,

    /// Page size
    #[serde(rename = "pageSize")]
    pub page_size: i32,

    /// Total number of records
    #[serde(rename = "totalNum")]
    pub total_num: i32,

    /// Total pages
    #[serde(rename = "totalPage")]
    pub total_page: i32,

    /// Withdrawal items
    pub items: Vec<Withdrawal>,
}

impl RestClient {
    /// Get withdrawal history
    ///
    /// [docs](https://docs.kucoin.com/#get-v1-withdrawals)
    pub async fn get_withdrawals(
        &self,
        request: GetWithdrawalsRequest,
    ) -> Result<(WithdrawalsResponse, ResponseHeaders)> {
        let (response, headers): (RestResponse<WithdrawalsResponse>, ResponseHeaders) = self
            .get_with_request(WITHDRAWAL_HISTORY_ENDPOINT, &request)
            .await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_withdrawals_request_default() {
        let request = GetWithdrawalsRequest::default();
        assert!(request.currency.is_none());
        assert!(request.status.is_none());
    }
}
