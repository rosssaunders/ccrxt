// Trading endpoints as documented in docs/private_trading.md

pub mod cancel_all;
pub mod cancel_all_block_rfq_quotes;
pub mod cancel_all_by_currency;
pub mod cancel_all_by_currency_pair;
pub mod cancel_all_by_instrument;
pub mod cancel_all_by_kind_or_type;
pub mod cancel_block_rfq;
pub mod cancel_block_rfq_quote;
pub mod cancel_by_label;
pub mod cancel_order;
pub mod cancel_quotes;
pub mod cancel_withdrawal;
pub mod get_mmp_status;
pub mod get_open_orders_by_currency;
pub mod get_open_orders_by_instrument;
pub mod get_order_margin_by_ids;
pub mod get_order_state_by_label;
pub mod get_settlement_history_by_instrument;
pub mod get_trigger_order_history;
pub mod get_user_trades_by_currency;
pub mod get_user_trades_by_currency_and_time;
pub mod get_user_trades_by_instrument;
pub mod get_user_trades_by_instrument_and_time;
pub mod get_user_trades_by_order;
pub mod reset_mmp;
pub mod set_mmp_config;

// Re-export all types
pub use cancel_all::*;
pub use cancel_all_block_rfq_quotes::*;
pub use cancel_all_by_currency::*;
pub use cancel_all_by_currency_pair::*;
pub use cancel_all_by_instrument::*;
pub use cancel_all_by_kind_or_type::*;
pub use cancel_block_rfq::*;
pub use cancel_block_rfq_quote::*;
pub use cancel_by_label::*;
pub use cancel_order::*;
pub use cancel_quotes::*;
pub use cancel_withdrawal::*;
pub use get_mmp_status::*;
pub use get_open_orders_by_currency::{
    GetOpenOrdersByCurrencyRequest, GetOpenOrdersByCurrencyResponse,
    OpenOrder as OpenOrderByCurrency, OpenOrderType,
};
pub use get_open_orders_by_instrument::{
    GetOpenOrdersByInstrumentRequest, GetOpenOrdersByInstrumentResponse,
    OpenOrder as OpenOrderByInstrument,
};
pub use get_order_margin_by_ids::*;
pub use get_order_state_by_label::*;
pub use get_settlement_history_by_instrument::*;
pub use get_trigger_order_history::*;
pub use get_user_trades_by_currency::*;
pub use get_user_trades_by_currency_and_time::*;
pub use get_user_trades_by_instrument::*;
pub use get_user_trades_by_instrument_and_time::*;
pub use get_user_trades_by_order::*;
pub use reset_mmp::{IndexName, ResetMmpRequest, ResetMmpResponse};
pub use set_mmp_config::{MmpConfig, SetMmpConfigRequest, SetMmpConfigResponse};
