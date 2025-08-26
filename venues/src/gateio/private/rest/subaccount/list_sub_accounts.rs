use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const SUB_ACCOUNTS_ENDPOINT: &str = "/sub_accounts";

/// List sub-accounts request parameters
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListSubAccountsRequest {
    /// Filter sub-account types: 0 for all, 1 for regular sub-accounts
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub account_type: Option<i32>,
}

/// Sub-account information
#[derive(Debug, Clone, Deserialize)]
pub struct SubAccount {
    /// Sub-account user ID
    pub user_id: String,

    /// Login name
    pub login_name: String,

    /// Account status
    pub status: String,

    /// Creation timestamp
    pub create_time: i64,

    /// Remark or note
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,

    /// Account type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub account_type: Option<i32>,
}

impl RestClient {
    /// List Sub-Accounts
    ///
    /// Retrieve a list of all sub-accounts under the main account with optional filtering.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#list-sub-accounts)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `req` - Optional request parameters for filtering sub-accounts
    ///
    /// # Returns
    /// List of sub-account information including user IDs, login names, and status
    pub async fn list_sub_accounts(
        &self,
        req: Option<ListSubAccountsRequest>,
    ) -> RestResult<Vec<SubAccount>> {
        self.send_get_request(SUB_ACCOUNTS_ENDPOINT, req.as_ref()).await
    }
}