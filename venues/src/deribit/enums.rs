use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

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
pub enum WithdrawalPriority {
    #[serde(rename = "insane")]
    Insane,
    #[serde(rename = "extreme_high")]
    ExtremeHigh,
    #[serde(rename = "very_high")]
    VeryHigh,
    #[serde(rename = "high")]
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

impl Default for WithdrawalPriority {
    fn default() -> Self {
        WithdrawalPriority::High
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
        assert_eq!(format!("{}", WithdrawalPriority::ExtremeHigh), "extreme_high");
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

        assert_eq!(serde_json::to_string(&unconfirmed).unwrap(), "\"unconfirmed\"");
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
        assert_eq!(serde_json::to_string(&withdrawal).unwrap(), "\"withdrawal\"");
        assert_eq!(serde_json::to_string(&deposit_source).unwrap(), "\"deposit_source\"");

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
        assert_eq!(format!("{}", AddressBookType::DepositSource), "deposit_source");
    }

    #[test]
    fn test_address_status_serialization() {
        let admin_locked = AddressStatus::AdminLocked;
        let waiting = AddressStatus::Waiting;
        let confirmed = AddressStatus::Confirmed;
        let ready = AddressStatus::Ready;

        assert_eq!(serde_json::to_string(&admin_locked).unwrap(), "\"admin_locked\"");
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

        assert_eq!(serde_json::to_string(&steth).unwrap(), "\"STETH\"");
        assert_eq!(serde_json::to_string(&matic).unwrap(), "\"MATIC\"");
        assert_eq!(serde_json::to_string(&sol).unwrap(), "\"SOL\"");

        let steth_from_json: Currency = serde_json::from_str("\"STETH\"").unwrap();
        let matic_from_json: Currency = serde_json::from_str("\"MATIC\"").unwrap();
        let sol_from_json: Currency = serde_json::from_str("\"SOL\"").unwrap();

        assert_eq!(steth_from_json, Currency::STETH);
        assert_eq!(matic_from_json, Currency::MATIC);
        assert_eq!(sol_from_json, Currency::SOL);
    }
}