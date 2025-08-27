// Product-specific modules organized by OKX API documentation structure
pub mod affiliate; // Affiliate program
pub mod block_trading; // Block trading
pub mod financial_product; // Financial products (staking, earning)
pub mod funding; // Funding Account
pub mod funding_account; // Funding Account endpoints
pub mod public_data; // Public data that requires authentication
pub mod spread_trading; // Spread trading
pub mod subaccount; // Sub-account management
pub mod trade; // Order Book Trading - Trade
pub mod trading_account; // Trading Account endpoints (previously account)

// Re-export key types from trading_account module
// Re-export key types from trade module
// Re-export key types from block_trading and public_data modules
pub use block_trading::get_counterparties::Counterparty;
pub use public_data::get_economic_calendar::{EconomicCalendarEvent, GetEconomicCalendarRequest};
pub use trade::{
    amend_order::{AmendOrderRequest, AmendOrderResponse},
    cancel_batch_orders::CancelBatchOrdersResponse,
    cancel_order::{CancelOrderRequest, CancelOrderResponse},
    close_position::{ClosePositionRequest, ClosePositionResponse},
    get_fills::{Fill, GetFillsRequest},
    get_order::{GetOrderRequest, OrderDetails},
    get_order_history::GetOrderHistoryRequest,
    get_pending_orders::GetPendingOrdersRequest,
    place_batch_orders::{PlaceBatchOrdersRequest, PlaceBatchOrdersResponse},
    place_order::{AttachedAlgoOrder, PlaceOrderRequest, PlaceOrderResponse},
};
pub use trading_account::{
    get_account_balance::{AccountBalance, BalanceDetail, GetAccountBalanceRequest},
    get_account_config::{AccountConfig, GetAccountConfigRequest, IpRestriction},
    get_positions::{CloseOrderAlgo, GetPositionsRequest, Position},
};

pub use crate::okx::{private_client::RestClient, response::ApiResponse};
