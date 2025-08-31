use std::fmt::{Display, Formatter, Result as FmtResult};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Currency types supported by Deribit
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Currency {
    BTC,
    ETH,
    STETH,
    ETHW,
    USDC,
    USDT,
    EURR,
    MATIC,
    SOL,
    XRP,
    USYC,
    PAXG,
    BNB,
    USDE,
    USD,
    BUIDL,
    Any,
    Other(String),
}

impl Currency {
    pub fn as_str(&self) -> &str {
        match self {
            Currency::BTC => "BTC",
            Currency::ETH => "ETH",
            Currency::STETH => "STETH",
            Currency::ETHW => "ETHW",
            Currency::USDC => "USDC",
            Currency::USDT => "USDT",
            Currency::EURR => "EURR",
            Currency::MATIC => "MATIC",
            Currency::SOL => "SOL",
            Currency::XRP => "XRP",
            Currency::USYC => "USYC",
            Currency::PAXG => "PAXG",
            Currency::BNB => "BNB",
            Currency::USDE => "USDE",
            Currency::USD => "USD",
            Currency::BUIDL => "BUIDL",
            Currency::Any => "any",
            Currency::Other(s) => s,
        }
    }
}

impl Display for Currency {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}

impl Serialize for Currency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Currency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "BTC" => Currency::BTC,
            "ETH" => Currency::ETH,
            "STETH" => Currency::STETH,
            "ETHW" => Currency::ETHW,
            "USDC" => Currency::USDC,
            "USDT" => Currency::USDT,
            "EURR" => Currency::EURR,
            "MATIC" => Currency::MATIC,
            "SOL" => Currency::SOL,
            "XRP" => Currency::XRP,
            "USYC" => Currency::USYC,
            "PAXG" => Currency::PAXG,
            "BNB" => Currency::BNB,
            "USDE" => Currency::USDE,
            "USD" => Currency::USD,
            "BUIDL" => Currency::BUIDL,
            "any" => Currency::Any,
            _ => Currency::Other(s),
        })
    }
}

/// Currency pair types supported by Deribit
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CurrencyPair {
    BtcUsd,
    EthUsd,
    BtcUsdc,
    EthUsdc,
    EthBtc,
    Other(String),
}

impl CurrencyPair {
    pub fn as_str(&self) -> &str {
        match self {
            CurrencyPair::BtcUsd => "btc_usd",
            CurrencyPair::EthUsd => "eth_usd",
            CurrencyPair::BtcUsdc => "btc_usdc",
            CurrencyPair::EthUsdc => "eth_usdc",
            CurrencyPair::EthBtc => "eth_btc",
            CurrencyPair::Other(s) => s,
        }
    }
}

impl Display for CurrencyPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}

impl Serialize for CurrencyPair {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for CurrencyPair {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "btc_usd" => CurrencyPair::BtcUsd,
            "eth_usd" => CurrencyPair::EthUsd,
            "btc_usdc" => CurrencyPair::BtcUsdc,
            "eth_usdc" => CurrencyPair::EthUsdc,
            "eth_btc" => CurrencyPair::EthBtc,
            _ => CurrencyPair::Other(s),
        })
    }
}

/// Order direction - buy or sell
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderDirection {
    Buy,
    Sell,
    Other(String),
}

impl OrderDirection {
    pub fn as_str(&self) -> &str {
        match self {
            OrderDirection::Buy => "buy",
            OrderDirection::Sell => "sell",
            OrderDirection::Other(s) => s,
        }
    }
}

impl Display for OrderDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}

impl Serialize for OrderDirection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for OrderDirection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "buy" => OrderDirection::Buy,
            "sell" => OrderDirection::Sell,
            _ => OrderDirection::Other(s),
        })
    }
}

/// Order type for different trading strategies
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderType {
    Limit,
    StopLimit,
    TakeLimit,
    Market,
    StopMarket,
    TakeMarket,
    MarketLimit,
    TrailingStop,
    Stop,
    TriggerAll,
    Other(String),
}

