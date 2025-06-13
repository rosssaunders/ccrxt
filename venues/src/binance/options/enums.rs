use serde::{Deserialize, Serialize};
use std::fmt;

/// Options contract type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OptionsContractType {
    Call,
    Put,
}

impl fmt::Display for OptionsContractType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptionsContractType::Call => write!(f, "CALL"),
            OptionsContractType::Put => write!(f, "PUT"),
        }
    }
}

/// Position side (positionSide) for Options
/// Note: Options only supports LONG and SHORT, not BOTH like futures
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OptionsPositionSide {
    Long,
    Short,
}

impl fmt::Display for OptionsPositionSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptionsPositionSide::Long => write!(f, "LONG"),
            OptionsPositionSide::Short => write!(f, "SHORT"),
        }
    }
}

/// Order types (type) for Options
/// Note: Options currently only supports LIMIT orders
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OptionsOrderType {
    Limit,
}

impl fmt::Display for OptionsOrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptionsOrderType::Limit => write!(f, "LIMIT"),
        }
    }
}

/// Order status (status) for Options
/// Note: Options uses different status names than futures
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OptionsOrderStatus {
    Accepted,
    Rejected,
    PartiallyFilled,
    Filled,
    Cancelled,
}

impl fmt::Display for OptionsOrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptionsOrderStatus::Accepted => write!(f, "ACCEPTED"),
            OptionsOrderStatus::Rejected => write!(f, "REJECTED"),
            OptionsOrderStatus::PartiallyFilled => write!(f, "PARTIALLY_FILLED"),
            OptionsOrderStatus::Filled => write!(f, "FILLED"),
            OptionsOrderStatus::Cancelled => write!(f, "CANCELLED"),
        }
    }
}

/// Rate limiters (rateLimitType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitType {
    RequestWeight,
    Orders,
}

impl fmt::Display for RateLimitType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RateLimitType::RequestWeight => write!(f, "REQUEST_WEIGHT"),
            RateLimitType::Orders => write!(f, "ORDERS"),
        }
    }
}

/// Rate limit intervals (interval)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitInterval {
    Minute,
}

impl fmt::Display for RateLimitInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RateLimitInterval::Minute => write!(f, "MINUTE"),
        }
    }
}
