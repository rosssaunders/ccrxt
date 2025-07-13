use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for listing spot accounts
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListSpotAccountsRequest {
    /// Retrieve data of the specified currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

/// Spot account balance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotAccount {
    /// Currency name
    pub currency: String,
    /// Available balance
    pub available: String,
    /// Locked balance
    pub locked: String,
}

/// Implementation for the client
impl RestClient {
    /// List spot accounts
    ///
    /// This endpoint returns all spot account balances or a specific currency balance.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#list-spot-accounts>
    pub async fn list_spot_accounts(
        &self,
        currency: Option<&str>,
    ) -> crate::gateio::spotandmargin::Result<Vec<SpotAccount>> {
        let request = ListSpotAccountsRequest {
            currency: currency.map(|s| s.to_string()),
        };

        if currency.is_some() {
            self.get_with_query("/spot/accounts", &request).await
        } else {
            self.get("/spot/accounts").await
        }
    }
}
