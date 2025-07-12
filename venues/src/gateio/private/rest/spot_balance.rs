// Removed unused Serialize and Deserialize imports

use super::{RestClient, SpotAccount};

impl RestClient {
    /// Get spot account balance for a specific currency
    ///
    /// This endpoint returns the spot account balance for a given currency.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#list-spot-accounts>
    pub async fn get_spot_account_balance(
        &self,
        currency: &str,
    ) -> crate::gateio::Result<SpotAccount> {
        let accounts = self.list_spot_accounts(Some(currency)).await?;
        accounts
            .into_iter()
            .find(|acc| acc.currency == currency)
            .ok_or_else(|| {
                crate::gateio::GateIoError::Unknown(format!("Currency {} not found", currency))
            })
    }
}
