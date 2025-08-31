use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result, private_client::RestClient};

const DEPOSIT_HISTORY_ENDPOINT: &str = "/api/v1/deposits";

/// Request for getting deposit history
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetDepositsRequest {
    /// Currency filter (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Start time filter (optional, milliseconds)
    #[serde(skip_serializing_if = "Option::is_none", rename = "startAt")]
    pub start_time: Option<i64>,

    /// End time filter (optional, milliseconds)
    #[serde(skip_serializing_if = "Option::is_none", rename = "endAt")]
    pub end_time: Option<i64>,

    /// Status filter (optional): PROCESSING, SUCCESS, FAILURE
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Deposit record
#[derive(Debug, Clone, Deserialize)]
pub struct Deposit {
    /// Deposit address
    pub address: String,

    /// Address memo/tag
    pub memo: Option<String>,

    /// Amount
    pub amount: String,

    /// Fee
    pub fee: String,

    /// Currency
    pub currency: String,

    /// Chain
    pub chain: String,

    /// Wallet transaction ID
    #[serde(rename = "walletTxId")]
    pub wallet_tx_id: String,

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

/// Response wrapper for deposits
#[derive(Debug, Clone, Deserialize)]
pub struct DepositsResponse {
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

    /// Deposit items
    pub items: Vec<Deposit>,
}

impl RestClient {
    /// Get deposit history
    ///
    /// [docs](https://docs.kucoin.com/#get-v1-deposits)
    pub async fn get_deposits(
        &self,
        request: GetDepositsRequest,
    ) -> Result<(DepositsResponse, ResponseHeaders)> {
        let (response, headers): (RestResponse<DepositsResponse>, ResponseHeaders) = self
            .get_with_request(DEPOSIT_HISTORY_ENDPOINT, &request)
            .await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deposits_request_default() {
        let request = GetDepositsRequest::default();
        assert!(request.currency.is_none());
        assert!(request.status.is_none());
    }
}
