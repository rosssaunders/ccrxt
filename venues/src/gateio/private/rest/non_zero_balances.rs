use super::{RestClient, SpotAccount};

impl RestClient {
    /// Get all non-zero spot account balances
    ///
    /// This endpoint returns all spot account balances with non-zero available or locked amounts.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#list-spot-accounts>
    pub async fn get_non_zero_spot_balances(&self) -> crate::gateio::Result<Vec<SpotAccount>> {
        let accounts = self.list_spot_accounts(None).await?;
        Ok(accounts
            .into_iter()
            .filter(|acc| {
                let available: f64 = acc.available.parse().unwrap_or(0.0);
                let locked: f64 = acc.locked.parse().unwrap_or(0.0);
                available > 0.0 || locked > 0.0
            })
            .collect())
    }
}
