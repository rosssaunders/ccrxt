use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const MARGIN_FUNDING_ACCOUNTS_ENDPOINT: &str = "/margin/funding_accounts";

/// Request parameters for querying funding accounts
#[derive(Debug, Clone, Serialize, Default)]
pub struct FundingAccountsRequest {
    /// Specific currency filter (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

/// Funding account information
#[derive(Debug, Clone, Deserialize)]
pub struct FundingAccount {
    /// Currency code
    pub currency: String,

    /// Available balance for lending
    pub available: String,

    /// Balance locked in lending orders
    pub locked: String,

    /// Total lent amount
    pub lent: String,

    /// Total borrowed amount
    pub total_lent: String,
}

impl RestClient {
    /// Funding Account List
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#funding-accounts)
    pub async fn get_margin_funding_accounts(
        &self,
        req: Option<FundingAccountsRequest>,
    ) -> RestResult<Vec<FundingAccount>> {
        self.send_get_request(MARGIN_FUNDING_ACCOUNTS_ENDPOINT, req.as_ref())
            .await
    }
}
