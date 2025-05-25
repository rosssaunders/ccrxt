use std::time::Duration;
use std::fmt;
use super::api_errors::BinanceCoinMAPIError;
use super::enums::{OrderSide, PositionSide, OrderType, TimeInForce, WorkingType, OrderStatus, PriceMatch, SelfTradePreventionMode};
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct BinanceHeaders {
    pub used_weight_1m: Option<u32>,
    pub order_count_1m: Option<u32>,
    pub order_count_1d: Option<u32>,
    pub order_count_1s: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct BinanceCoinMResponse<T> {
    pub data: T,
    pub rate_limit_duration: Duration,
    pub request_duration: Duration,
    pub headers: BinanceHeaders,
}

/// Represents all possible errors that can occur when interacting with the Binance API
#[derive(Debug)]
pub enum BinanceCoinMError {
    /// An error returned by the Binance API
    ApiError(BinanceCoinMAPIError),
    /// An HTTP-level error (network, timeout, etc.)
    HttpError(reqwest::Error),
    /// A general error with a descriptive message
    Error(String),
}

impl fmt::Display for BinanceCoinMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinanceCoinMError::ApiError(err) => write!(f, "API error: {}", err),
            BinanceCoinMError::HttpError(err) => write!(f, "HTTP error: {}", err),
            BinanceCoinMError::Error(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for BinanceCoinMError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BinanceCoinMError::HttpError(err) => Some(err),
            _ => None,
        }
    }
}

/// Type alias for results returned by Binance API operations
pub type BinanceCoinMResult<T> = Result<BinanceCoinMResponse<T>, BinanceCoinMError>; 

/// Represents an error response from the Binance API.
#[derive(Debug, Deserialize)]
pub(crate) struct ErrorResponse {
    pub code: i32,
    pub msg: String,
}

/// Parameters for placing a new order on Binance COINM Futures
#[derive(Debug, Clone, Serialize)]
pub struct OrderRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP")
    pub symbol: String,
    
    /// Order side (BUY or SELL)
    pub side: OrderSide,
    
    /// Position side (BOTH, LONG or SHORT) - Default BOTH for One-way Mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positionSide: Option<PositionSide>,
    
    /// Order type (LIMIT, MARKET, STOP, etc.)
    #[serde(rename = "type")]
    pub order_type: OrderType,
    
    /// Time in force (GTC, IOC, FOK, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeInForce: Option<TimeInForce>,
    
    /// Order quantity measured by contract number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Decimal>,
    
    /// Reduce only flag (true or false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduceOnly: Option<String>,
    
    /// Order price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,
    
    /// Custom client order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newClientOrderId: Option<String>,
    
    /// Stop price for STOP/STOP_MARKET or TAKE_PROFIT/TAKE_PROFIT_MARKET orders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopPrice: Option<Decimal>,
    
    /// Close position flag (true or false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closePosition: Option<String>,
    
    /// Activation price for TRAILING_STOP_MARKET orders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activationPrice: Option<Decimal>,
    
    /// Callback rate for TRAILING_STOP_MARKET orders (0.1 to 10)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callbackRate: Option<Decimal>,
    
    /// Stop price trigger type (MARK_PRICE or CONTRACT_PRICE)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workingType: Option<WorkingType>,
    
    /// Price protection flag (TRUE or FALSE)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priceProtect: Option<String>,
    
    /// Type of response (ACK or RESULT)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newOrderRespType: Option<String>,
    
    /// Price match mode for orders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priceMatch: Option<PriceMatch>,
    
    /// Self-trade prevention mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selfTradePreventionMode: Option<SelfTradePreventionMode>,
    
    /// Recv window in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recvWindow: Option<u64>,
    
    /// Timestamp in milliseconds
    #[serde(skip)]
    pub timestamp: Option<u64>,
}

/// Response for a new order placement on Binance COINM Futures
#[derive(Debug, Clone, Deserialize)]
pub struct OrderResponse {
    /// Client-specified order ID
    pub clientOrderId: String,
    
    /// Cumulative quantity filled
    pub cumQty: String,
    
    /// Cumulative quote asset quantity
    pub cumBase: String,
    
    /// Executed quantity
    pub executedQty: String,
    
    /// Exchange-assigned order ID
    pub orderId: u64,
    
    /// Average fill price
    pub avgPrice: String,
    
    /// Original order quantity
    pub origQty: String,
    
    /// Order price
    pub price: String,
    
    /// Whether the order is reduce-only
    pub reduceOnly: bool,
    
    /// Order side (BUY or SELL)
    pub side: OrderSide,
    
    /// Position side (LONG, SHORT, or BOTH)
    pub positionSide: PositionSide,
    
    /// Order status (NEW, FILLED, etc.)
    pub status: OrderStatus,
    
    /// Stop price (for conditional orders)
    pub stopPrice: String,
    
    /// Close position flag
    pub closePosition: bool,
    
    /// Trading symbol (e.g., "BTCUSD_200925")
    pub symbol: String,
    
    /// Trading pair (e.g., "BTCUSD")
    pub pair: String,
    
    /// Time in force
    pub timeInForce: TimeInForce,
    
    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,
    
    /// Original order type
    pub origType: OrderType,
    
    /// Activation price (only for TRAILING_STOP_MARKET orders)
    #[serde(rename = "activatePrice")]
    pub activate_price: Option<String>,
    
    /// Price rate (only for TRAILING_STOP_MARKET orders)
    pub priceRate: Option<String>,
    
    /// Order update timestamp
    pub updateTime: u64,
    
    /// Working type (CONTRACT_PRICE or MARK_PRICE)
    pub workingType: WorkingType,
    
    /// Price protection flag
    pub priceProtect: bool,
    
    /// Price match mode
    pub priceMatch: String,
    
    /// Self-trade prevention mode
    pub selfTradePreventionMode: String,
}