impl OrderType {
    pub fn as_str(&self) -> &str {
        match self {
            OrderType::Limit => "limit",
            OrderType::StopLimit => "stop_limit",
            OrderType::TakeLimit => "take_limit",
            OrderType::Market => "market",
            OrderType::StopMarket => "stop_market",
            OrderType::TakeMarket => "take_market",
            OrderType::MarketLimit => "market_limit",
            OrderType::TrailingStop => "trailing_stop",
            OrderType::Stop => "stop",
            OrderType::TriggerAll => "trigger_all",
            OrderType::Other(s) => s,
        }
    }
}

impl Display for OrderType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}

impl Serialize for OrderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for OrderType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "limit" => OrderType::Limit,
            "stop_limit" => OrderType::StopLimit,
            "take_limit" => OrderType::TakeLimit,
            "market" => OrderType::Market,
            "stop_market" => OrderType::StopMarket,
            "take_market" => OrderType::TakeMarket,
            "market_limit" => OrderType::MarketLimit,
            "trailing_stop" => OrderType::TrailingStop,
            "stop" => OrderType::Stop,
            "trigger_all" => OrderType::TriggerAll,
            _ => OrderType::Other(s),
        })
    }
}

/// Time in force options for orders
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimeInForce {
    GoodTilCancelled,
    GoodTilDay,
    FillOrKill,
    ImmediateOrCancel,
    Other(String),
}

impl TimeInForce {
    pub fn as_str(&self) -> &str {
        match self {
            TimeInForce::GoodTilCancelled => "good_til_cancelled",
            TimeInForce::GoodTilDay => "good_til_day",
            TimeInForce::FillOrKill => "fill_or_kill",
            TimeInForce::ImmediateOrCancel => "immediate_or_cancel",
            TimeInForce::Other(s) => s,
        }
    }
}

impl Display for TimeInForce {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}

impl Serialize for TimeInForce {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for TimeInForce {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "good_til_cancelled" => TimeInForce::GoodTilCancelled,
            "good_til_day" => TimeInForce::GoodTilDay,
            "fill_or_kill" => TimeInForce::FillOrKill,
            "immediate_or_cancel" => TimeInForce::ImmediateOrCancel,
            _ => TimeInForce::Other(s),
        })
    }
}

/// Instrument kind for different types of financial instruments
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstrumentKind {
    Future,
    Option,
    Combo,
    Spot,
    FutureCombo,
    OptionCombo,
    Other(String),
}

impl InstrumentKind {
    pub fn as_str(&self) -> &str {
        match self {
            InstrumentKind::Future => "future",
            InstrumentKind::Option => "option",
            InstrumentKind::Combo => "combo",
            InstrumentKind::Spot => "spot",
            InstrumentKind::FutureCombo => "future_combo",
            InstrumentKind::OptionCombo => "option_combo",
            InstrumentKind::Other(s) => s,
        }
    }
}

impl Display for InstrumentKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}

impl Serialize for InstrumentKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for InstrumentKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "future" => InstrumentKind::Future,
            "option" => InstrumentKind::Option,
            "combo" => InstrumentKind::Combo,
            "spot" => InstrumentKind::Spot,
            "future_combo" => InstrumentKind::FutureCombo,
            "option_combo" => InstrumentKind::OptionCombo,
            _ => InstrumentKind::Other(s),
        })
    }
}

/// Order state enum for tracking order status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderState {
    Open,
    Filled,
    Rejected,
    Cancelled,
    Untriggered,
    Archive,
    Other(String),
}

impl OrderState {
    pub fn as_str(&self) -> &str {
        match self {
            OrderState::Open => "open",
            OrderState::Filled => "filled",
            OrderState::Rejected => "rejected",
            OrderState::Cancelled => "cancelled",
            OrderState::Untriggered => "untriggered",
            OrderState::Archive => "archive",
            OrderState::Other(s) => s,
        }
    }
}

impl Display for OrderState {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}

impl Serialize for OrderState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for OrderState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "open" => OrderState::Open,
            "filled" => OrderState::Filled,
            "rejected" => OrderState::Rejected,
            "cancelled" => OrderState::Cancelled,
            "untriggered" => OrderState::Untriggered,
            "archive" => OrderState::Archive,
            _ => OrderState::Other(s),
        })
    }
}

