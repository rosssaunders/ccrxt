pub mod client;
pub use client::UsdmClient;

// Order management endpoints
pub mod account_trades;
pub mod account_v3;
pub mod all_orders;
pub mod batch_order;
pub mod cancel_all_open_orders;
pub mod cancel_batch_orders;
pub mod cancel_order;
pub mod countdown_cancel_all;
pub mod current_open_order;
pub mod force_orders;
pub mod modify_batch_orders;
pub mod modify_order;
pub mod new_order;
pub mod new_order_test;
pub mod open_orders;
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
pub mod get_income_download_id;
pub mod get_income_download_link;
pub mod get_order_download_id;
pub mod get_order_download_link;
pub mod get_trade_download_id;
pub mod get_trade_download_link;

// Fee management endpoints
pub mod fee_burn_status;
pub mod fee_burn_toggle;

// Convert endpoints
pub mod convert_accept_quote;
pub mod convert_exchange_info;
pub mod convert_order_status;
pub mod convert_quote;

// Portfolio margin endpoints
pub mod portfolio_margin;
