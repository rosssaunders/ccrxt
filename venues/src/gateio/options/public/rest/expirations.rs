// Removed unused Serialize and Deserialize imports

use super::RestClient;

impl RestClient {
    /// List all expiration times
    ///
    /// Retrieves all available expiration times for options contracts.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#list-all-expiration-times>
    /// Returns Unix timestamps.
    pub async fn get_options_expirations(&self) -> crate::gateio::options::Result<Vec<i64>> {
        self.get("/options/expirations").await
    }
}
