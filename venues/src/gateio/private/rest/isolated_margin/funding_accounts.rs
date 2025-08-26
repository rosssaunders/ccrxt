use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const MARGIN_FUNDING_ACCOUNTS_ENDPOINT: &str = "/margin/funding_accounts";

#[derive(Debug, Clone, Serialize, Default)]
pub struct FundingAccountsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundingAccount {
    pub currency: String,
    pub available: String,
    pub locked: String,
    pub lent: String,
    pub total_lent: String,
}

impl RestClient {
    pub async fn get_funding_accounts(
        &self,
        params: FundingAccountsRequest,
    ) -> RestResult<Vec<FundingAccount>> {
        self.send_get_request(MARGIN_FUNDING_ACCOUNTS_ENDPOINT, Some(&params))
            .await
    }
}
