use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for futures contracts
#[derive(Debug, Clone, Serialize, Default)]
pub struct FuturesContractsRequest {
    /// Settlement currency
    pub settle: String,
}

/// Request parameters for single futures contract
#[derive(Debug, Clone, Serialize, Default)]
pub struct FuturesContractRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name
    pub contract: String,
}

/// Futures contract information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesContract {
    /// Futures contract
    pub name: String,

    /// Underlying
    pub underlying: Option<String>,

    /// Quote currency
    pub quote_currency: Option<String>,

    /// Settlement currency
    pub settle_currency: Option<String>,

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

    /// Funding rate
    pub funding_rate: String,

    /// Funding interval (in seconds)
    pub funding_interval: i64,

    /// Next funding time
    pub funding_next_apply: i64,

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
    pub in_delisting: Option<bool>,

    /// Total traded volume in quote currency
    pub orders_limit: i32,

    /// Whether inverse contract
    pub enable_bonus: Option<bool>,

    /// Enable credit trading
    pub enable_credit: Option<bool>,

    /// Create time
    pub create_time: Option<f64>,

    /// Funding rate high limit
    pub funding_cap: Option<String>,

    /// Funding rate low limit  
    pub funding_floor: Option<String>,
}

impl RestClient {
    /// List all futures contracts
    ///
    /// Retrieves all available futures contracts for the specified settlement currency.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#list-all-futures-contracts>
    pub async fn get_futures_contracts(
        &self,
        params: FuturesContractsRequest,
    ) -> crate::gateio::perpetual::Result<Vec<FuturesContract>> {
        let endpoint = format!("/futures/{}/contracts", params.settle);
        self.get(&endpoint).await
    }

    /// Get a single futures contract
    ///
    /// Retrieves detailed information about a specific futures contract.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#get-a-single-contract>
    pub async fn get_futures_contract(
        &self,
        params: FuturesContractRequest,
    ) -> crate::gateio::perpetual::Result<FuturesContract> {
        let endpoint = format!("/futures/{}/contracts/{}", params.settle, params.contract);
        self.get(&endpoint).await
    }
}
