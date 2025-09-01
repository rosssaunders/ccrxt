pub mod enums;
mod errors;
pub mod pagination;
mod rate_limit;
pub mod rate_limiter_trait;

// Canonical top-level clients now live in `private_client.rs` and `public_client.rs`.
// Nested `private::rest` / `public::rest` modules provide endpoint type definitions and
// re-export these top-level clients for backwards compatibility of existing impl blocks.

pub mod private {
    pub mod rest;
    // Re-export canonical client so existing `impl RestClient` blocks under rest continue working.
    pub use crate::bullish::private_client::RestClient;
}

pub mod public {
    pub mod rest;
    // Re-export canonical public client
    pub use crate::bullish::public_client::RestClient;
}

mod private_client;
mod public_client;

pub use enums::*;
pub use errors::{ApiError, ErrorResponse, Errors};
pub use pagination::{
    DataOrPaginated, ListResponse, PaginatedResponse, PaginatedResult, PaginationLinks,
    PaginationParams,
};
// Re-export credentials at the top-level for convenience
mod credentials;
pub use credentials::Credentials;
// Re-export balance types for convenience
pub use private::rest::AssetAccount;
// Re-export order types for convenience
pub use private::rest::{
    CreateOrderRequest, CreateOrderResponse, GetOrdersHistoryParams, GetOrdersParams, Order,
};
// Re-export custody types for convenience
pub use private::rest::{
    CustodyCryptoDepositInstructions, CustodyCryptoWithdrawalInstructions,
    CustodyFiatDepositInstructions, CustodyFiatNetwork, CustodyHistory, CustodyHistoryParams,
    CustodyLimits,
};
// Re-export derivatives positions types for convenience
pub use private::rest::{DerivativesPosition, GetDerivativesPositionsParams};
// Re-export history - orders, trades, derivatives settlement
pub use private::rest::{DerivativesSettlementResponse, GetDerivativesSettlementHistoryParams};
pub use private::rest::{GetTradesHistoryParams, HistoryTrade};
// Re-export trade types for convenience
pub use private::rest::{GetTradesParams, Trade};
// Re-export trading account types for convenience
pub use private::rest::{
    GetTradingAccountRequest, TradingAccount, TradingAccountsResponse, TransferAssetCommand,
    TransferAssetRequest, TransferAssetResponse, TransferCommandType,
};
// Re-export wallet transaction types for convenience
pub use private::rest::{
    GetWalletTransactionsParams, TransactionStatus, TransactionType, WalletTransaction,
    WalletTransactionsResponse,
};
// Re-export private client type
pub use private_client::RestClient as PrivateRestClient;
// Re-export index price types for convenience
pub use public::rest::IndexPrice;
// Re-export nonce types for convenience
pub use public::rest::Nonce;
// Re-export time types for convenience
pub use public::rest::ServerTime;
// Re-export ticker types for convenience
pub use public::rest::Ticker;
// Re-export asset types for convenience
pub use public::rest::{Asset, AssetNetwork, AssetStatus, AssetsResponse, SingleAssetResponse};
// Re-export candle types for convenience
pub use public::rest::{Candle, GetCandlesRequest};
// Re-export public orderbook types for convenience
pub use public::rest::{HybridOrderbook, OrderbookEntry, OrderbookRequest};
// Re-export public market types for convenience
pub use public::rest::{Market, MarketStatus, MarketType, MarketsResponse, SingleMarketResponse};
// Re-export public trade types for convenience
pub use public::rest::{PublicTrade, PublicTradesRequest};
pub use public_client::RestClient as PublicRestClient;
pub use rate_limit::{EndpointType, RateLimit, RateLimitError, RateLimiter};

/// Type alias for results returned by Bullish API operations
pub type RestResult<T> = Result<T, Errors>;