/// Combo state types for filtering
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComboState {
    RFQ,
    Active,
    Inactive,
    Other(String),
}

impl ComboState {
    pub fn as_str(&self) -> &str {
        match self {
            ComboState::RFQ => "rfq",
            ComboState::Active => "active",
            ComboState::Inactive => "inactive",
            ComboState::Other(s) => s,
        }
    }
}

impl Display for ComboState {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}

impl Serialize for ComboState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for ComboState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "rfq" => ComboState::RFQ,
            "active" => ComboState::Active,
            "inactive" => ComboState::Inactive,
            _ => ComboState::Other(s),
        })
    }
}

// AccountTier is defined in rate_limit.rs

/// Future type classification
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FutureType {
    Linear,
    Inverse,
    Other(String),
}

impl FutureType {
    pub fn as_str(&self) -> &str {
        match self {
            FutureType::Linear => "linear",
            FutureType::Inverse => "inverse",
            FutureType::Other(s) => s,
        }
    }
}

impl Serialize for FutureType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for FutureType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "linear" => Ok(FutureType::Linear),
            "inverse" => Ok(FutureType::Inverse),
            _ => Ok(FutureType::Other(s)),
        }
    }
}

/// Option type classification
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptionType {
    Call,
    Put,
    Other(String),
}

impl OptionType {
    pub fn as_str(&self) -> &str {
        match self {
            OptionType::Call => "call",
            OptionType::Put => "put",
            OptionType::Other(s) => s,
        }
    }
}

impl Serialize for OptionType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for OptionType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "call" => Ok(OptionType::Call),
            "put" => Ok(OptionType::Put),
            _ => Ok(OptionType::Other(s)),
        }
    }
}

/// Instrument type classification
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstrumentType {
    Future,
    Option,
    Spot,
    Other(String),
}

impl InstrumentType {
    pub fn as_str(&self) -> &str {
        match self {
            InstrumentType::Future => "future",
            InstrumentType::Option => "option",
            InstrumentType::Spot => "spot",
            InstrumentType::Other(s) => s,
        }
    }
}

impl Serialize for InstrumentType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for InstrumentType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "future" => Ok(InstrumentType::Future),
            "option" => Ok(InstrumentType::Option),
            "spot" => Ok(InstrumentType::Spot),
            _ => Ok(InstrumentType::Other(s)),
        }
    }
}

/// Sorting direction for API queries
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sorting {
    Asc,
    Desc,
    Default,
    Other(String),
}

impl Sorting {
    pub fn as_str(&self) -> &str {
        match self {
            Sorting::Asc => "asc",
            Sorting::Desc => "desc",
            Sorting::Default => "default",
            Sorting::Other(s) => s,
        }
    }
}

impl Serialize for Sorting {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Sorting {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "asc" => Ok(Sorting::Asc),
            "desc" => Ok(Sorting::Desc),
            "default" => Ok(Sorting::Default),
            _ => Ok(Sorting::Other(s)),
        }
    }
}

/// Resolution for chart data
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Resolution {
    #[default]
    Min1,
    Min5,
    Min15,
    Min30,
    Hour1,
    Hour2,
    Hour6,
    Hour12,
    Day1,
    Other(String),
}

impl Resolution {
    pub fn as_str(&self) -> &str {
        match self {
            Resolution::Min1 => "1",
            Resolution::Min5 => "5",
            Resolution::Min15 => "15",
            Resolution::Min30 => "30",
            Resolution::Hour1 => "3600",
            Resolution::Hour2 => "120",
            Resolution::Hour6 => "360",
            Resolution::Hour12 => "720",
            Resolution::Day1 => "1D",
            Resolution::Other(s) => s,
        }
    }
}

impl Serialize for Resolution {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Resolution {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "1" => Ok(Resolution::Min1),
            "5" => Ok(Resolution::Min5),
            "15" => Ok(Resolution::Min15),
            "30" => Ok(Resolution::Min30),
            "3600" => Ok(Resolution::Hour1),
            "120" => Ok(Resolution::Hour2),
            "360" => Ok(Resolution::Hour6),
            "720" => Ok(Resolution::Hour12),
            "1D" => Ok(Resolution::Day1),
            _ => Ok(Resolution::Other(s)),
        }
    }
}

