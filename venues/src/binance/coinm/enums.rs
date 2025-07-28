use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderSide {
    Buy,
    Sell,
}

impl fmt::Display for OrderSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderSide::Buy => write!(f, "BUY"),
            OrderSide::Sell => write!(f, "SELL"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PositionSide {
    Both,
    Long,
    Short,
}

impl fmt::Display for PositionSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PositionSide::Both => write!(f, "BOTH"),
            PositionSide::Long => write!(f, "LONG"),
            PositionSide::Short => write!(f, "SHORT"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Limit,
    Market,
    Stop,
    StopMarket,
    TakeProfit,
    TakeProfitMarket,
    TrailingStopMarket,
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderType::Limit => write!(f, "LIMIT"),
            OrderType::Market => write!(f, "MARKET"),
            OrderType::Stop => write!(f, "STOP"),
            OrderType::StopMarket => write!(f, "STOP_MARKET"),
            OrderType::TakeProfit => write!(f, "TAKE_PROFIT"),
            OrderType::TakeProfitMarket => write!(f, "TAKE_PROFIT_MARKET"),
            OrderType::TrailingStopMarket => write!(f, "TRAILING_STOP_MARKET"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TimeInForce {
    /// Good Till Cancel
    GTC,
    /// Immediate or Cancel
    IOC,
    /// Fill or Kill
    FOK,
    /// Good Till Crossing (Post Only)
    GTX,
}

impl fmt::Display for TimeInForce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeInForce::GTC => write!(f, "GTC"),
            TimeInForce::IOC => write!(f, "IOC"),
            TimeInForce::FOK => write!(f, "FOK"),
            TimeInForce::GTX => write!(f, "GTX"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkingType {
    MarkPrice,
    ContractPrice,
}

impl fmt::Display for WorkingType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorkingType::MarkPrice => write!(f, "MARK_PRICE"),
            WorkingType::ContractPrice => write!(f, "CONTRACT_PRICE"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    /// The order has been accepted by the system but not yet filled.
    New,
    /// The order has been partially filled.
    PartiallyFilled,
    /// The order has been completely filled.
    Filled,
    /// The order has been canceled by the user.
    Canceled,
    /// The order has expired.
    Expired,
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderStatus::New => write!(f, "NEW"),
            OrderStatus::PartiallyFilled => write!(f, "PARTIALLY_FILLED"),
            OrderStatus::Filled => write!(f, "FILLED"),
            OrderStatus::Canceled => write!(f, "CANCELED"),
            OrderStatus::Expired => write!(f, "EXPIRED"),
        }
    }
}

/// Represents the response type for new order requests.
///
/// Can be set to ACK or RESULT. Default is ACK.
/// Used in the newOrderRespType field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderResponseType {
    Ack,
    Result,
}

impl fmt::Display for OrderResponseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderResponseType::Ack => write!(f, "ACK"),
            OrderResponseType::Result => write!(f, "RESULT"),
        }
    }
}

/// Represents the self-trade prevention mode for orders.
///
/// Can be set to NONE, EXPIRE_TAKER, EXPIRE_MAKER, or EXPIRE_BOTH.
/// Used in the selfTradePreventionMode field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SelfTradePreventionMode {
    None,
    ExpireTaker,
    ExpireMaker,
    ExpireBoth,
}

