use serde::{Deserialize, Serialize};

/// Account types supported by BingX
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountType {
    /// Spot/Fund account
    Spot,
    /// Standard futures account
    StdFutures,
    /// Coin-M perpetual futures account
    CoinMPerp,
    /// USDT-M perpetual futures account
    UsdtMPerp,
    /// Copy trading account
    CopyTrading,
    /// Grid trading account
    Grid,
    /// Wealth management account
    Earn,
    /// C2C trading account
    C2C,
}

impl AccountType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AccountType::Spot => "spot",
            AccountType::StdFutures => "stdFutures",
            AccountType::CoinMPerp => "coinMPerp",
            AccountType::UsdtMPerp => "USDTMPerp",
            AccountType::CopyTrading => "copyTrading",
            AccountType::Grid => "grid",
            AccountType::Earn => "earn",
            AccountType::C2C => "c2c",
        }
    }
}
