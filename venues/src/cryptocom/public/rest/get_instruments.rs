//! Request and response structs for public/get-instruments endpoint
//!
//! Provides information on all supported instruments (e.g. BTCUSD-PERP).

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use crate::crypto_com::enums::InstrumentType;

/// Response for public/get-instruments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInstrumentsResponse {
    /// Response id
    pub id: i64,

    /// Method name
    pub method: Cow<'static, str>,

    /// Response code
    pub code: i32,

    /// Result data
    pub result: InstrumentsResult,
}

/// Result data for instruments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstrumentsResult {
    /// List of instruments
    pub data: Vec<Instrument>,
}

/// Instrument object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instrument {
    /// Symbol, e.g. BTCUSD-PERP
    pub symbol: Cow<'static, str>,

    /// Instrument type, e.g. PERPETUAL_SWAP
    pub inst_type: Cow<'static, str>,

    /// Display name, e.g. BTCUSD Perpetual
    pub display_name: Cow<'static, str>,

    /// Base currency, e.g. BTC
    pub base_ccy: Cow<'static, str>,

    /// Quote currency, e.g. USD
    pub quote_ccy: Cow<'static, str>,

    /// Minimum decimal place for price field
    pub quote_decimals: u32,

    /// Minimum decimal place for qty field
    pub quantity_decimals: u32,

    /// Minimum price tick size
    pub price_tick_size: Cow<'static, str>,

    /// Minimum trading quantity / tick size
    pub qty_tick_size: Cow<'static, str>,

    /// Max leverage of the product
    pub max_leverage: Cow<'static, str>,

    /// True if tradable
    pub tradable: bool,

    /// Expiry timestamp in millisecond
    pub expiry_timestamp_ms: Option<i64>,

    /// Underlying symbol
    pub underlying_symbol: Option<Cow<'static, str>>,
}
