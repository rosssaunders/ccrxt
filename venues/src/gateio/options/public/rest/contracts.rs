use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for options contracts
#[derive(Debug, Clone, Serialize, Default)]
pub struct OptionsContractsRequest {
    /// Underlying asset filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,

    /// Expiration time filter (Unix timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<i64>,
}

/// Options contract information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsContract {
    /// Contract name
    pub name: String,

    /// Tag for mark price usage
    pub tag: String,

    /// Creation time
    pub create_time: f64,

    /// Expiration time
    pub expiration_time: i64,

    /// Underlying asset
    pub underlying: String,

    /// Underlying price
    pub underlying_price: Option<String>,

    /// Last trading time
    pub last_price: Option<String>,

    /// Mark price
    pub mark_price: Option<String>,

    /// Index price
    pub index_price: Option<String>,

    /// Mark IV (implied volatility)
    pub mark_iv: Option<String>,

    /// Option type (call/put)
    #[serde(rename = "type")]
    pub option_type: Option<String>,

    /// Strike price
    pub strike_price: String,

    /// Is call option
    pub is_call: bool,

    /// Multiplier
    pub multiplier: String,

    /// Current total long position size
    pub position_size: Option<i64>,

    /// Maximum number of open orders
    pub orders_limit: i32,
}

impl RestClient {
    /// List all the contracts with specified underlying and expiration time
    ///
    /// Retrieves options contracts with optional filtering by underlying and expiration.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#list-all-the-contracts-with-specified-underlying-and-expiration-time>
    pub async fn get_options_contracts(
        &self,
        params: OptionsContractsRequest,
    ) -> crate::gateio::options::Result<Vec<OptionsContract>> {
        self.get_with_query("/options/contracts", Some(&params))
            .await
    }

    /// Query specified contract detail
    ///
    /// Retrieves detailed information for a specific options contract.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#query-specified-contract-detail>
    pub async fn get_options_contract(
        &self,
        contract: &str,
    ) -> crate::gateio::options::Result<OptionsContract> {
        let endpoint = format!("/options/contracts/{}", contract);
        self.get(&endpoint).await
    }
}
