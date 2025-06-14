// Portfolio Margin enums - reuse COIN-M enums since they're identical for trading operations
pub use crate::binance::coinm::{
    ContractStatus, ContractType, ExchangeFilterType, IncomeType, KlineInterval, MarginType,
    OrderResponseType, OrderSide, OrderStatus, OrderType, PositionSide, PriceMatch,
    SelfTradePreventionMode, SymbolFilterType, SymbolStatus, SymbolType, TimeInForce,
    UnderlyingType, WebSocketEventType, WorkingType,
};