impl fmt::Display for SelfTradePreventionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SelfTradePreventionMode::None => write!(f, "NONE"),
            SelfTradePreventionMode::ExpireTaker => write!(f, "EXPIRE_TAKER"),
            SelfTradePreventionMode::ExpireMaker => write!(f, "EXPIRE_MAKER"),
            SelfTradePreventionMode::ExpireBoth => write!(f, "EXPIRE_BOTH"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IncomeType {
    Transfer,
    WelcomeBonus,
    RealizedPnl,
    FundingFee,
    Commission,
    InsuranceClear,
    ReferralKickback,
    CommissionRebate,
    ApiRebate,
    ContReward,
    UsdVsTokenSettlement,
    FeeReward,
    TokenReward,
    TransferIn,
    TransferOut,
}

impl fmt::Display for IncomeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IncomeType::Transfer => write!(f, "TRANSFER"),
            IncomeType::WelcomeBonus => write!(f, "WELCOME_BONUS"),
            IncomeType::RealizedPnl => write!(f, "REALIZED_PNL"),
            IncomeType::FundingFee => write!(f, "FUNDING_FEE"),
            IncomeType::Commission => write!(f, "COMMISSION"),
            IncomeType::InsuranceClear => write!(f, "INSURANCE_CLEAR"),
            IncomeType::ReferralKickback => write!(f, "REFERRAL_KICKBACK"),
            IncomeType::CommissionRebate => write!(f, "COMMISSION_REBATE"),
            IncomeType::ApiRebate => write!(f, "API_REBATE"),
            IncomeType::ContReward => write!(f, "CONT_REWARD"),
            IncomeType::UsdVsTokenSettlement => write!(f, "USD_VS_TOKEN_SETTLEMENT"),
            IncomeType::FeeReward => write!(f, "FEE_REWARD"),
            IncomeType::TokenReward => write!(f, "TOKEN_REWARD"),
            IncomeType::TransferIn => write!(f, "TRANSFER_IN"),
            IncomeType::TransferOut => write!(f, "TRANSFER_OUT"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarginType {
    #[serde(rename = "CROSS")]
    Cross,
    #[serde(rename = "ISOLATED")]
    Isolated,
}

impl fmt::Display for MarginType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MarginType::Cross => write!(f, "cross"),
            MarginType::Isolated => write!(f, "isolated"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WebSocketEventType {
    DepthUpdate,
}

impl fmt::Display for WebSocketEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WebSocketEventType::DepthUpdate => write!(f, "depthUpdate"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExchangeFilterType {
    ExchangeMaxNumOrders,
    ExchangeMaxNumAlgoOrders,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SymbolStatus {
    PreTrading,
    Trading,
    PostTrading,
    EndOfDay,
    Halt,
    AuctionMatch,
    Break,
}

/// Represents the type of a symbol (contract).
///
/// - DELIVERY_CONTRACT: Delivery contract
/// - PERPETUAL_CONTRACT: Perpetual contract
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SymbolType {
    DeliveryContract,
    PerpetualContract,
}

/// Represents the contract type for a symbol.
///
/// - PERPETUAL
/// - CURRENT_QUARTER
/// - NEXT_QUARTER
/// - CURRENT_QUARTER_DELIVERING (invalid, only for DELIVERING status)
/// - NEXT_QUARTER_DELIVERING (invalid, only for DELIVERING status)
/// - PERPETUAL_DELIVERING
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContractType {
    Perpetual,
    CurrentQuarter,
    NextQuarter,
    CurrentQuarterDelivering,
    NextQuarterDelivering,

    #[serde(rename = "PERPETUAL DELIVERING")]
    PerpetualDelivering,
}

/// Represents the contract status (`contractStatus`, `status`).
///
/// [docs]: https://developers.binance.com/docs/derivatives/coin-margined-futures/common-definition#enum-definitions
///
/// Variants:
/// - `PendingTrading`
/// - `Trading`
/// - `PreDelivering`
/// - `Delivering`
/// - `Delivered`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContractStatus {
    PendingTrading,
    Trading,
    PreDelivering,
    Delivering,
    Delivered,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UnderlyingType {
    Coin,
    Index,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SymbolFilterType {
    PriceFilter,
    LotSizeFilter,
    MinNotionalFilter,
    MaxNumOrdersFilter,
    MaxNumAlgoOrdersFilter,
    PercentPriceFilter,
    MaxPositionFilter,
    TrailingDataFilter,
}

/// Represents the price match mode for orders.
///
/// Can be set to OPPONENT, OPPONENT_5, OPPONENT_10, OPPONENT_20, QUEUE, QUEUE_5, QUEUE_10, QUEUE_20, or NONE.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PriceMatch {
    None,
    Opponent,
    Opponent5,
    Opponent10,
    Opponent20,
    Queue,
    Queue5,
    Queue10,
    Queue20,
}

impl fmt::Display for PriceMatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PriceMatch::None => write!(f, "NONE"),
            PriceMatch::Opponent => write!(f, "OPPONENT"),
            PriceMatch::Opponent5 => write!(f, "OPPONENT_5"),
            PriceMatch::Opponent10 => write!(f, "OPPONENT_10"),
            PriceMatch::Opponent20 => write!(f, "OPPONENT_20"),
            PriceMatch::Queue => write!(f, "QUEUE"),
            PriceMatch::Queue5 => write!(f, "QUEUE_5"),
            PriceMatch::Queue10 => write!(f, "QUEUE_10"),
            PriceMatch::Queue20 => write!(f, "QUEUE_20"),
        }
    }
}

/// Represents the kline/candlestick chart intervals.
///
/// m -> minutes; h -> hours; d -> days; w -> weeks; M -> months
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum KlineInterval {
    #[serde(rename = "1m")]
    I1m,
    #[serde(rename = "3m")]
    I3m,
    #[serde(rename = "5m")]
    I5m,
    #[serde(rename = "15m")]
    I15m,
    #[serde(rename = "30m")]
    I30m,
    #[serde(rename = "1h")]
    I1h,
    #[serde(rename = "2h")]
    I2h,
    #[serde(rename = "4h")]
    I4h,
    #[serde(rename = "6h")]
    I6h,
    #[serde(rename = "8h")]
    I8h,
    #[serde(rename = "12h")]
    I12h,
    #[serde(rename = "1d")]
    I1d,
    #[serde(rename = "3d")]
    I3d,
    #[serde(rename = "1w")]
    I1w,
    #[serde(rename = "1M")]
    I1M,
}

impl fmt::Display for KlineInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KlineInterval::I1m => write!(f, "1m"),
            KlineInterval::I3m => write!(f, "3m"),
            KlineInterval::I5m => write!(f, "5m"),
            KlineInterval::I15m => write!(f, "15m"),
            KlineInterval::I30m => write!(f, "30m"),
            KlineInterval::I1h => write!(f, "1h"),
            KlineInterval::I2h => write!(f, "2h"),
            KlineInterval::I4h => write!(f, "4h"),
            KlineInterval::I6h => write!(f, "6h"),
            KlineInterval::I8h => write!(f, "8h"),
            KlineInterval::I12h => write!(f, "12h"),
            KlineInterval::I1d => write!(f, "1d"),
            KlineInterval::I3d => write!(f, "3d"),
            KlineInterval::I1w => write!(f, "1w"),
            KlineInterval::I1M => write!(f, "1M"),
        }
    }
}

