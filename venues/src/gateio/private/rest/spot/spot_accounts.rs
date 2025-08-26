use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const SPOT_ACCOUNTS_ENDPOINT: &str = "/spot/accounts";

/// Request parameters for listing spot accounts.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListSpotAccountsRequest {
    /// Currency filter for retrieving balance of a specific currency (e.g., "BTC", "ETH").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

/// Represents a spot trading account balance for a specific currency.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotAccount {
    /// Currency symbol (e.g., "BTC", "ETH", "USDT").
    pub currency: String,

    /// Available balance amount that can be used for trading or withdrawal.
    pub available: String,

    /// Locked balance amount that is currently reserved in open orders or pending operations.
    pub locked: String,
}

impl RestClient {
    /// List Spot Trading Accounts
    ///
    /// Retrieve balances for all currencies or filter by specific currency.
    /// Returns available and locked amounts for each currency.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#list-spot-accounts)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `req` - Optional request parameters including currency filter
    ///
    /// # Returns
    /// List of spot account balances with available and locked amounts
    pub async fn get_spot_accounts(
        &self,
        req: Option<ListSpotAccountsRequest>,
    ) -> RestResult<Vec<SpotAccount>> {
        self.send_get_request(SPOT_ACCOUNTS_ENDPOINT, req.as_ref())
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_list_spot_accounts_request_no_currency() {
        let request = ListSpotAccountsRequest { currency: None };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }
}
