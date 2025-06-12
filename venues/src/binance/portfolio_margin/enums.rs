// Portfolio Margin enums - reuse COIN-M enums since they're identical for trading operations
pub use crate::binance::coinm::{
    OrderSide, PositionSide, OrderType, TimeInForce, WorkingType, OrderStatus,
    OrderResponseType, SelfTradePreventionMode, IncomeType, MarginType,
    WebSocketEventType, ExchangeFilterType, SymbolStatus, SymbolType,
    ContractType, ContractStatus, UnderlyingType, SymbolFilterType,
    PriceMatch, KlineInterval
};