use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const WITHDRAWALS_ENDPOINT: &str = "/withdrawal/withdrawals";

/// Query withdrawals
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListWithdrawalsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

/// Withdrawal record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawalRecord {
    pub id: String,

    pub currency: String,

    pub amount: String,

    pub address: String,

    pub status: String,

    pub timestamp: i64,
}

impl RestClient {
    /// List withdrawals
    pub async fn list_withdrawals(
        &self,
        query: ListWithdrawalsQuery,
    ) -> RestResult<Vec<WithdrawalRecord>> {
        self.send_get_request(WITHDRAWALS_ENDPOINT, Some(&query))
            .await
    }
}
