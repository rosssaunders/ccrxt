use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for delivery contracts
#[derive(Debug, Clone, Serialize, Default)]
pub struct DeliveryContractsRequest {
    /// Settlement currency
    pub settle: String,
}

/// Request parameters for single delivery contract
#[derive(Debug, Clone, Serialize, Default)]
pub struct DeliveryContractRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name
    pub contract: String,
}

/// Delivery contract information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryContract {
    /// Futures contract
    pub name: String,

    /// Underlying
    pub underlying: String,

    /// Cycle: 'WEEK', 'MONTH', 'QUARTER'
    pub cycle: String,

    /// Contract type (inverse, linear)
    #[serde(rename = "type")]
    pub contract_type: String,

    /// Quantitative scale
    pub quanto_multiplier: String,

    /// Leverage amount
    pub leverage_min: String,

    /// Leverage amount
    pub leverage_max: String,

    /// Maintenance rate
    pub maintenance_rate: String,

    /// Mark price
    pub mark_price: String,

    /// Index price
    pub index_price: String,

    /// Last trading time
    pub last_price: String,

    /// Maker fee rate
    pub maker_fee_rate: String,

    /// Taker fee rate  
    pub taker_fee_rate: String,

    /// Value of each contract
    pub order_price_round: String,

    /// Minimum order price increment
    pub mark_price_round: String,

    /// Risk limit base
    pub risk_limit_base: String,

    /// Risk limit step
    pub risk_limit_step: String,

    /// Maximum risk limit  
    pub risk_limit_max: String,

    /// Minimum order size
    pub order_size_min: i64,

    /// Maximum order size
    pub order_size_max: i64,

    /// Order price deviation from current mark price
    pub order_price_deviate: String,

    /// Reference discount rate for buying
    pub ref_discount_rate: String,

    /// Reference rebate rate for selling
    pub ref_rebate_rate: String,

    /// Current orderbook ID
    pub orderbook_id: i64,

    /// Trade ID
    pub trade_id: i64,

    /// Trade size
    pub trade_size: i64,

    /// Position size
    pub position_size: i64,

    /// Configuration change time
    pub config_change_time: f64,

    /// Whether the contract is delisted
    pub in_delisting: bool,

    /// Total traded volume in quote currency
    pub orders_limit: i32,

    /// Whether inverse contract
    pub enable_bonus: bool,

    /// Enable credit trading
    pub enable_credit: bool,

    /// Create time
    pub create_time: f64,

    /// Expiration time
    pub expire_time: i64,

    /// Settlement price
    pub settle_price: String,

    /// Settlement size
    pub settle_size: i64,
}

impl RestClient {
    /// List all delivery contracts
    ///
    /// Retrieves all available delivery contracts for the specified settlement currency.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#list-all-futures-contracts-2>
    pub async fn get_delivery_contracts(
        &self,
        params: DeliveryContractsRequest,
    ) -> crate::gateio::spotandmargin::Result<Vec<DeliveryContract>> {
        let endpoint = format!("/delivery/{}/contracts", params.settle);
        self.get(&endpoint).await
    }

    /// Get a single delivery contract
    ///
    /// Retrieves detailed information about a specific delivery contract.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#get-a-single-contract-2>
    pub async fn get_delivery_contract(
        &self,
        params: DeliveryContractRequest,
    ) -> crate::gateio::spotandmargin::Result<DeliveryContract> {
        let endpoint = format!("/delivery/{}/contracts/{}", params.settle, params.contract);
        self.get(&endpoint).await
    }
}
