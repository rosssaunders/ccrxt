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
    pub async fn get_options_underlyings(&self) -> crate::gateio::Result<Vec<OptionsUnderlying>> {
        self.get("/options/underlyings").await
    }
}