/// Represents the contract type for continuous klines and statistics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContractTypeFilter {
    All,
    Perpetual,
    CurrentQuarter,
    NextQuarter,
}

impl fmt::Display for ContractTypeFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContractTypeFilter::All => write!(f, "ALL"),
            ContractTypeFilter::Perpetual => write!(f, "PERPETUAL"),
            ContractTypeFilter::CurrentQuarter => write!(f, "CURRENT_QUARTER"),
            ContractTypeFilter::NextQuarter => write!(f, "NEXT_QUARTER"),
        }
    }
}

/// Represents the period for statistical endpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StatsPeriod {
    #[serde(rename = "5m")]
    I5m,
    #[serde(rename = "15m")]
    I15m,
    #[serde(rename = "30m")]
    I30m,
    #[serde(rename = "1h")]
    I1h,
    #[serde(rename = "2h")]
    I2h,
    #[serde(rename = "4h")]
    I4h,
    #[serde(rename = "6h")]
    I6h,
    #[serde(rename = "12h")]
    I12h,
    #[serde(rename = "1d")]
    I1d,
}

impl fmt::Display for StatsPeriod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatsPeriod::I5m => write!(f, "5m"),
            StatsPeriod::I15m => write!(f, "15m"),
            StatsPeriod::I30m => write!(f, "30m"),
            StatsPeriod::I1h => write!(f, "1h"),
            StatsPeriod::I2h => write!(f, "2h"),
            StatsPeriod::I4h => write!(f, "4h"),
            StatsPeriod::I6h => write!(f, "6h"),
            StatsPeriod::I12h => write!(f, "12h"),
            StatsPeriod::I1d => write!(f, "1d"),
        }
    }
}

