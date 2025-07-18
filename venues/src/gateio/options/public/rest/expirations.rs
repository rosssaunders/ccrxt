use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for options expirations
#[derive(Debug, Clone, Serialize, Default)]
pub struct OptionsExpirationsRequest {
    /// Underlying asset
    pub underlying: String,
}

impl RestClient {
    /// List all expiration times
    ///
    /// Retrieves all available expiration times for options contracts.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#list-all-expiration-times>
    /// Returns Unix timestamps.
    pub async fn get_options_expirations(
        &self,
        params: OptionsExpirationsRequest,
    ) -> crate::gateio::options::Result<Vec<i64>> {
        self.get_with_query("/options/expirations", Some(&params))
            .await
    }
}
