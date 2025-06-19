use std::fmt::{Display, Formatter, Result as FmtResult};

use serde::{Deserialize, Serialize};

/// Currency types supported by Deribit
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Currency {
    #[serde(rename = "BTC")]
    BTC,
    #[serde(rename = "ETH")]
    ETH,
    #[serde(rename = "STETH")]
    STETH,
    #[serde(rename = "ETHW")]
    ETHW,
    #[serde(rename = "USDC")]
    USDC,
    #[serde(rename = "USDT")]
    USDT,
    #[serde(rename = "EURR")]
    EURR,
    #[serde(rename = "MATIC")]
    MATIC,
    #[serde(rename = "SOL")]
    SOL,
    #[serde(rename = "XRP")]
    XRP,
    #[serde(rename = "USYC")]
    USYC,
    #[serde(rename = "PAXG")]
    PAXG,
    #[serde(rename = "BNB")]
    BNB,
    #[serde(rename = "USDE")]
    USDE,
    #[serde(rename = "any")]
    Any,
}

impl Display for Currency {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Currency::BTC => write!(f, "BTC"),
            Currency::ETH => write!(f, "ETH"),
            Currency::STETH => write!(f, "STETH"),
            Currency::ETHW => write!(f, "ETHW"),
            Currency::USDC => write!(f, "USDC"),
            Currency::USDT => write!(f, "USDT"),
            Currency::EURR => write!(f, "EURR"),
            Currency::MATIC => write!(f, "MATIC"),
            Currency::SOL => write!(f, "SOL"),
            Currency::XRP => write!(f, "XRP"),
            Currency::USYC => write!(f, "USYC"),
            Currency::PAXG => write!(f, "PAXG"),
            Currency::BNB => write!(f, "BNB"),
            Currency::USDE => write!(f, "USDE"),
            Currency::Any => write!(f, "any"),
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

/// Withdrawal priority levels for Bitcoin withdrawals
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[derive(Default)]
pub enum WithdrawalPriority {
    #[serde(rename = "insane")]
    Insane,
    #[serde(rename = "extreme_high")]
    ExtremeHigh,
    #[serde(rename = "very_high")]
    VeryHigh,
    #[serde(rename = "high")]
    #[default]
    High,
    #[serde(rename = "mid")]
    Mid,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "very_low")]
    VeryLow,
}

impl Display for WithdrawalPriority {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            WithdrawalPriority::Insane => write!(f, "insane"),
            WithdrawalPriority::ExtremeHigh => write!(f, "extreme_high"),
            WithdrawalPriority::VeryHigh => write!(f, "very_high"),
            WithdrawalPriority::High => write!(f, "high"),
            WithdrawalPriority::Mid => write!(f, "mid"),
            WithdrawalPriority::Low => write!(f, "low"),
            WithdrawalPriority::VeryLow => write!(f, "very_low"),
        }
    }
}


/// Withdrawal state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WithdrawalState {
    #[serde(rename = "unconfirmed")]
    Unconfirmed,
    #[serde(rename = "confirmed")]
    Confirmed,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "interrupted")]
    Interrupted,
    #[serde(rename = "rejected")]
    Rejected,
}

impl Display for WithdrawalState {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            WithdrawalState::Unconfirmed => write!(f, "unconfirmed"),
            WithdrawalState::Confirmed => write!(f, "confirmed"),
            WithdrawalState::Cancelled => write!(f, "cancelled"),
            WithdrawalState::Completed => write!(f, "completed"),
            WithdrawalState::Interrupted => write!(f, "interrupted"),
            WithdrawalState::Rejected => write!(f, "rejected"),
        }
    }
}

/// Address book type for address book entries
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AddressBookType {
    #[serde(rename = "transfer")]
    Transfer,
    #[serde(rename = "withdrawal")]
    Withdrawal,
    #[serde(rename = "deposit_source")]
    DepositSource,
}

impl Display for AddressBookType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            AddressBookType::Transfer => write!(f, "transfer"),
            AddressBookType::Withdrawal => write!(f, "withdrawal"),
            AddressBookType::DepositSource => write!(f, "deposit_source"),
        }
    }
}

/// Order state for Deribit orders
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OrderState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "filled")]
    Filled,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "untriggered")]
    Untriggered,
}

