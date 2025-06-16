use serde::{Deserialize, Serialize};

/// Account types for ByBit wallet balance requests
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum AccountType {
    /// Spot trading account
    Spot,
    /// Contract trading account (futures/derivatives)
    Contract,
    /// Unified trading account
    Unified,
}

impl std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountType::Spot => write!(f, "SPOT"),
            AccountType::Contract => write!(f, "CONTRACT"),
            AccountType::Unified => write!(f, "UNIFIED"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_type_serialization() {
        assert_eq!(
            serde_json::to_string(&AccountType::Spot).unwrap(),
            "\"SPOT\""
        );
        assert_eq!(
            serde_json::to_string(&AccountType::Contract).unwrap(),
            "\"CONTRACT\""
        );
        assert_eq!(
            serde_json::to_string(&AccountType::Unified).unwrap(),
            "\"UNIFIED\""
        );
    }

    #[test]
    fn test_account_type_display() {
        assert_eq!(AccountType::Spot.to_string(), "SPOT");
        assert_eq!(AccountType::Contract.to_string(), "CONTRACT");
        assert_eq!(AccountType::Unified.to_string(), "UNIFIED");
    }
}
