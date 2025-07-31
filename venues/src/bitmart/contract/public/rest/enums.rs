//! Enums for BitMart Futures public endpoints
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// Contract status as returned by BitMart API
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumString, Display)]
pub enum ContractStatus {
    /// Normal trading status (API returns "Trading")
    #[serde(rename = "Trading")]
    Normal,
    
    /// Delisted status
    #[serde(rename = "Delisted")]
    Delisted,
    
    /// Unknown status for any other values
    #[serde(other)]
    Unknown,
}