impl Display for OrderState {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            OrderState::Open => write!(f, "open"),
            OrderState::Filled => write!(f, "filled"),
            OrderState::Rejected => write!(f, "rejected"),
            OrderState::Cancelled => write!(f, "cancelled"),
            OrderState::Untriggered => write!(f, "untriggered"),
        }
    }
}

/// Address status in the address book
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AddressStatus {
    #[serde(rename = "admin_locked")]
    AdminLocked,
    #[serde(rename = "waiting")]
    Waiting,
    #[serde(rename = "confirmed")]
    Confirmed,
    #[serde(rename = "ready")]
    Ready,
}

impl Display for AddressStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            AddressStatus::AdminLocked => write!(f, "admin_locked"),
            AddressStatus::Waiting => write!(f, "waiting"),
            AddressStatus::Confirmed => write!(f, "confirmed"),
            AddressStatus::Ready => write!(f, "ready"),
        }
    }
}

/// Order direction for Deribit orders
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OrderDirection {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}

impl Display for OrderDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            OrderDirection::Buy => write!(f, "buy"),
            OrderDirection::Sell => write!(f, "sell"),
        }
    }
}

/// Cancel reason for Deribit orders
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CancelReason {
    #[serde(rename = "user_request")]
    UserRequest,
    #[serde(rename = "autoliquidation")]
    Autoliquidation,
    #[serde(rename = "cancel_on_disconnect")]
    CancelOnDisconnect,
    #[serde(rename = "risk_mitigation")]
    RiskMitigation,
    #[serde(rename = "pme_risk_reduction")]
    PmeRiskReduction,
    #[serde(rename = "pme_account_locked")]
    PmeAccountLocked,
    #[serde(rename = "position_locked")]
    PositionLocked,
    #[serde(rename = "mmp_trigger")]
    MmpTrigger,
    #[serde(rename = "mmp_config_curtailment")]
    MmpConfigCurtailment,
    #[serde(rename = "edit_post_only_reject")]
    EditPostOnlyReject,
    #[serde(rename = "oco_other_closed")]
    OcoOtherClosed,
    #[serde(rename = "oto_primary_closed")]
    OtoPrimaryClosed,
    #[serde(rename = "settlement")]
    Settlement,
}

impl Display for CancelReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            CancelReason::UserRequest => write!(f, "user_request"),
            CancelReason::Autoliquidation => write!(f, "autoliquidation"),
            CancelReason::CancelOnDisconnect => write!(f, "cancel_on_disconnect"),
            CancelReason::RiskMitigation => write!(f, "risk_mitigation"),
            CancelReason::PmeRiskReduction => write!(f, "pme_risk_reduction"),
            CancelReason::PmeAccountLocked => write!(f, "pme_account_locked"),
            CancelReason::PositionLocked => write!(f, "position_locked"),
            CancelReason::MmpTrigger => write!(f, "mmp_trigger"),
            CancelReason::MmpConfigCurtailment => write!(f, "mmp_config_curtailment"),
            CancelReason::EditPostOnlyReject => write!(f, "edit_post_only_reject"),
            CancelReason::OcoOtherClosed => write!(f, "oco_other_closed"),
            CancelReason::OtoPrimaryClosed => write!(f, "oto_primary_closed"),
            CancelReason::Settlement => write!(f, "settlement"),
        }
    }
}

/// Advanced order type for options
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AdvancedType {
    #[serde(rename = "usd")]
    Usd,
    #[serde(rename = "implv")]
    Implv,
}

impl Display for AdvancedType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            AdvancedType::Usd => write!(f, "usd"),
            AdvancedType::Implv => write!(f, "implv"),
        }
    }
}

/// Trigger type for trigger orders
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TriggerType {
    #[serde(rename = "index_price")]
    IndexPrice,
    #[serde(rename = "mark_price")]
    MarkPrice,
    #[serde(rename = "last_price")]
    LastPrice,
}

impl Display for TriggerType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            TriggerType::IndexPrice => write!(f, "index_price"),
            TriggerType::MarkPrice => write!(f, "mark_price"),
            TriggerType::LastPrice => write!(f, "last_price"),
        }
    }
}

