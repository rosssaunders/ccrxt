use serde::{Deserialize, Serialize};

use super::RestClient;

/// Options underlying asset information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsUnderlying {
    /// Underlying name
    pub name: String,

    /// Spot index price (quote currency)
    pub index_price: String,
}

impl RestClient {
    /// List all underlyings
    ///
    /// Retrieves all available underlying assets for options trading.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#list-all-underlyings>
    pub async fn get_options_underlyings(
        &self,
    ) -> crate::gateio::spotandmargin::Result<Vec<OptionsUnderlying>> {
        self.get("/options/underlyings").await
    }
}
