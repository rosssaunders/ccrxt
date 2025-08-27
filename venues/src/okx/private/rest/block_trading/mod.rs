/// Block trading private REST endpoints
///
/// This module contains private endpoints for OKX block trading functionality:
/// - RFQ (Request for Quote) management: create, cancel, execute
/// - Quote management: create, cancel
/// - MMP (Market Maker Protection) configuration
/// - Trade history and RFQ/quote data retrieval
/// - Counterparties and maker instrument settings
///
/// These endpoints require authentication and are used for active trading.
// Counterparties
pub mod get_counterparties;

// RFQ management
pub mod cancel_all_rfqs;
pub mod cancel_batch_rfqs;
pub mod cancel_rfq;
pub mod create_rfq;
pub mod execute_quote;
pub mod get_rfqs;

// Quote management
pub mod cancel_all_after;
pub mod cancel_all_quotes;
pub mod cancel_batch_quotes;
pub mod cancel_quote;
pub mod create_quote;
pub mod get_quotes;

// Maker instrument settings
pub mod get_maker_instrument_settings;
pub mod set_maker_instrument_settings;

// MMP configuration
pub mod get_mmp_config;
pub mod mmp_reset;
pub mod set_mmp_config;

// Trade data
pub mod get_trades;

// Re-export request and response types
// Note: Specific types can be imported individually when needed
// to avoid namespace conflicts
