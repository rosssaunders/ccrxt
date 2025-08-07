pub mod enums;
mod errors;
mod rate_limit;

pub mod private {
    pub mod rest;
    pub use self::rest::RestClient;
}

pub mod public {
    pub mod rest;
    pub use self::rest::RestClient;
}

pub use enums::*;
pub use errors::{ApiError, ErrorResponse, Errors};
pub use private::RestClient as PrivateRestClient;
// Re-export balance types for convenience
pub use private::rest::{AssetBalance, AssetBalancesResponse, SingleAssetBalanceResponse};
// Re-export order types for convenience
pub use private::rest::{CreateOrderRequest, CreateOrderResponse, GetOrdersParams, Order};
// Re-export trade types for convenience
pub use private::rest::{GetTradesParams, Trade};
// Re-export wallet transaction types for convenience
pub use private::rest::{
    GetWalletTransactionsParams, TransactionStatus, TransactionType, WalletTransaction,
    WalletTransactionsResponse,
};
// Re-export trading account types for convenience
pub use private::rest::{TradingAccount, TradingAccountsResponse};
pub use public::RestClient as PublicRestClient;
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
pub use rate_limit::{EndpointType, RateLimit, RateLimitError, RateLimiter};

/// Type alias for results returned by Bullish API operations
pub type RestResult<T> = Result<T, Errors>;