/// Represents the period for data endpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Period {
    #[serde(rename = "5m")]
    I5m,

    #[serde(rename = "15m")]
    I15m,

    #[serde(rename = "30m")]
    I30m,

    #[serde(rename = "1h")]
    I1h,

    #[serde(rename = "2h")]
    I2h,

    #[serde(rename = "4h")]
    I4h,

    #[serde(rename = "6h")]
    I6h,

    #[serde(rename = "12h")]
    I12h,

    #[serde(rename = "1d")]
    I1d,
}

impl Period {
    pub fn as_str(&self) -> &'static str {
        match self {
            Period::I5m => "5m",
            Period::I15m => "15m",
            Period::I30m => "30m",
            Period::I1h => "1h",
            Period::I2h => "2h",
            Period::I4h => "4h",
            Period::I6h => "6h",
            Period::I12h => "12h",
            Period::I1d => "1d",
        }
    }
}

impl fmt::Display for Period {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Represents the auto close type for force orders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AutoCloseType {
    Liquidation,
    Adl,
}

impl fmt::Display for AutoCloseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AutoCloseType::Liquidation => write!(f, "LIQUIDATION"),
            AutoCloseType::Adl => write!(f, "ADL"),
        }
    }
}

/// Represents the transfer type for universal transfers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransferType {
    MainUmfuture,
    MainCmfuture,
    MainMargin,
    UmfutureMain,
    UmfutureMargin,
    CmfutureMain,
    CmfutureMargin,
    MarginMain,
    MarginUmfuture,
    MarginCmfuture,
    IsolatedmarginMargin,
    MarginIsolatedmargin,
    IsolatedmarginIsolatedmargin,
    MainFunding,
    FundingMain,
    FundingUmfuture,
    UmfutureFunding,
    MarginFunding,
    FundingMargin,
    FundingCmfuture,
    CmfutureFunding,
    MainOption,
    OptionMain,
    UmfutureOption,
    OptionUmfuture,
    MarginOption,
    OptionMargin,
    FundingOption,
    OptionFunding,
    MainPortfolioMargin,
    PortfolioMarginMain,
}

impl fmt::Display for TransferType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransferType::MainUmfuture => write!(f, "MAIN_UMFUTURE"),
            TransferType::MainCmfuture => write!(f, "MAIN_CMFUTURE"),
            TransferType::MainMargin => write!(f, "MAIN_MARGIN"),
            TransferType::UmfutureMain => write!(f, "UMFUTURE_MAIN"),
            TransferType::UmfutureMargin => write!(f, "UMFUTURE_MARGIN"),
            TransferType::CmfutureMain => write!(f, "CMFUTURE_MAIN"),
            TransferType::CmfutureMargin => write!(f, "CMFUTURE_MARGIN"),
            TransferType::MarginMain => write!(f, "MARGIN_MAIN"),
            TransferType::MarginUmfuture => write!(f, "MARGIN_UMFUTURE"),
            TransferType::MarginCmfuture => write!(f, "MARGIN_CMFUTURE"),
            TransferType::IsolatedmarginMargin => write!(f, "ISOLATEDMARGIN_MARGIN"),
            TransferType::MarginIsolatedmargin => write!(f, "MARGIN_ISOLATEDMARGIN"),
            TransferType::IsolatedmarginIsolatedmargin => {
                write!(f, "ISOLATEDMARGIN_ISOLATEDMARGIN")
            }
            TransferType::MainFunding => write!(f, "MAIN_FUNDING"),
            TransferType::FundingMain => write!(f, "FUNDING_MAIN"),
            TransferType::FundingUmfuture => write!(f, "FUNDING_UMFUTURE"),
            TransferType::UmfutureFunding => write!(f, "UMFUTURE_FUNDING"),
            TransferType::MarginFunding => write!(f, "MARGIN_FUNDING"),
            TransferType::FundingMargin => write!(f, "FUNDING_MARGIN"),
            TransferType::FundingCmfuture => write!(f, "FUNDING_CMFUTURE"),
            TransferType::CmfutureFunding => write!(f, "CMFUTURE_FUNDING"),
            TransferType::MainOption => write!(f, "MAIN_OPTION"),
            TransferType::OptionMain => write!(f, "OPTION_MAIN"),
            TransferType::UmfutureOption => write!(f, "UMFUTURE_OPTION"),
            TransferType::OptionUmfuture => write!(f, "OPTION_UMFUTURE"),
            TransferType::MarginOption => write!(f, "MARGIN_OPTION"),
            TransferType::OptionMargin => write!(f, "OPTION_MARGIN"),
            TransferType::FundingOption => write!(f, "FUNDING_OPTION"),
            TransferType::OptionFunding => write!(f, "OPTION_FUNDING"),
            TransferType::MainPortfolioMargin => write!(f, "MAIN_PORTFOLIO_MARGIN"),
            TransferType::PortfolioMarginMain => write!(f, "PORTFOLIO_MARGIN_MAIN"),
        }
    }
}