/// Platform lock status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlatformLockStatus {
    Open,
    Locked,
    PartialLocked,
    AllLocked,
    Unlocked,
    Other(String),
}

impl PlatformLockStatus {
    pub fn as_str(&self) -> &str {
        match self {
            PlatformLockStatus::Open => "false",
            PlatformLockStatus::Locked => "locked",
            PlatformLockStatus::PartialLocked => "partial",
            PlatformLockStatus::AllLocked => "true",
            PlatformLockStatus::Unlocked => "false",
            PlatformLockStatus::Other(s) => s,
        }
    }
}

impl Serialize for PlatformLockStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for PlatformLockStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "false" => Ok(PlatformLockStatus::Open),
            "locked" => Ok(PlatformLockStatus::Locked),
            "partial" => Ok(PlatformLockStatus::PartialLocked),
            "true" => Ok(PlatformLockStatus::AllLocked),
            "open" => Ok(PlatformLockStatus::Unlocked),
            _ => Ok(PlatformLockStatus::Other(s)),
        }
    }
}

impl std::fmt::Display for PlatformLockStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Clearance state for deposits
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClearanceState {
    Success,
    Pending,
    Failed,
    InProgress,
    PendingAdminDecision,
    PendingUserInput,
    Cancelled,
    RefundInitiated,
    Refunded,
    Other(String),
}

impl ClearanceState {
    pub fn as_str(&self) -> &str {
        match self {
            ClearanceState::Success => "success",
            ClearanceState::Pending => "pending",
            ClearanceState::Failed => "failed",
            ClearanceState::InProgress => "in_progress",
            ClearanceState::PendingAdminDecision => "pending_admin_decision",
            ClearanceState::PendingUserInput => "pending_user_input",
            ClearanceState::Cancelled => "cancelled",
            ClearanceState::RefundInitiated => "refund_initiated",
            ClearanceState::Refunded => "refunded",
            ClearanceState::Other(s) => s,
        }
    }
}

impl Serialize for ClearanceState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for ClearanceState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "success" => Ok(ClearanceState::Success),
            "pending" => Ok(ClearanceState::Pending),
            "failed" => Ok(ClearanceState::Failed),
            "in_progress" => Ok(ClearanceState::InProgress),
            "pending_admin_decision" => Ok(ClearanceState::PendingAdminDecision),
            "pending_user_input" => Ok(ClearanceState::PendingUserInput),
            "cancelled" => Ok(ClearanceState::Cancelled),
            "refund_initiated" => Ok(ClearanceState::RefundInitiated),
            "refunded" => Ok(ClearanceState::Refunded),
            _ => Ok(ClearanceState::Other(s)),
        }
    }
}

/// Address book entry type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressBookType {
    Transfer,
    Withdrawal,
    DepositSource,
    Other(String),
}

impl AddressBookType {
    pub fn as_str(&self) -> &str {
        match self {
            AddressBookType::Transfer => "transfer",
            AddressBookType::Withdrawal => "withdrawal",
            AddressBookType::DepositSource => "deposit_source",
            AddressBookType::Other(s) => s,
        }
    }
}

impl Serialize for AddressBookType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for AddressBookType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "transfer" => Ok(AddressBookType::Transfer),
            "withdrawal" => Ok(AddressBookType::Withdrawal),
            "deposit_source" => Ok(AddressBookType::DepositSource),
            _ => Ok(AddressBookType::Other(s)),
        }
    }
}

/// Address status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressStatus {
    Active,
    Inactive,
    Confirmed,
    Waiting,
    Other(String),
}

impl AddressStatus {
    pub fn as_str(&self) -> &str {
        match self {
            AddressStatus::Active => "active",
            AddressStatus::Inactive => "inactive",
            AddressStatus::Confirmed => "confirmed",
            AddressStatus::Waiting => "waiting",
            AddressStatus::Other(s) => s,
        }
    }
}

