use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const SUBACCOUNTS_ENDPOINT: &str = "/subaccount/accounts";

/// Query sub-accounts
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListSubAccountsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Sub-account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubAccountInfo {
    pub uid: String,

    pub note: Option<String>,

    pub status: String,
}

impl RestClient {
    /// List sub-accounts
    pub async fn list_subaccounts(
        &self,
        query: ListSubAccountsQuery,
    ) -> RestResult<Vec<SubAccountInfo>> {
        self.send_get_request(SUBACCOUNTS_ENDPOINT, Some(&query))
            .await
    }
}