/// Represents the download status for transaction history.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DownloadStatus {
    Completed,
    Processing,
}

impl fmt::Display for DownloadStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DownloadStatus::Completed => write!(f, "completed"),
            DownloadStatus::Processing => write!(f, "processing"),
        }
    }
}

/// Represents the margin modification type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarginModificationType {
    /// Add position margin
    #[serde(rename = "1")]
    Add = 1,
    /// Reduce position margin
    #[serde(rename = "2")]
    Reduce = 2,
}

impl fmt::Display for MarginModificationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MarginModificationType::Add => write!(f, "1"),
            MarginModificationType::Reduce => write!(f, "2"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_side_serialization() {
        assert_eq!(serde_json::to_string(&OrderSide::Buy).unwrap(), "\"BUY\"");
        assert_eq!(serde_json::to_string(&OrderSide::Sell).unwrap(), "\"SELL\"");
    }

    #[test]
    fn test_order_side_deserialization() {
        assert_eq!(
            serde_json::from_str::<OrderSide>("\"BUY\"").unwrap(),
            OrderSide::Buy
        );
        assert_eq!(
            serde_json::from_str::<OrderSide>("\"SELL\"").unwrap(),
            OrderSide::Sell
        );
    }

    #[test]
    fn test_order_side_display() {
        assert_eq!(OrderSide::Buy.to_string(), "BUY");
        assert_eq!(OrderSide::Sell.to_string(), "SELL");
    }

    #[test]
    fn test_position_side_serialization() {
        assert_eq!(
            serde_json::to_string(&PositionSide::Both).unwrap(),
            "\"BOTH\""
        );
        assert_eq!(
            serde_json::to_string(&PositionSide::Long).unwrap(),
            "\"LONG\""
        );
        assert_eq!(
            serde_json::to_string(&PositionSide::Short).unwrap(),
            "\"SHORT\""
        );
    }

    #[test]
    fn test_position_side_deserialization() {
        assert_eq!(
            serde_json::from_str::<PositionSide>("\"BOTH\"").unwrap(),
            PositionSide::Both
        );
        assert_eq!(
            serde_json::from_str::<PositionSide>("\"LONG\"").unwrap(),
            PositionSide::Long
        );
        assert_eq!(
            serde_json::from_str::<PositionSide>("\"SHORT\"").unwrap(),
            PositionSide::Short
        );
    }

    #[test]
    fn test_order_type_serialization() {
        assert_eq!(
            serde_json::to_string(&OrderType::Limit).unwrap(),
            "\"LIMIT\""
        );
        assert_eq!(
            serde_json::to_string(&OrderType::Market).unwrap(),
            "\"MARKET\""
        );
        assert_eq!(serde_json::to_string(&OrderType::Stop).unwrap(), "\"STOP\"");
        assert_eq!(
            serde_json::to_string(&OrderType::StopMarket).unwrap(),
            "\"STOP_MARKET\""
        );
    }

    #[test]
    fn test_order_type_deserialization() {
        assert_eq!(
            serde_json::from_str::<OrderType>("\"LIMIT\"").unwrap(),
            OrderType::Limit
        );
        assert_eq!(
            serde_json::from_str::<OrderType>("\"MARKET\"").unwrap(),
            OrderType::Market
        );
        assert_eq!(
            serde_json::from_str::<OrderType>("\"STOP\"").unwrap(),
            OrderType::Stop
        );
        assert_eq!(
            serde_json::from_str::<OrderType>("\"STOP_MARKET\"").unwrap(),
            OrderType::StopMarket
        );
    }

    #[test]
    fn test_time_in_force_serialization() {
        assert_eq!(serde_json::to_string(&TimeInForce::GTC).unwrap(), "\"GTC\"");
        assert_eq!(serde_json::to_string(&TimeInForce::IOC).unwrap(), "\"IOC\"");
        assert_eq!(serde_json::to_string(&TimeInForce::FOK).unwrap(), "\"FOK\"");
        assert_eq!(serde_json::to_string(&TimeInForce::GTX).unwrap(), "\"GTX\"");
    }

    #[test]
    fn test_time_in_force_deserialization() {
        assert_eq!(
            serde_json::from_str::<TimeInForce>("\"GTC\"").unwrap(),
            TimeInForce::GTC
        );
        assert_eq!(
            serde_json::from_str::<TimeInForce>("\"IOC\"").unwrap(),
            TimeInForce::IOC
        );
        assert_eq!(
            serde_json::from_str::<TimeInForce>("\"FOK\"").unwrap(),
            TimeInForce::FOK
        );
        assert_eq!(
            serde_json::from_str::<TimeInForce>("\"GTX\"").unwrap(),
            TimeInForce::GTX
        );
    }

    #[test]
    fn test_order_status_serialization() {
        assert_eq!(serde_json::to_string(&OrderStatus::New).unwrap(), "\"NEW\"");
        assert_eq!(
            serde_json::to_string(&OrderStatus::Filled).unwrap(),
            "\"FILLED\""
        );
        assert_eq!(
            serde_json::to_string(&OrderStatus::Canceled).unwrap(),
            "\"CANCELED\""
        );
        assert_eq!(
            serde_json::to_string(&OrderStatus::Expired).unwrap(),
            "\"EXPIRED\""
        );
    }

    #[test]
    fn test_order_status_deserialization() {
        assert_eq!(
            serde_json::from_str::<OrderStatus>("\"NEW\"").unwrap(),
            OrderStatus::New
        );
        assert_eq!(
            serde_json::from_str::<OrderStatus>("\"FILLED\"").unwrap(),
            OrderStatus::Filled
        );
        assert_eq!(
            serde_json::from_str::<OrderStatus>("\"CANCELED\"").unwrap(),
            OrderStatus::Canceled
        );
        assert_eq!(
            serde_json::from_str::<OrderStatus>("\"EXPIRED\"").unwrap(),
            OrderStatus::Expired
        );
    }

    #[test]
    fn test_margin_type_serialization() {
        assert_eq!(
            serde_json::to_string(&MarginType::Cross).unwrap(),
            "\"CROSS\""
        );
        assert_eq!(
            serde_json::to_string(&MarginType::Isolated).unwrap(),
            "\"ISOLATED\""
        );
    }

    #[test]
    fn test_margin_type_deserialization() {
        assert_eq!(
            serde_json::from_str::<MarginType>("\"CROSS\"").unwrap(),
            MarginType::Cross
        );
        assert_eq!(
            serde_json::from_str::<MarginType>("\"ISOLATED\"").unwrap(),
            MarginType::Isolated
        );
    }

    #[test]
    fn test_margin_modification_type_serialization() {
        assert_eq!(
            serde_json::to_string(&MarginModificationType::Add).unwrap(),
            "\"1\""
        );
        assert_eq!(
            serde_json::to_string(&MarginModificationType::Reduce).unwrap(),
            "\"2\""
        );
    }

    #[test]
    fn test_margin_modification_type_deserialization() {
        assert_eq!(
            serde_json::from_str::<MarginModificationType>("\"1\"").unwrap(),
            MarginModificationType::Add
        );
        assert_eq!(
            serde_json::from_str::<MarginModificationType>("\"2\"").unwrap(),
            MarginModificationType::Reduce
        );
    }

    #[test]
    fn test_margin_modification_type_display() {
        assert_eq!(MarginModificationType::Add.to_string(), "1");
        assert_eq!(MarginModificationType::Reduce.to_string(), "2");
    }
}
