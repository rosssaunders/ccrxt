use serde::{Deserialize, Serialize};

/// Request parameters for unified mode
#[derive(Debug, Clone, Serialize, Default)]
pub struct UnifiedModeRequest {
    /// Mode (true for unified, false for classic)
    pub unified: bool,
}

/// Unified mode response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedModeResponse {
    /// User ID
    pub user_id: i64,

    /// Unified mode status
    pub unified: bool,
}

/// Request parameters for unified currencies
#[derive(Debug, Clone, Serialize, Default)]
pub struct UnifiedCurrenciesRequest {
    /// Currency filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

/// Unified currency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedCurrency {
    /// Currency code
    pub currency: String,

    /// Currency name
    pub name: String,

    /// Delisted status
    pub delisted: bool,

    /// Withdraw disabled
    pub withdraw_disabled: bool,

    /// Withdraw delayed
    pub withdraw_delayed: bool,

    /// Deposit disabled
    pub deposit_disabled: bool,

    /// Trade disabled
    pub trade_disabled: bool,

    /// Fixed rate
    pub fixed_rate: String,

    /// Cross margin supported
    pub cross_margin: bool,

    /// Lendable
    pub lendable: bool,

    /// Borrowable
    pub borrowable: bool,
}
