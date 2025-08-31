// Market Data endpoints as documented in docs/market_data.md

pub mod get_apr_history;
pub mod get_book_summary_by_currency;
pub mod get_book_summary_by_instrument;
pub mod get_combo_details;
pub mod get_combo_ids;
pub mod get_combos;
pub mod get_contract_size;
pub mod get_currencies;
pub mod get_delivery_prices;
pub mod get_expirations;
pub mod get_funding_chart_data;
pub mod get_funding_rate_history;
pub mod get_funding_rate_value;
pub mod get_historical_volatility;
pub mod get_index;
pub mod get_index_price;
pub mod get_index_price_names;
pub mod get_instrument;
pub mod get_instruments;
pub mod get_last_settlements_by_currency;
pub mod get_last_settlements_by_instrument;
pub mod get_last_trades_by_currency;
pub mod get_last_trades_by_currency_and_time;
pub mod get_last_trades_by_instrument;
pub mod get_last_trades_by_instrument_and_time;
pub mod get_mark_price_history;
pub mod get_order_book;
pub mod get_order_book_by_instrument_id;
pub mod get_rfqs;
pub mod get_supported_index_names;
pub mod get_time;
pub mod get_trade_volumes;
pub mod get_tradingview_chart_data;
pub mod get_volatility_index_data;
pub mod status;

// Re-export all the types from the endpoints
pub use get_apr_history::*;
pub use get_book_summary_by_currency::*;
pub use get_book_summary_by_instrument::*;
pub use get_combo_details::*;
pub use get_combo_ids::*;
pub use get_combos::*;
pub use get_contract_size::*;
pub use get_currencies::*;
pub use get_delivery_prices::*;
pub use get_expirations::*;
pub use get_funding_chart_data::*;
pub use get_funding_rate_history::*;
pub use get_funding_rate_value::*;
pub use get_historical_volatility::*;
pub use get_index::*;
pub use get_index_price::*;
pub use get_index_price_names::*;
pub use get_instrument::*;
pub use get_instruments::{
    GetInstrumentsRequest, GetInstrumentsResponse, InstrumentData as InstrumentsInstrumentData,
};
// Import settlement endpoints with specific naming to avoid SettlementEntry conflicts
pub use get_last_settlements_by_currency::{
    GetLastSettlementsByCurrencyRequest, GetLastSettlementsByCurrencyResponse,
    GetLastSettlementsByCurrencyResult, SettlementEntry,
};
pub use get_last_settlements_by_instrument::{
    GetLastSettlementsByInstrumentRequest, GetLastSettlementsByInstrumentResponse,
    GetLastSettlementsByInstrumentResult, SettlementEntry as InstrumentSettlementEntry,
};
// Import trade endpoints with specific naming to avoid TradeEntry conflicts
pub use get_last_trades_by_currency::{
    GetLastTradesByCurrencyRequest, GetLastTradesByCurrencyResponse, GetLastTradesByCurrencyResult,
    TradeEntry,
};
pub use get_last_trades_by_currency_and_time::{
    GetLastTradesByCurrencyAndTimeRequest, GetLastTradesByCurrencyAndTimeResponse,
    GetLastTradesByCurrencyAndTimeResult, TradeEntry as CurrencyTimeTradeEntry,
};
pub use get_last_trades_by_instrument::{
    GetLastTradesByInstrumentRequest, GetLastTradesByInstrumentResponse,
    GetLastTradesByInstrumentResult, TradeEntry as InstrumentTradeEntry,
};
pub use get_last_trades_by_instrument_and_time::{
    GetLastTradesByInstrumentAndTimeRequest, GetLastTradesByInstrumentAndTimeResponse,
    GetLastTradesByInstrumentAndTimeResult, TradeEntry as InstrumentTimeTradeEntry,
};
pub use get_mark_price_history::*;
pub use get_order_book::*;
pub use get_order_book_by_instrument_id::*;
pub use get_rfqs::*;
pub use get_supported_index_names::*;
pub use get_time::*;
pub use get_trade_volumes::*;
pub use get_tradingview_chart_data::*;
pub use get_volatility_index_data::*;
pub use status::*;
