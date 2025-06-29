pub mod client;

// Market Data endpoints
pub mod get_server_time;
pub mod get_kline;
pub mod get_orderbook;
pub mod get_tickers;
pub mod get_recent_trades;

// Price Kline endpoints
pub mod get_mark_price_kline;
pub mod get_index_price_kline;
pub mod get_premium_index_price_kline;

// Instrument & Trading endpoints
pub mod get_instruments_info;
pub mod get_funding_history;

// Market Statistics endpoints
pub mod get_open_interest;
pub mod get_historical_volatility;

// Risk Management endpoints
pub mod get_insurance;
pub mod get_risk_limit;
pub mod get_delivery_price;

// Account Ratio endpoint
pub mod get_long_short_ratio;

// Margin Trade endpoints
pub mod get_vip_margin_data;
pub mod get_collateral_ratio;

// Loan endpoints
pub mod get_collateral_coins;
pub mod get_borrowable_coins;
pub mod get_ins_product_info;
pub mod get_ins_margin_coin_info;

pub use client::RestClient;