/// Instrument kind types for filtering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstrumentKind {
    #[serde(rename = "future")]
    Future,
    #[serde(rename = "option")]
    Option,
    #[serde(rename = "spot")]
    Spot,
    #[serde(rename = "future_combo")]
    FutureCombo,
    #[serde(rename = "option_combo")]
    OptionCombo,
    #[serde(rename = "combo")]
    Combo,
    #[serde(rename = "any")]
    Any,
}

impl Display for InstrumentKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            InstrumentKind::Future => write!(f, "future"),
            InstrumentKind::Option => write!(f, "option"),
            InstrumentKind::Spot => write!(f, "spot"),
            InstrumentKind::FutureCombo => write!(f, "future_combo"),
            InstrumentKind::OptionCombo => write!(f, "option_combo"),
            InstrumentKind::Combo => write!(f, "combo"),
            InstrumentKind::Any => write!(f, "any"),
        }
    }
}

/// Order type for filtering cancel operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "limit")]
    Limit,
    #[serde(rename = "trigger_all")]
    TriggerAll,
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "take")]
    Take,
    #[serde(rename = "trailing_stop")]
    TrailingStop,
}

impl Display for OrderType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            OrderType::All => write!(f, "all"),
            OrderType::Limit => write!(f, "limit"),
            OrderType::TriggerAll => write!(f, "trigger_all"),
            OrderType::Stop => write!(f, "stop"),
            OrderType::Take => write!(f, "take"),
            OrderType::TrailingStop => write!(f, "trailing_stop"),
        }
    }
}

/// Sorting direction for trade queries
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Sorting {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
    #[serde(rename = "default")]
    Default,
}

impl Display for Sorting {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Sorting::Asc => write!(f, "asc"),
            Sorting::Desc => write!(f, "desc"),
            Sorting::Default => write!(f, "default"),
        }
    }
}

/// Tick direction for trades  
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TickDirection {
    #[serde(rename = "0")]
    PlusTick,
    #[serde(rename = "1")]
    ZeroPlusTick,
    #[serde(rename = "2")]
    MinusTick,
    #[serde(rename = "3")]
    ZeroMinusTick,
}

impl Display for TickDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            TickDirection::PlusTick => write!(f, "0"),
            TickDirection::ZeroPlusTick => write!(f, "1"),
            TickDirection::MinusTick => write!(f, "2"),
            TickDirection::ZeroMinusTick => write!(f, "3"),
        }
    }
}

/// Liquidity role in trade (maker or taker)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Liquidity {
    #[serde(rename = "M")]
    Maker,
    #[serde(rename = "T")]
    Taker,
}

impl Display for Liquidity {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Liquidity::Maker => write!(f, "M"),
            Liquidity::Taker => write!(f, "T"),
        }
    }
}

/// Trade order type (different from general OrderType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradeOrderType {
    #[serde(rename = "limit")]
    Limit,
    #[serde(rename = "market")]
    Market,
    #[serde(rename = "liquidation")]
    Liquidation,
}

impl Display for TradeOrderType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            TradeOrderType::Limit => write!(f, "limit"),
            TradeOrderType::Market => write!(f, "market"),
            TradeOrderType::Liquidation => write!(f, "liquidation"),
        }
    }
}

/// Liquidation side for trades
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LiquidationSide {
    #[serde(rename = "M")]
    Maker,
    #[serde(rename = "T")]
    Taker,
    #[serde(rename = "MT")]
    Both,
}

