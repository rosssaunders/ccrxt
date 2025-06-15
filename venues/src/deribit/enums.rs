//! Common enums and types for Deribit API

use serde::{Deserialize, Serialize};

/// Currency types supported by Deribit
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Currency {
    #[serde(rename = "BTC")]
    Btc,
    #[serde(rename = "ETH")]
    Eth,
    #[serde(rename = "USDC")]
    Usdc,
    #[serde(rename = "USDT")]
    Usdt,
    #[serde(rename = "EURR")]
    Eurr,
    #[serde(rename = "any")]
    Any,
}

impl Currency {
    /// Convert to string representation for API calls
    pub fn as_str(&self) -> &'static str {
        match self {
            Currency::Btc => "BTC",
            Currency::Eth => "ETH",
            Currency::Usdc => "USDC",
            Currency::Usdt => "USDT",
            Currency::Eurr => "EURR",
            Currency::Any => "any",
        }
    }
}

/// Combo state as returned by the API
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComboState {
    #[serde(rename = "rfq")]
    Rfq,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_currency_serialization() {
        assert_eq!(serde_json::to_string(&Currency::Btc).unwrap(), "\"BTC\"");
        assert_eq!(serde_json::to_string(&Currency::Any).unwrap(), "\"any\"");
    }

    #[test]
    fn test_currency_deserialization() {
        assert_eq!(serde_json::from_str::<Currency>("\"BTC\"").unwrap(), Currency::Btc);
        assert_eq!(serde_json::from_str::<Currency>("\"any\"").unwrap(), Currency::Any);
    }

    #[test]
    fn test_combo_state_serialization() {
        assert_eq!(serde_json::to_string(&ComboState::Active).unwrap(), "\"active\"");
        assert_eq!(serde_json::to_string(&ComboState::Rfq).unwrap(), "\"rfq\"");
    }
}