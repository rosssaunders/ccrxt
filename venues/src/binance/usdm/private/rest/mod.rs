pub mod client;
pub use client::RestClient;

// Order management endpoints
pub mod account;
pub mod account_trades;
pub mod all_orders;
pub mod batch_order;
pub mod batch_order_modify;
pub mod cancel_all_orders;
pub mod current_open_order;
pub mod force_orders;
pub mod open_orders;
pub mod order;
pub mod order_amendment;
pub mod position_risk;
pub mod query_order;

// Position & margin management endpoints
pub mod leverage;
pub mod margin_type;
pub mod multi_assets_mode;
pub mod position_margin;
pub mod position_mode;

// Advanced position & margin management endpoints
pub mod adl_quantile;
pub mod position_margin_history;
pub mod position_risk_v3;

// Advanced account information endpoints
pub mod account_config;
pub mod account_v2;
pub mod api_trading_status;
pub mod balance_v2;
pub mod balance_v3;
pub mod commission_rate;
pub mod income_history;
pub mod leverage_bracket;
pub mod multi_assets_margin_status;
pub mod position_mode_status;
pub mod rate_limit_order;
pub mod symbol_config;

// Data download (async) endpoints
pub mod income_download;
pub mod order_download;
pub mod trade_download;

// Fee management endpoints
pub mod fee_management;

// Convert endpoints
pub mod convert;

// Portfolio margin endpoints
pub mod portfolio_margin;
