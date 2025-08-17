use serde::{Deserialize, Serialize};

/// AMM Instruction entity
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmmInstruction {
    #[serde(rename = "instructionId")]
    pub instruction_id: String,

    /// Some responses may use liquidityId for backward compatibility
    #[serde(rename = "liquidityId", default)]
    pub liquidity_id: Option<String>,

    /// Market symbol
    pub symbol: String,

    /// Trading account ID
    #[serde(rename = "tradingAccountId")]
    pub trading_account_id: String,

    /// AMM parameters
    #[serde(rename = "lowerBound", default)]
    pub lower_bound: Option<String>,

    #[serde(rename = "upperBound", default)]
    pub upper_bound: Option<String>,

    #[serde(rename = "feeTierId", default)]
    pub fee_tier_id: Option<String>,

    /// Status string (e.g., OPEN, CLOSED)
    #[serde(default)]
    pub status: Option<String>,

    /// Metrics and amounts
    #[serde(rename = "apy", default)]
    pub apy: Option<String>,

    #[serde(rename = "24HrApy", default)]
    pub apy_24h: Option<String>,

    #[serde(rename = "24HrYieldEarn", default)]
    pub yield_24h: Option<String>,

    #[serde(rename = "baseCurrentQuantity", default)]
    pub base_current_quantity: Option<String>,

    #[serde(rename = "quoteCurrentQuantity", default)]
    pub quote_current_quantity: Option<String>,

    #[serde(rename = "baseWithdrawQuantity", default)]
    pub base_withdraw_quantity: Option<String>,

    #[serde(rename = "quoteWithdrawQuantity", default)]
    pub quote_withdraw_quantity: Option<String>,

    #[serde(rename = "currentValue", default)]
    pub current_value: Option<String>,

    #[serde(rename = "finalValue", default)]
    pub final_value: Option<String>,

    #[serde(rename = "initialValue", default)]
    pub initial_value: Option<String>,

    #[serde(rename = "initialBasePrice", default)]
    pub initial_base_price: Option<String>,

    #[serde(rename = "initialQuotePrice", default)]
    pub initial_quote_price: Option<String>,

    #[serde(rename = "lastDistributedPrice", default)]
    pub last_distributed_price: Option<String>,

    /// Timestamps
    #[serde(rename = "createdAtDatetime", default)]
    pub created_at_datetime: Option<String>,

    #[serde(rename = "createdAtTimestamp", default)]
    pub created_at_timestamp: Option<String>,

    #[serde(rename = "updatedAtDatetime", default)]
    pub updated_at_datetime: Option<String>,

    #[serde(rename = "updatedAtTimestamp", default)]
    pub updated_at_timestamp: Option<String>,
}