impl Display for LiquidationSide {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            LiquidationSide::Maker => write!(f, "M"),
            LiquidationSide::Taker => write!(f, "T"),
            LiquidationSide::Both => write!(f, "MT"),
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
        assert_eq!(format!("{}", Currency::STETH), "STETH");
        assert_eq!(format!("{}", Currency::ETHW), "ETHW");
        assert_eq!(format!("{}", Currency::USDC), "USDC");
        assert_eq!(format!("{}", Currency::USDT), "USDT");
        assert_eq!(format!("{}", Currency::EURR), "EURR");
        assert_eq!(format!("{}", Currency::MATIC), "MATIC");
        assert_eq!(format!("{}", Currency::SOL), "SOL");
        assert_eq!(format!("{}", Currency::XRP), "XRP");
        assert_eq!(format!("{}", Currency::USYC), "USYC");
        assert_eq!(format!("{}", Currency::PAXG), "PAXG");
        assert_eq!(format!("{}", Currency::BNB), "BNB");
        assert_eq!(format!("{}", Currency::USDE), "USDE");
        assert_eq!(format!("{}", Currency::Any), "any");
    }

    #[test]
    fn test_combo_state_display() {
        assert_eq!(format!("{}", ComboState::RFQ), "rfq");
        assert_eq!(format!("{}", ComboState::Active), "active");
        assert_eq!(format!("{}", ComboState::Inactive), "inactive");
    }

    #[test]
    fn test_withdrawal_priority_serialization() {
        let high = WithdrawalPriority::High;
        let insane = WithdrawalPriority::Insane;

        assert_eq!(serde_json::to_string(&high).unwrap(), "\"high\"");
        assert_eq!(serde_json::to_string(&insane).unwrap(), "\"insane\"");

        let high_from_json: WithdrawalPriority = serde_json::from_str("\"high\"").unwrap();
        let insane_from_json: WithdrawalPriority = serde_json::from_str("\"insane\"").unwrap();

        assert_eq!(high_from_json, WithdrawalPriority::High);
        assert_eq!(insane_from_json, WithdrawalPriority::Insane);
    }

    #[test]
    fn test_withdrawal_priority_display() {
        assert_eq!(format!("{}", WithdrawalPriority::Insane), "insane");
        assert_eq!(
            format!("{}", WithdrawalPriority::ExtremeHigh),
            "extreme_high"
        );
        assert_eq!(format!("{}", WithdrawalPriority::VeryHigh), "very_high");
        assert_eq!(format!("{}", WithdrawalPriority::High), "high");
        assert_eq!(format!("{}", WithdrawalPriority::Mid), "mid");
        assert_eq!(format!("{}", WithdrawalPriority::Low), "low");
        assert_eq!(format!("{}", WithdrawalPriority::VeryLow), "very_low");
    }

    #[test]
    fn test_withdrawal_priority_default() {
        assert_eq!(WithdrawalPriority::default(), WithdrawalPriority::High);
    }

    #[test]
    fn test_withdrawal_state_serialization() {
        let unconfirmed = WithdrawalState::Unconfirmed;
        let completed = WithdrawalState::Completed;

        assert_eq!(
            serde_json::to_string(&unconfirmed).unwrap(),
            "\"unconfirmed\""
        );
        assert_eq!(serde_json::to_string(&completed).unwrap(), "\"completed\"");

        let unconfirmed_from_json: WithdrawalState = serde_json::from_str("\"unconfirmed\"").unwrap();
        let completed_from_json: WithdrawalState = serde_json::from_str("\"completed\"").unwrap();

        assert_eq!(unconfirmed_from_json, WithdrawalState::Unconfirmed);
        assert_eq!(completed_from_json, WithdrawalState::Completed);
    }

    #[test]
    fn test_withdrawal_state_display() {
        assert_eq!(format!("{}", WithdrawalState::Unconfirmed), "unconfirmed");
        assert_eq!(format!("{}", WithdrawalState::Confirmed), "confirmed");
        assert_eq!(format!("{}", WithdrawalState::Cancelled), "cancelled");
        assert_eq!(format!("{}", WithdrawalState::Completed), "completed");
        assert_eq!(format!("{}", WithdrawalState::Interrupted), "interrupted");
        assert_eq!(format!("{}", WithdrawalState::Rejected), "rejected");
    }

    #[test]
    fn test_address_book_type_serialization() {
        let transfer = AddressBookType::Transfer;
        let withdrawal = AddressBookType::Withdrawal;
        let deposit_source = AddressBookType::DepositSource;

        assert_eq!(serde_json::to_string(&transfer).unwrap(), "\"transfer\"");
        assert_eq!(
            serde_json::to_string(&withdrawal).unwrap(),
            "\"withdrawal\""
        );
        assert_eq!(
            serde_json::to_string(&deposit_source).unwrap(),
            "\"deposit_source\""
        );

        let transfer_from_json: AddressBookType = serde_json::from_str("\"transfer\"").unwrap();
        let withdrawal_from_json: AddressBookType = serde_json::from_str("\"withdrawal\"").unwrap();
        let deposit_source_from_json: AddressBookType = serde_json::from_str("\"deposit_source\"").unwrap();

        assert_eq!(transfer_from_json, AddressBookType::Transfer);
        assert_eq!(withdrawal_from_json, AddressBookType::Withdrawal);
        assert_eq!(deposit_source_from_json, AddressBookType::DepositSource);
    }

    #[test]
    fn test_address_book_type_display() {
        assert_eq!(format!("{}", AddressBookType::Transfer), "transfer");
        assert_eq!(format!("{}", AddressBookType::Withdrawal), "withdrawal");
        assert_eq!(
            format!("{}", AddressBookType::DepositSource),
            "deposit_source"
        );
    }

    #[test]
    fn test_address_status_serialization() {
        let admin_locked = AddressStatus::AdminLocked;
        let waiting = AddressStatus::Waiting;
        let confirmed = AddressStatus::Confirmed;
        let ready = AddressStatus::Ready;

        assert_eq!(
            serde_json::to_string(&admin_locked).unwrap(),
            "\"admin_locked\""
        );
        assert_eq!(serde_json::to_string(&waiting).unwrap(), "\"waiting\"");
        assert_eq!(serde_json::to_string(&confirmed).unwrap(), "\"confirmed\"");
        assert_eq!(serde_json::to_string(&ready).unwrap(), "\"ready\"");

        let admin_locked_from_json: AddressStatus = serde_json::from_str("\"admin_locked\"").unwrap();
        let waiting_from_json: AddressStatus = serde_json::from_str("\"waiting\"").unwrap();
        let confirmed_from_json: AddressStatus = serde_json::from_str("\"confirmed\"").unwrap();
        let ready_from_json: AddressStatus = serde_json::from_str("\"ready\"").unwrap();

        assert_eq!(admin_locked_from_json, AddressStatus::AdminLocked);
        assert_eq!(waiting_from_json, AddressStatus::Waiting);
        assert_eq!(confirmed_from_json, AddressStatus::Confirmed);
        assert_eq!(ready_from_json, AddressStatus::Ready);
    }

    #[test]
    fn test_address_status_display() {
        assert_eq!(format!("{}", AddressStatus::AdminLocked), "admin_locked");
        assert_eq!(format!("{}", AddressStatus::Waiting), "waiting");
        assert_eq!(format!("{}", AddressStatus::Confirmed), "confirmed");
        assert_eq!(format!("{}", AddressStatus::Ready), "ready");
    }

    #[test]
    fn test_new_currency_serialization() {
        let steth = Currency::STETH;
        let matic = Currency::MATIC;
        let sol = Currency::SOL;
        let any = Currency::Any;

        assert_eq!(serde_json::to_string(&steth).unwrap(), "\"STETH\"");
        assert_eq!(serde_json::to_string(&matic).unwrap(), "\"MATIC\"");
        assert_eq!(serde_json::to_string(&sol).unwrap(), "\"SOL\"");
        assert_eq!(serde_json::to_string(&any).unwrap(), "\"any\"");

        let steth_from_json: Currency = serde_json::from_str("\"STETH\"").unwrap();
        let matic_from_json: Currency = serde_json::from_str("\"MATIC\"").unwrap();
        let sol_from_json: Currency = serde_json::from_str("\"SOL\"").unwrap();
        let any_from_json: Currency = serde_json::from_str("\"any\"").unwrap();

        assert_eq!(steth_from_json, Currency::STETH);
        assert_eq!(matic_from_json, Currency::MATIC);
        assert_eq!(sol_from_json, Currency::SOL);
        assert_eq!(any_from_json, Currency::Any);
    }

    #[test]
    fn test_order_state_serialization() {
        let open = OrderState::Open;
        let cancelled = OrderState::Cancelled;

        assert_eq!(serde_json::to_string(&open).unwrap(), "\"open\"");
        assert_eq!(serde_json::to_string(&cancelled).unwrap(), "\"cancelled\"");

        let open_from_json: OrderState = serde_json::from_str("\"open\"").unwrap();
        let cancelled_from_json: OrderState = serde_json::from_str("\"cancelled\"").unwrap();

        assert_eq!(open_from_json, OrderState::Open);
        assert_eq!(cancelled_from_json, OrderState::Cancelled);
    }

    #[test]
    fn test_order_direction_serialization() {
        let buy = OrderDirection::Buy;
        let sell = OrderDirection::Sell;

        assert_eq!(serde_json::to_string(&buy).unwrap(), "\"buy\"");
        assert_eq!(serde_json::to_string(&sell).unwrap(), "\"sell\"");

        let buy_from_json: OrderDirection = serde_json::from_str("\"buy\"").unwrap();
        let sell_from_json: OrderDirection = serde_json::from_str("\"sell\"").unwrap();

        assert_eq!(buy_from_json, OrderDirection::Buy);
        assert_eq!(sell_from_json, OrderDirection::Sell);
    }

    #[test]
    fn test_cancel_reason_serialization() {
        let user_request = CancelReason::UserRequest;
        let autoliquidation = CancelReason::Autoliquidation;

        assert_eq!(
            serde_json::to_string(&user_request).unwrap(),
            "\"user_request\""
        );
        assert_eq!(
            serde_json::to_string(&autoliquidation).unwrap(),
            "\"autoliquidation\""
        );

        let user_request_from_json: CancelReason = serde_json::from_str("\"user_request\"").unwrap();
        let autoliquidation_from_json: CancelReason = serde_json::from_str("\"autoliquidation\"").unwrap();

        assert_eq!(user_request_from_json, CancelReason::UserRequest);
        assert_eq!(autoliquidation_from_json, CancelReason::Autoliquidation);
    }

    #[test]
    fn test_advanced_type_serialization() {
        let usd = AdvancedType::Usd;
        let implv = AdvancedType::Implv;

        assert_eq!(serde_json::to_string(&usd).unwrap(), "\"usd\"");
        assert_eq!(serde_json::to_string(&implv).unwrap(), "\"implv\"");

        let usd_from_json: AdvancedType = serde_json::from_str("\"usd\"").unwrap();
        let implv_from_json: AdvancedType = serde_json::from_str("\"implv\"").unwrap();

        assert_eq!(usd_from_json, AdvancedType::Usd);
        assert_eq!(implv_from_json, AdvancedType::Implv);
    }

    #[test]
    fn test_trigger_type_serialization() {
        let index_price = TriggerType::IndexPrice;
        let mark_price = TriggerType::MarkPrice;

        assert_eq!(
            serde_json::to_string(&index_price).unwrap(),
            "\"index_price\""
        );
        assert_eq!(
            serde_json::to_string(&mark_price).unwrap(),
            "\"mark_price\""
        );

        let index_price_from_json: TriggerType = serde_json::from_str("\"index_price\"").unwrap();
        let mark_price_from_json: TriggerType = serde_json::from_str("\"mark_price\"").unwrap();

        assert_eq!(index_price_from_json, TriggerType::IndexPrice);
        assert_eq!(mark_price_from_json, TriggerType::MarkPrice);
    }

    #[test]
    fn test_instrument_kind_serialization() {
        let future = InstrumentKind::Future;
        let option = InstrumentKind::Option;
        let spot = InstrumentKind::Spot;
        let combo = InstrumentKind::Combo;

        assert_eq!(serde_json::to_string(&future).unwrap(), "\"future\"");
        assert_eq!(serde_json::to_string(&option).unwrap(), "\"option\"");
        assert_eq!(serde_json::to_string(&spot).unwrap(), "\"spot\"");
        assert_eq!(serde_json::to_string(&combo).unwrap(), "\"combo\"");

        let future_from_json: InstrumentKind = serde_json::from_str("\"future\"").unwrap();
        let option_from_json: InstrumentKind = serde_json::from_str("\"option\"").unwrap();
        let spot_from_json: InstrumentKind = serde_json::from_str("\"spot\"").unwrap();
        let combo_from_json: InstrumentKind = serde_json::from_str("\"combo\"").unwrap();

        assert_eq!(future_from_json, InstrumentKind::Future);
        assert_eq!(option_from_json, InstrumentKind::Option);
        assert_eq!(spot_from_json, InstrumentKind::Spot);
        assert_eq!(combo_from_json, InstrumentKind::Combo);
    }

    #[test]
    fn test_instrument_kind_display() {
        assert_eq!(format!("{}", InstrumentKind::Future), "future");
        assert_eq!(format!("{}", InstrumentKind::Option), "option");
        assert_eq!(format!("{}", InstrumentKind::Spot), "spot");
        assert_eq!(format!("{}", InstrumentKind::FutureCombo), "future_combo");
        assert_eq!(format!("{}", InstrumentKind::OptionCombo), "option_combo");
        assert_eq!(format!("{}", InstrumentKind::Combo), "combo");
        assert_eq!(format!("{}", InstrumentKind::Any), "any");
    }

    #[test]
    fn test_order_type_serialization() {
        let all = OrderType::All;
        let limit = OrderType::Limit;
        let trigger_all = OrderType::TriggerAll;
        let stop = OrderType::Stop;

        assert_eq!(serde_json::to_string(&all).unwrap(), "\"all\"");
        assert_eq!(serde_json::to_string(&limit).unwrap(), "\"limit\"");
        assert_eq!(serde_json::to_string(&trigger_all).unwrap(), "\"trigger_all\"");
        assert_eq!(serde_json::to_string(&stop).unwrap(), "\"stop\"");

        let all_from_json: OrderType = serde_json::from_str("\"all\"").unwrap();
        let limit_from_json: OrderType = serde_json::from_str("\"limit\"").unwrap();
        let trigger_all_from_json: OrderType = serde_json::from_str("\"trigger_all\"").unwrap();
        let stop_from_json: OrderType = serde_json::from_str("\"stop\"").unwrap();

        assert_eq!(all_from_json, OrderType::All);
        assert_eq!(limit_from_json, OrderType::Limit);
        assert_eq!(trigger_all_from_json, OrderType::TriggerAll);
        assert_eq!(stop_from_json, OrderType::Stop);
    }

    #[test]
    fn test_order_type_display() {
        assert_eq!(format!("{}", OrderType::All), "all");
        assert_eq!(format!("{}", OrderType::Limit), "limit");
        assert_eq!(format!("{}", OrderType::TriggerAll), "trigger_all");
        assert_eq!(format!("{}", OrderType::Stop), "stop");
        assert_eq!(format!("{}", OrderType::Take), "take");
        assert_eq!(format!("{}", OrderType::TrailingStop), "trailing_stop");
    }

    #[test]
    fn test_sorting_serialization() {
        let asc = Sorting::Asc;
        let desc = Sorting::Desc;
        let default = Sorting::Default;

        assert_eq!(serde_json::to_string(&asc).unwrap(), "\"asc\"");
        assert_eq!(serde_json::to_string(&desc).unwrap(), "\"desc\"");
        assert_eq!(serde_json::to_string(&default).unwrap(), "\"default\"");

        let asc_from_json: Sorting = serde_json::from_str("\"asc\"").unwrap();
        let desc_from_json: Sorting = serde_json::from_str("\"desc\"").unwrap();
        let default_from_json: Sorting = serde_json::from_str("\"default\"").unwrap();

        assert_eq!(asc_from_json, Sorting::Asc);
        assert_eq!(desc_from_json, Sorting::Desc);
        assert_eq!(default_from_json, Sorting::Default);
    }

    #[test]
    fn test_tick_direction_serialization() {
        let plus_tick = TickDirection::PlusTick;
        let zero_plus_tick = TickDirection::ZeroPlusTick;
        let minus_tick = TickDirection::MinusTick;
        let zero_minus_tick = TickDirection::ZeroMinusTick;

        assert_eq!(serde_json::to_string(&plus_tick).unwrap(), "\"0\"");
        assert_eq!(serde_json::to_string(&zero_plus_tick).unwrap(), "\"1\"");
        assert_eq!(serde_json::to_string(&minus_tick).unwrap(), "\"2\"");
        assert_eq!(serde_json::to_string(&zero_minus_tick).unwrap(), "\"3\"");

        let plus_tick_from_json: TickDirection = serde_json::from_str("\"0\"").unwrap();
        let zero_plus_tick_from_json: TickDirection = serde_json::from_str("\"1\"").unwrap();
        let minus_tick_from_json: TickDirection = serde_json::from_str("\"2\"").unwrap();
        let zero_minus_tick_from_json: TickDirection = serde_json::from_str("\"3\"").unwrap();

        assert_eq!(plus_tick_from_json, TickDirection::PlusTick);
        assert_eq!(zero_plus_tick_from_json, TickDirection::ZeroPlusTick);
        assert_eq!(minus_tick_from_json, TickDirection::MinusTick);
        assert_eq!(zero_minus_tick_from_json, TickDirection::ZeroMinusTick);
    }

    #[test]
    fn test_liquidity_serialization() {
        let maker = Liquidity::Maker;
        let taker = Liquidity::Taker;

        assert_eq!(serde_json::to_string(&maker).unwrap(), "\"M\"");
        assert_eq!(serde_json::to_string(&taker).unwrap(), "\"T\"");

        let maker_from_json: Liquidity = serde_json::from_str("\"M\"").unwrap();
        let taker_from_json: Liquidity = serde_json::from_str("\"T\"").unwrap();

        assert_eq!(maker_from_json, Liquidity::Maker);
        assert_eq!(taker_from_json, Liquidity::Taker);
    }

    #[test]
    fn test_trade_order_type_serialization() {
        let limit = TradeOrderType::Limit;
        let market = TradeOrderType::Market;
        let liquidation = TradeOrderType::Liquidation;

        assert_eq!(serde_json::to_string(&limit).unwrap(), "\"limit\"");
        assert_eq!(serde_json::to_string(&market).unwrap(), "\"market\"");
        assert_eq!(serde_json::to_string(&liquidation).unwrap(), "\"liquidation\"");

        let limit_from_json: TradeOrderType = serde_json::from_str("\"limit\"").unwrap();
        let market_from_json: TradeOrderType = serde_json::from_str("\"market\"").unwrap();
        let liquidation_from_json: TradeOrderType = serde_json::from_str("\"liquidation\"").unwrap();

        assert_eq!(limit_from_json, TradeOrderType::Limit);
        assert_eq!(market_from_json, TradeOrderType::Market);
        assert_eq!(liquidation_from_json, TradeOrderType::Liquidation);
    }

    #[test]
    fn test_liquidation_side_serialization() {
        let maker = LiquidationSide::Maker;
        let taker = LiquidationSide::Taker;
        let both = LiquidationSide::Both;

        assert_eq!(serde_json::to_string(&maker).unwrap(), "\"M\"");
        assert_eq!(serde_json::to_string(&taker).unwrap(), "\"T\"");
        assert_eq!(serde_json::to_string(&both).unwrap(), "\"MT\"");

        let maker_from_json: LiquidationSide = serde_json::from_str("\"M\"").unwrap();
        let taker_from_json: LiquidationSide = serde_json::from_str("\"T\"").unwrap();
        let both_from_json: LiquidationSide = serde_json::from_str("\"MT\"").unwrap();

        assert_eq!(maker_from_json, LiquidationSide::Maker);
        assert_eq!(taker_from_json, LiquidationSide::Taker);
        assert_eq!(both_from_json, LiquidationSide::Both);
    }

    #[test]
    fn test_new_enums_display() {
        assert_eq!(format!("{}", Sorting::Asc), "asc");
        assert_eq!(format!("{}", Sorting::Desc), "desc");
        assert_eq!(format!("{}", Sorting::Default), "default");

        assert_eq!(format!("{}", TickDirection::PlusTick), "0");
        assert_eq!(format!("{}", TickDirection::ZeroPlusTick), "1");
        assert_eq!(format!("{}", TickDirection::MinusTick), "2");
        assert_eq!(format!("{}", TickDirection::ZeroMinusTick), "3");

        assert_eq!(format!("{}", Liquidity::Maker), "M");
        assert_eq!(format!("{}", Liquidity::Taker), "T");

        assert_eq!(format!("{}", TradeOrderType::Limit), "limit");
        assert_eq!(format!("{}", TradeOrderType::Market), "market");
        assert_eq!(format!("{}", TradeOrderType::Liquidation), "liquidation");

        assert_eq!(format!("{}", LiquidationSide::Maker), "M");
        assert_eq!(format!("{}", LiquidationSide::Taker), "T");
        assert_eq!(format!("{}", LiquidationSide::Both), "MT");
    }
}
