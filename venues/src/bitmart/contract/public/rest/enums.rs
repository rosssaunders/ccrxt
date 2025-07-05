//! Enums for BitMart Futures public endpoints
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// Contract status as returned by BitMart API
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumString, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContractStatus {
    #[serde(rename = "NORMAL")]
    Normal,
    #[serde(rename = "DELISTED")]
    Delisted,
    #[serde(other)]
    Unknown,
}
