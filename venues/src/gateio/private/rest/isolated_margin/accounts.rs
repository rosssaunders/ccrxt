use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const MARGIN_ACCOUNTS_ENDPOINT: &str = "/margin/accounts";

/// Request parameters for querying margin account information
#[derive(Debug, Clone, Serialize, Default)]
pub struct MarginAccountsRequest {
    /// Trading pair filter (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,
}

/// Margin account information
#[derive(Debug, Clone, Deserialize)]
pub struct MarginAccount {
    /// Trading pair identifier
    pub currency_pair: String,

    /// Account lock status
    pub locked: bool,

    /// Risk ratio
    pub risk: String,

    /// Base currency balance details
    pub base: MarginBalance,

    /// Quote currency balance details
    pub quote: MarginBalance,
}

/// Balance information for a currency in margin account
#[derive(Debug, Clone, Deserialize)]
pub struct MarginBalance {
    /// Currency code
    pub currency: String,

    /// Available balance
    pub available: String,

    /// Locked balance
    pub locked: String,

    /// Borrowed amount
    pub borrowed: String,

    /// Interest amount
    pub interest: String,
}

impl RestClient {
    /// Margin Account List
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#margin-accounts)
    pub async fn get_margin_accounts(
        &self,
        req: Option<MarginAccountsRequest>,
    ) -> RestResult<Vec<MarginAccount>> {
        self.send_get_request(MARGIN_ACCOUNTS_ENDPOINT, req.as_ref())
            .await
    }
}
