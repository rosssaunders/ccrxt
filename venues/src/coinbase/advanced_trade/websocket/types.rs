use super::error;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use websockets::VenueMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookLevel(pub Decimal, pub Decimal);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookSnapshot {
    pub product_id: String,
    pub bids: Vec<OrderBookLevel>,
    pub asks: Vec<OrderBookLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderSide {
    #[serde(rename = "BUY")]
    Buy,
    #[serde(rename = "SELL")]
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderType {
    #[serde(rename = "LIMIT")]
    Limit,
    #[serde(rename = "MARKET")]
    Market,
    #[serde(rename = "STOP_LIMIT")]
    StopLimit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderStatus {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "OPEN")]
    Open,
    #[serde(rename = "FILLED")]
    Filled,
    #[serde(rename = "CANCEL_QUEUED")]
    CancelQueued,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "EXPIRED")]
    Expired,
    #[serde(rename = "FAILED")]
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeInForce {
    #[serde(rename = "UNKNOWN_TIME_IN_FORCE")]
    Unknown,
    #[serde(rename = "GOOD_UNTIL_DATE_TIME")]
    GoodUntilDateTime,
    #[serde(rename = "GOOD_UNTIL_CANCELLED")]
    GoodUntilCancelled,
    #[serde(rename = "IMMEDIATE_OR_CANCEL")]
    ImmediateOrCancel,
    #[serde(rename = "FILL_OR_KILL")]
    FillOrKill,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProductType {
    #[serde(rename = "UNKNOWN_PRODUCT_TYPE")]
    Unknown,
    #[serde(rename = "SPOT")]
    Spot,
    #[serde(rename = "FUTURE")]
    Future,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskManagementType {
    #[serde(rename = "UNKNOWN_RISK_MANAGEMENT_TYPE")]
    Unknown,
    #[serde(rename = "MANAGED_BY_FCM")]
    ManagedByFcm,
    #[serde(rename = "MANAGED_BY_VENUE")]
    ManagedByVenue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TriggerStatus {
    #[serde(rename = "UNKNOWN_TRIGGER_STATUS")]
    Unknown,
    #[serde(rename = "INVALID_ORDER_TYPE")]
    InvalidOrderType,
    #[serde(rename = "STOP_PENDING")]
    StopPending,
    #[serde(rename = "STOP_TRIGGERED")]
    StopTriggered,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContractExpiryType {
    #[serde(rename = "UNKNOWN_CONTRACT_EXPIRY")]
    Unknown,
    #[serde(rename = "EXPIRING")]
    Expiring,
    #[serde(rename = "PERPETUAL")]
    Perpetual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarginType {
    #[serde(rename = "Cross")]
    Cross,
    #[serde(rename = "Isolated")]
    Isolated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PositionSide {
    #[serde(rename = "Long")]
    Long,
    #[serde(rename = "Short")]
    Short,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarginWindowType {
    #[serde(rename = "FCM_MARGIN_WINDOW_TYPE_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "FCM_MARGIN_WINDOW_TYPE_OVERNIGHT")]
    Overnight,
    #[serde(rename = "FCM_MARGIN_WINDOW_TYPE_WEEKEND")]
    Weekend,
    #[serde(rename = "FCM_MARGIN_WINDOW_TYPE_INTRADAY")]
    Intraday,
    #[serde(rename = "FCM_MARGIN_WINDOW_TYPE_TRANSITION")]
    Transition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarginLevelType {
    #[serde(rename = "MARGIN_LEVEL_TYPE_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "MARGIN_LEVEL_TYPE_BASE")]
    Base,
    #[serde(rename = "MARGIN_LEVEL_TYPE_WARNING")]
    Warning,
    #[serde(rename = "MARGIN_LEVEL_TYPE_DANGER")]
    Danger,
    #[serde(rename = "MARGIN_LEVEL_TYPE_LIQUIDATION")]
    Liquidation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookUpdate {
    pub side: OrderSide,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub event_time: DateTime<Utc>,
    pub price_level: Decimal,
    pub new_quantity: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookEvent {
    pub type_: String,
    pub product_id: String,
    pub updates: Vec<OrderBookUpdate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookMessage {
    pub channel: String,
    pub client_id: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
    pub sequence_num: i64,
    pub events: Vec<OrderBookEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketTradesEvent {
    pub type_: String,
    pub product_id: String,
    pub trades: Vec<Trade>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub trade_id: String,
    pub price: Decimal,
    pub size: Decimal,
    pub side: OrderSide,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketTradesMessage {
    pub channel: String,
    pub client_id: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
    pub sequence_num: i64,
    pub events: Vec<MarketTradesEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickerEvent {
    pub type_: String,
    pub product_id: String,
    pub updates: Vec<TickerUpdate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickerUpdate {
    #[serde(with = "chrono::serde::ts_seconds")]
    pub event_time: DateTime<Utc>,
    pub price: Decimal,
    pub volume_24h: Decimal,
    pub volume_30d: Decimal,
    pub trades_24h: i64,
    pub low_24h: Decimal,
    pub high_24h: Decimal,
    pub open_24h: Decimal,
    pub close_24h: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickerMessage {
    pub channel: String,
    pub client_id: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
    pub sequence_num: i64,
    pub events: Vec<TickerEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandlesEvent {
    pub type_: String,
    pub product_id: String,
    pub candles: Vec<Candle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    #[serde(with = "chrono::serde::ts_seconds")]
    pub start: DateTime<Utc>,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandlesMessage {
    pub channel: String,
    pub client_id: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
    pub sequence_num: i64,
    pub events: Vec<CandlesEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEvent {
    pub type_: String,
    pub products: Vec<ProductStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductStatus {
    pub product_id: String,
    pub status: String,
    pub trading_disabled: bool,
    pub cancel_only: bool,
    pub post_only: bool,
    pub limit_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusMessage {
    pub channel: String,
    pub client_id: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
    pub sequence_num: i64,
    pub events: Vec<StatusEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebSocketMessage {
    OrderBook(OrderBookMessage),
    MarketTrades(MarketTradesMessage),
    Ticker(TickerMessage),
    Candles(CandlesMessage),
    Status(StatusMessage),
    Raw(Value),
}

impl VenueMessage for WebSocketMessage {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionMessage {
    pub type_: String,
    pub channel: String,
    pub product_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMessage {
    pub type_: String,
    pub message: String,
}

impl TryFrom<Value> for OrderBookMessage {
    type Error = error::CoinbaseAdvancedTradeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
            .map_err(|e| error::CoinbaseAdvancedTradeError::ParseError(e.to_string()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub avg_price: Decimal,
    pub cancel_reason: String,
    pub client_order_id: String,
    pub completion_percentage: Decimal,
    pub contract_expiry_type: ContractExpiryType,
    pub cumulative_quantity: Decimal,
    pub filled_value: Decimal,
    pub leaves_quantity: Decimal,
    pub limit_price: Decimal,
    pub number_of_fills: String,
    pub order_id: String,
    pub order_side: OrderSide,
    pub order_type: OrderType,
    pub outstanding_hold_amount: Decimal,
    pub post_only: String,
    pub product_id: String,
    pub product_type: ProductType,
    pub reject_reason: String,
    pub retail_portfolio_id: String,
    pub risk_managed_by: RiskManagementType,
    pub status: OrderStatus,
    pub stop_price: Decimal,
    pub time_in_force: TimeInForce,
    pub total_fees: Decimal,
    pub total_value_after_fees: Decimal,
    pub trigger_status: TriggerStatus,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub creation_time: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub end_time: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub start_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerpetualFuturesPosition {
    pub product_id: String,
    pub portfolio_uuid: String,
    pub vwap: Decimal,
    pub entry_vwap: Decimal,
    pub position_side: PositionSide,
    pub margin_type: MarginType,
    pub net_size: Decimal,
    pub buy_order_size: Decimal,
    pub sell_order_size: Decimal,
    pub leverage: Decimal,
    pub mark_price: Decimal,
    pub liquidation_price: Decimal,
    pub im_notional: Decimal,
    pub mm_notional: Decimal,
    pub position_notional: Decimal,
    pub unrealized_pnl: Decimal,
    pub aggregated_pnl: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpiringFuturesPosition {
    pub product_id: String,
    pub side: PositionSide,
    pub number_of_contracts: Decimal,
    pub realized_pnl: Decimal,
    pub unrealized_pnl: Decimal,
    pub entry_price: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginWindowMeasure {
    pub margin_window_type: MarginWindowType,
    pub margin_level: MarginLevelType,
    pub initial_margin: Decimal,
    pub maintenance_margin: Decimal,
    pub liquidation_buffer_percentage: Decimal,
    pub total_hold: Decimal,
    pub futures_buying_power: Decimal,
}
