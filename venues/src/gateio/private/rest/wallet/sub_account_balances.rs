use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const SUB_ACCOUNT_BALANCES_ENDPOINT: &str = "/wallet/sub_account_balances";

/// Request parameters for querying sub-account balances
#[derive(Debug, Clone, Serialize, Default)]
pub struct SubAccountBalancesRequest {
    /// Sub-account user ID filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_uid: Option<String>,
}

/// Sub-account balance information
#[derive(Debug, Clone, Deserialize)]
pub struct SubAccountBalance {
    /// Sub-account user ID
    pub sub_uid: String,

    /// Available balance
    pub available: String,

    /// Frozen balance
    pub freeze: String,

    /// Currency
    pub currency: String,
}

impl RestClient {
    /// Query Sub-Account Balance Information
    ///
    /// Retrieve balance information for sub-accounts under the main account.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#query-sub-account-balance-information)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `req` - Optional request parameters to filter by sub-account
    ///
    /// # Returns
    /// List of sub-account balance information
    pub async fn get_sub_account_balances(
        &self,
        req: Option<SubAccountBalancesRequest>,
    ) -> RestResult<Vec<SubAccountBalance>> {
        self.send_get_request(SUB_ACCOUNT_BALANCES_ENDPOINT, req.as_ref())
            .await
    }
}
