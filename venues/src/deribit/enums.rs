#[derive(Debug)]
pub enum AccessScope {
    MainAccount,
    Connection,
    Session(String),
    AccountRead,
    AccountReadWrite,
    TradeRead,
    TradeReadWrite,
    WalletRead,
    WalletReadWrite,
}

impl std::fmt::Display for AccessScope {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AccessScope::MainAccount => write!(f, "mainaccount"),
            AccessScope::Connection => write!(f, "connection"),
            AccessScope::Session(name) => write!(f, "session:{}", name),
            AccessScope::AccountRead => write!(f, "account:read"),
            AccessScope::AccountReadWrite => write!(f, "account:read_write"),
            AccessScope::TradeRead => write!(f, "trade:read"),
            AccessScope::TradeReadWrite => write!(f, "trade:read_write"),
            AccessScope::WalletRead => write!(f, "wallet:read"),
            AccessScope::WalletReadWrite => write!(f, "wallet:read_write"),
        }
    }
}

#[derive(Debug)]
pub enum Expiration {
    Seconds(u64),
}

impl std::fmt::Display for Expiration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expiration::Seconds(seconds) => write!(f, "expires:{}s", seconds),
        }
    }
}

#[derive(Debug)]
pub enum IpAddress {
    Single(String),
    Any,
}

impl std::fmt::Display for IpAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            IpAddress::Single(ip) => write!(f, "ip:{}", ip),
            IpAddress::Any => write!(f, "*"),
        }
    }
}

#[derive(Debug)]
pub enum BlockTradeScope {
    Read,
    Write,
}

impl std::fmt::Display for BlockTradeScope {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BlockTradeScope::Read => write!(f, "block_trade:read"),
            BlockTradeScope::Write => write!(f, "blocktrade:readwrite"),
        }
    }
}

#[derive(Debug)]
pub enum BlockRfqScope {
    Read,
    Write,
}

impl std::fmt::Display for BlockRfqScope {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BlockRfqScope::Read => write!(f, "block_rfq:read"),
            BlockRfqScope::Write => write!(f, "blockrfq:readwrite"),
        }
    }
}