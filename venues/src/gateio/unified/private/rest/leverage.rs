use serde::{Deserialize, Serialize};

/// Leverage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeverageConfig {
    /// Currency
    pub currency: String,

    /// Maximum leverage
    pub max_leverage: String,

    /// Minimum size
    pub min_size: String,

    /// Maximum size
    pub max_size: String,

    /// Maintenance margin rate
    pub maintenance_rate: String,
}

/// Request to set leverage
#[derive(Debug, Clone, Serialize)]
pub struct SetLeverageConfigRequest {
    /// Currency
    pub currency: String,

    /// Leverage
    pub leverage: String,
}