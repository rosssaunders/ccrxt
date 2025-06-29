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

/// Product categories for ByBit trading
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    /// Linear derivatives (USDT/USDC perpetual, futures)
    Linear,
    /// Inverse derivatives (coin-margined)
    Inverse,
    /// Spot trading
    Spot,
    /// Options trading
    Option,
}

/// Order side (buy/sell)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

/// Order types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OrderType {
    Market,
    Limit,
}

/// Time in force options
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TimeInForce {
    /// Good Till Cancel
    GTC,
    /// Immediate or Cancel
    IOC,
    /// Fill or Kill
    FOK,
    /// Post Only
    PostOnly,
    /// Retail Price Improvement
    RPI,
}

/// Position index for hedge mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PositionIdx {
    /// One-way mode
    #[serde(rename = "0")]
    OneWay,
    /// Hedge mode Buy side
    #[serde(rename = "1")]
    HedgeBuy,
    /// Hedge mode Sell side
    #[serde(rename = "2")]
    HedgeSell,
}

/// Trigger price types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TriggerBy {
    LastPrice,
    IndexPrice,
    MarkPrice,
}

/// Order filter types for spot trading
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OrderFilter {
    Order,
    #[serde(rename = "tpslOrder")]
    TpSlOrder,
    StopOrder,
}

/// Market unit for spot market orders
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum MarketUnit {
    BaseCoin,
    QuoteCoin,
}

/// Slippage tolerance type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SlippageToleranceType {
    TickSize,
    Percent,
}

/// Self-match prevention types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SmpType {
    None,
    CancelMaker,
    CancelTaker,
    CancelBoth,
}

/// Take profit / Stop loss mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TpSlMode {
    Full,
    Partial,
}

/// Order status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Untriggered,
    Triggered,
    Filled,
    Cancelled,
    Rejected,
    Deactivated,
}

/// Execution types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExecType {
    Trade,
    Funding,
    AdlTrade,
    BustTrade,
    Delivery,
    BlockTrade,
}

/// Stop order types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StopOrderType {
    Stop,
    TrailingStop,
    PartialTrailingStop,
}

/// Kline intervals
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Interval {
    #[serde(rename = "1")]
    Min1,
    #[serde(rename = "3")]
    Min3,
    #[serde(rename = "5")]
    Min5,
    #[serde(rename = "15")]
    Min15,
    #[serde(rename = "30")]
    Min30,
    #[serde(rename = "60")]
    Min60,
    #[serde(rename = "120")]
    Min120,
    #[serde(rename = "240")]
    Min240,
    #[serde(rename = "360")]
    Min360,
    #[serde(rename = "720")]
    Min720,
    #[serde(rename = "D")]
    Day,
    #[serde(rename = "W")]
    Week,
    #[serde(rename = "M")]
    Month,
}

/// Option types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OptionType {
    Call,
    Put,
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
