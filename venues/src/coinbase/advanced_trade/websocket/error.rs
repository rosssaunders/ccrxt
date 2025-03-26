use std::fmt;

#[derive(Debug)]
pub enum CoinbaseAdvancedTradeError {
    WebSocketError(String),
    ParseError(String),
    ConnectionError(String),
    SubscriptionError(String),
    RateLimitError(String),
    InvalidMessage(String),
}

impl fmt::Display for CoinbaseAdvancedTradeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoinbaseAdvancedTradeError::WebSocketError(msg) => write!(f, "WebSocket error: {}", msg),
            CoinbaseAdvancedTradeError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            CoinbaseAdvancedTradeError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            CoinbaseAdvancedTradeError::SubscriptionError(msg) => write!(f, "Subscription error: {}", msg),
            CoinbaseAdvancedTradeError::RateLimitError(msg) => write!(f, "Rate limit error: {}", msg),
            CoinbaseAdvancedTradeError::InvalidMessage(msg) => write!(f, "Invalid message: {}", msg),
        }
    }
}

impl std::error::Error for CoinbaseAdvancedTradeError {} 