impl Serialize for AddressStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for AddressStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "active" => Ok(AddressStatus::Active),
            "inactive" => Ok(AddressStatus::Inactive),
            "confirmed" => Ok(AddressStatus::Confirmed),
            "waiting" => Ok(AddressStatus::Waiting),
            _ => Ok(AddressStatus::Other(s)),
        }
    }
}

/// Advanced order type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdvancedType {
    Usd,
    Implv,
    Other(String),
}

impl AdvancedType {
    pub fn as_str(&self) -> &str {
        match self {
            AdvancedType::Usd => "usd",
            AdvancedType::Implv => "implv",
            AdvancedType::Other(s) => s,
        }
    }
}

impl Serialize for AdvancedType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for AdvancedType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "usd" => Ok(AdvancedType::Usd),
            "implv" => Ok(AdvancedType::Implv),
            _ => Ok(AdvancedType::Other(s)),
        }
    }
}

/// Cancel reason for orders
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CancelReason {
    User,
    AutoLiquidation,
    AutoDelever,
    OptionExpiration,
    PositionLiquidation,
    MmpTrigger,
    UserRequest,
    Other(String),
}

impl CancelReason {
    pub fn as_str(&self) -> &str {
        match self {
            CancelReason::User => "user",
            CancelReason::AutoLiquidation => "auto_liquidation",
            CancelReason::AutoDelever => "auto_delever",
            CancelReason::OptionExpiration => "option_expiration",
            CancelReason::PositionLiquidation => "position_liquidation",
            CancelReason::MmpTrigger => "mmp_trigger",
            CancelReason::UserRequest => "user_request",
            CancelReason::Other(s) => s,
        }
    }
}

impl Serialize for CancelReason {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for CancelReason {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "user" => Ok(CancelReason::User),
            "auto_liquidation" => Ok(CancelReason::AutoLiquidation),
            "auto_delever" => Ok(CancelReason::AutoDelever),
            "option_expiration" => Ok(CancelReason::OptionExpiration),
            "position_liquidation" => Ok(CancelReason::PositionLiquidation),
            "mmp_trigger" => Ok(CancelReason::MmpTrigger),
            "user_request" => Ok(CancelReason::UserRequest),
            _ => Ok(CancelReason::Other(s)),
        }
    }
}

/// Trigger type for conditional orders
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TriggerType {
    IndexPrice,
    MarkPrice,
    LastPrice,
    Other(String),
}

impl TriggerType {
    pub fn as_str(&self) -> &str {
        match self {
            TriggerType::IndexPrice => "index_price",
            TriggerType::MarkPrice => "mark_price",
            TriggerType::LastPrice => "last_price",
            TriggerType::Other(s) => s,
        }
    }
}

impl Serialize for TriggerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for TriggerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "index_price" => Ok(TriggerType::IndexPrice),
            "mark_price" => Ok(TriggerType::MarkPrice),
            "last_price" => Ok(TriggerType::LastPrice),
            _ => Ok(TriggerType::Other(s)),
        }
    }
}

/// Deposit state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DepositState {
    Pending,
    Completed,
    Cancelled,
    Rejected,
    Replaced,
    Other(String),
}

impl DepositState {
    pub fn as_str(&self) -> &str {
        match self {
            DepositState::Pending => "pending",
            DepositState::Completed => "completed",
            DepositState::Cancelled => "cancelled",
            DepositState::Rejected => "rejected",
            DepositState::Replaced => "replaced",
            DepositState::Other(s) => s,
        }
    }
}

impl Serialize for DepositState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for DepositState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "pending" => Ok(DepositState::Pending),
            "completed" => Ok(DepositState::Completed),
            "cancelled" => Ok(DepositState::Cancelled),
            "rejected" => Ok(DepositState::Rejected),
            "replaced" => Ok(DepositState::Replaced),
            _ => Ok(DepositState::Other(s)),
        }
    }
}

// EndpointType is defined in rate_limit.rs

/// Order type for trading operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TradeOrderType {
    Limit,
    Market,
    StopLimit,
    StopMarket,
    Other(String),
}

