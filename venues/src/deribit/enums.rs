use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Currency types supported by Deribit
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Currency {
    #[serde(rename = "BTC")]
    BTC,
    #[serde(rename = "ETH")]
    ETH,
    #[serde(rename = "USDC")]
    USDC,
    #[serde(rename = "USDT")]
    USDT,
    #[serde(rename = "EURR")]
    EURR,
}

impl Display for Currency {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Currency::BTC => write!(f, "BTC"),
            Currency::ETH => write!(f, "ETH"),
            Currency::USDC => write!(f, "USDC"),
            Currency::USDT => write!(f, "USDT"),
            Currency::EURR => write!(f, "EURR"),
        }
    }
}

/// Combo state types for filtering
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ComboState {
    #[serde(rename = "rfq")]
    RFQ,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
}

impl Display for ComboState {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ComboState::RFQ => write!(f, "rfq"),
            ComboState::Active => write!(f, "active"),
            ComboState::Inactive => write!(f, "inactive"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_serialization() {
        let btc = Currency::BTC;
        let eth = Currency::ETH;

        assert_eq!(serde_json::to_string(&btc).unwrap(), "\"BTC\"");
        assert_eq!(serde_json::to_string(&eth).unwrap(), "\"ETH\"");

        let btc_from_json: Currency = serde_json::from_str("\"BTC\"").unwrap();
        let eth_from_json: Currency = serde_json::from_str("\"ETH\"").unwrap();

        assert_eq!(btc_from_json, Currency::BTC);
        assert_eq!(eth_from_json, Currency::ETH);
    }

    #[test]
    fn test_combo_state_serialization() {
        let rfq = ComboState::RFQ;
        let active = ComboState::Active;

        assert_eq!(serde_json::to_string(&rfq).unwrap(), "\"rfq\"");
        assert_eq!(serde_json::to_string(&active).unwrap(), "\"active\"");

        let rfq_from_json: ComboState = serde_json::from_str("\"rfq\"").unwrap();
        let active_from_json: ComboState = serde_json::from_str("\"active\"").unwrap();

        assert_eq!(rfq_from_json, ComboState::RFQ);
        assert_eq!(active_from_json, ComboState::Active);
    }

    #[test]
    fn test_currency_display() {
        assert_eq!(format!("{}", Currency::BTC), "BTC");
        assert_eq!(format!("{}", Currency::ETH), "ETH");
        assert_eq!(format!("{}", Currency::USDC), "USDC");
        assert_eq!(format!("{}", Currency::USDT), "USDT");
        assert_eq!(format!("{}", Currency::EURR), "EURR");
    }

    #[test]
    fn test_combo_state_display() {
        assert_eq!(format!("{}", ComboState::RFQ), "rfq");
        assert_eq!(format!("{}", ComboState::Active), "active");
        assert_eq!(format!("{}", ComboState::Inactive), "inactive");
    }
}