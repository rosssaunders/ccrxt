pub mod client;

// Market Data endpoints
pub mod get_kline;
pub mod get_orderbook;
pub mod get_recent_trades;
pub mod get_server_time;
pub mod get_tickers;

// Price Kline endpoints
pub mod get_index_price_kline;
pub mod get_mark_price_kline;
pub mod get_premium_index_price_kline;

// Instrument & Trading endpoints
pub mod get_funding_history;
pub mod get_instruments_info;

// Market Statistics endpoints
pub mod get_historical_volatility;
pub mod get_open_interest;

// Risk Management endpoints
pub mod get_delivery_price;
pub mod get_insurance;
pub mod get_risk_limit;

// Account Ratio endpoint
pub mod get_long_short_ratio;

// Margin Trade endpoints
pub mod get_collateral_ratio;
pub mod get_vip_margin_data;

// Loan endpoints
pub mod get_borrowable_coins;
pub mod get_collateral_coins;
pub mod get_ins_margin_coin_info;
pub mod get_ins_product_info;

pub use client::RestClient;
pub use get_instruments_info::{
    GetInstrumentsInfoData, GetInstrumentsInfoRequest, GetInstrumentsInfoResponse, InstrumentInfo,
};
pub use get_kline::{GetKlineData, GetKlineRequest, GetKlineResponse, Kline};
pub use get_orderbook::{
    GetOrderbookData, GetOrderbookRequest, GetOrderbookResponse, OrderbookLevel,
};
pub use get_recent_trades::{
    GetRecentTradesData, GetRecentTradesRequest, GetRecentTradesResponse, TradeInfo,
};
// Re-export key types for integration tests
pub use get_server_time::{GetServerTimeRequest, GetServerTimeResponse, ServerTimeData};
pub use get_tickers::{GetTickersData, GetTickersRequest, GetTickersResponse, TickerInfo};