impl TradeOrderType {
    pub fn as_str(&self) -> &str {
        match self {
            TradeOrderType::Limit => "limit",
            TradeOrderType::Market => "market",
            TradeOrderType::StopLimit => "stop_limit",
            TradeOrderType::StopMarket => "stop_market",
            TradeOrderType::Other(s) => s,
        }
    }
}

impl Serialize for TradeOrderType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for TradeOrderType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "limit" => Ok(TradeOrderType::Limit),
            "market" => Ok(TradeOrderType::Market),
            "stop_limit" => Ok(TradeOrderType::StopLimit),
            "stop_market" => Ok(TradeOrderType::StopMarket),
            _ => Ok(TradeOrderType::Other(s)),
        }
    }
}

/// Order type for open orders queries
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenOrdersOrderType {
    Limit,
    Market,
    StopLimit,
    StopMarket,
    Other(String),
}

impl OpenOrdersOrderType {
    pub fn as_str(&self) -> &str {
        match self {
            OpenOrdersOrderType::Limit => "limit",
            OpenOrdersOrderType::Market => "market",
            OpenOrdersOrderType::StopLimit => "stop_limit",
            OpenOrdersOrderType::StopMarket => "stop_market",
            OpenOrdersOrderType::Other(s) => s,
        }
    }
}

impl Serialize for OpenOrdersOrderType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for OpenOrdersOrderType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "limit" => Ok(OpenOrdersOrderType::Limit),
            "market" => Ok(OpenOrdersOrderType::Market),
            "stop_limit" => Ok(OpenOrdersOrderType::StopLimit),
            "stop_market" => Ok(OpenOrdersOrderType::StopMarket),
            _ => Ok(OpenOrdersOrderType::Other(s)),
        }
    }
}

/// Trigger fill condition for orders
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TriggerFillCondition {
    Incremental,
    All,
    Other(String),
}

impl TriggerFillCondition {
    pub fn as_str(&self) -> &str {
        match self {
            TriggerFillCondition::Incremental => "incremental",
            TriggerFillCondition::All => "all",
            TriggerFillCondition::Other(s) => s,
        }
    }
}

impl Serialize for TriggerFillCondition {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for TriggerFillCondition {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "incremental" => Ok(TriggerFillCondition::Incremental),
            "all" => Ok(TriggerFillCondition::All),
            _ => Ok(TriggerFillCondition::Other(s)),
        }
    }
}

/// Liquidation side
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiquidationSide {
    Buy,
    Sell,
    Maker,
    Other(String),
}

impl LiquidationSide {
    pub fn as_str(&self) -> &str {
        match self {
            LiquidationSide::Buy => "buy",
            LiquidationSide::Sell => "sell",
            LiquidationSide::Maker => "maker",
            LiquidationSide::Other(s) => s,
        }
    }
}

impl Serialize for LiquidationSide {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for LiquidationSide {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "buy" => Ok(LiquidationSide::Buy),
            "sell" => Ok(LiquidationSide::Sell),
            "maker" | "M" => Ok(LiquidationSide::Maker),
            _ => Ok(LiquidationSide::Other(s)),
        }
    }
}

/// Liquidity provider/taker side
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Liquidity {
    Maker,
    Taker,
    Other(String),
}

impl Liquidity {
    pub fn as_str(&self) -> &str {
        match self {
            Liquidity::Maker => "M",
            Liquidity::Taker => "T",
            Liquidity::Other(s) => s,
        }
    }
}

impl Serialize for Liquidity {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Liquidity {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "M" => Ok(Liquidity::Maker),
            "T" => Ok(Liquidity::Taker),
            _ => Ok(Liquidity::Other(s)),
        }
    }
}

/// Tick direction for price movements
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TickDirection {
    PlusTick,
    ZeroPlusTick,
    MinusTick,
    ZeroMinusTick,
    Other(String),
}

impl TickDirection {
    pub fn as_str(&self) -> &str {
        match self {
            TickDirection::PlusTick => "+",
            TickDirection::ZeroPlusTick => "0+",
            TickDirection::MinusTick => "-",
            TickDirection::ZeroMinusTick => "0-",
            TickDirection::Other(s) => s,
        }
    }
}

