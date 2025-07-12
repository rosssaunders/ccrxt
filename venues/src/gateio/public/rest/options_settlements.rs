use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for options settlements
#[derive(Debug, Clone, Serialize, Default)]
pub struct OptionsSettlementsRequest {
    /// Underlying asset filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,

    /// Maximum number of records to return (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Options settlement information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsSettlement {
    /// Settlement time
    pub time: i64,

    /// Contract name
    pub contract: String,

    /// Underlying asset
    pub underlying: String,

    /// Strike price (quote currency)
    pub strike_price: String,

    /// Settlement price (quote currency)
    pub settle_price: String,
}

impl RestClient {
    /// List settlement history
    ///
    /// Retrieves settlement history for options contracts.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#list-settlement-history-2>
    pub async fn get_options_settlements(
        &self,
        params: OptionsSettlementsRequest,
    ) -> crate::gateio::Result<Vec<OptionsSettlement>> {
        self.get_with_query("/options/settlements", Some(&params))
            .await
    }

    /// Get specified contract's settlement
    ///
    /// Retrieves settlement information for a specific options contract.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#get-specified-contract-s-settlement>
    pub async fn get_options_contract_settlement(
        &self,
        contract: &str,
    ) -> crate::gateio::Result<OptionsSettlement> {
        let endpoint = format!("/options/settlements/{}", contract);
        self.get(&endpoint).await
    }
}