impl Serialize for TickDirection {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for TickDirection {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "+" | "0" => Ok(TickDirection::PlusTick),
            "0+" | "1" => Ok(TickDirection::ZeroPlusTick),
            "-" | "2" => Ok(TickDirection::MinusTick),
            "0-" | "3" => Ok(TickDirection::ZeroMinusTick),
            _ => Ok(TickDirection::Other(s)),
        }
    }
}

/// Withdrawal priority levels
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WithdrawalPriority {
    Low,
    High,
    Other(String),
}

impl WithdrawalPriority {
    pub fn as_str(&self) -> &str {
        match self {
            WithdrawalPriority::Low => "low",
            WithdrawalPriority::High => "high",
            WithdrawalPriority::Other(s) => s,
        }
    }
}

impl Serialize for WithdrawalPriority {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for WithdrawalPriority {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "low" => Ok(WithdrawalPriority::Low),
            "high" => Ok(WithdrawalPriority::High),
            _ => Ok(WithdrawalPriority::Other(s)),
        }
    }
}

/// Withdrawal state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WithdrawalState {
    Unconfirmed,
    Confirmed,
    Cancelled,
    Completed,
    Interrupted,
    Rejected,
    Other(String),
}

impl WithdrawalState {
    pub fn as_str(&self) -> &str {
        match self {
            WithdrawalState::Unconfirmed => "unconfirmed",
            WithdrawalState::Confirmed => "confirmed",
            WithdrawalState::Cancelled => "cancelled",
            WithdrawalState::Completed => "completed",
            WithdrawalState::Interrupted => "interrupted",
            WithdrawalState::Rejected => "rejected",
            WithdrawalState::Other(s) => s,
        }
    }
}

impl Serialize for WithdrawalState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for WithdrawalState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "unconfirmed" => Ok(WithdrawalState::Unconfirmed),
            "confirmed" => Ok(WithdrawalState::Confirmed),
            "cancelled" => Ok(WithdrawalState::Cancelled),
            "completed" => Ok(WithdrawalState::Completed),
            "interrupted" => Ok(WithdrawalState::Interrupted),
            "rejected" => Ok(WithdrawalState::Rejected),
            _ => Ok(WithdrawalState::Other(s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_currency_forward_compatibility() {
        // Test known value
        let btc = serde_json::from_str::<Currency>("\"BTC\"").unwrap();
        assert_eq!(btc, Currency::BTC);

        // Test unknown value preserves string
        let unknown = serde_json::from_str::<Currency>("\"NEW_COIN\"").unwrap();
        assert_eq!(unknown, Currency::Other("NEW_COIN".to_string()));

        // Test round trip
        let serialized = serde_json::to_string(&unknown).unwrap();
        assert_eq!(serialized, "\"NEW_COIN\"");
    }

    #[test]
    fn test_order_direction_forward_compatibility() {
        // Test known value
        let buy = serde_json::from_str::<OrderDirection>("\"buy\"").unwrap();
        assert_eq!(buy, OrderDirection::Buy);

        // Test unknown value preserves string
        let unknown = serde_json::from_str::<OrderDirection>("\"new_direction\"").unwrap();
        assert_eq!(unknown, OrderDirection::Other("new_direction".to_string()));

        // Test round trip
        let serialized = serde_json::to_string(&unknown).unwrap();
        assert_eq!(serialized, "\"new_direction\"");
    }

    #[test]
    fn test_order_type_forward_compatibility() {
        // Test known value
        let limit = serde_json::from_str::<OrderType>("\"limit\"").unwrap();
        assert_eq!(limit, OrderType::Limit);

        // Test unknown value preserves string
        let unknown = serde_json::from_str::<OrderType>("\"new_order_type\"").unwrap();
        assert_eq!(unknown, OrderType::Other("new_order_type".to_string()));

        // Test round trip
        let serialized = serde_json::to_string(&unknown).unwrap();
        assert_eq!(serialized, "\"new_order_type\"");
    }
}
