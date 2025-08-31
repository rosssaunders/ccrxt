pub mod enums;
mod errors;

pub mod rate_limit;
pub mod rate_limiter_trait;

// New modules at root level
pub mod credentials;
pub mod private_client;
pub mod public_client;

pub mod public {
    pub mod rest {
        pub mod spot;
        pub use spot::*;
    }
    pub use self::rest::{
        AggregationType, Get24hrTickerRequest, Get24hrTickerResponse, GetHistoricalKlineRequest,
        GetHistoricalKlineResponse, GetKlineRequest, GetKlineResponse, GetOldTradeRequest,
        GetOldTradeResponse, GetOrderBookAggregationRequest, GetOrderBookAggregationResponse,
        GetOrderBookRequest, GetOrderBookResponse, GetRecentTradesRequest, GetRecentTradesResponse,
        GetServerTimeRequest, GetServerTimeResponse, GetSymbolOrderBookTickerRequest,
        GetSymbolOrderBookTickerResponse, GetSymbolPriceTickerRequest,
        GetSymbolPriceTickerResponse, GetSymbolsRequest, GetSymbolsResponse, HistoricalKline,
        Kline, OldTrade, Symbol, Ticker24hr, Trade,
    };
}

pub mod private {
    pub mod rest {
        pub mod account;
        pub mod sub_account;
        pub mod trading;
        pub mod wallet;

        // Re-export all endpoints at rest level for convenience
        pub use account::*;
        pub use sub_account::*;
        pub use trading::*;
        pub use wallet::*;
    }
    pub use self::rest::get_balances::{Balance, GetBalancesRequest, GetBalancesResponse};
}

// Use new root-level credentials
pub use credentials::Credentials;
pub use enums::*;
pub use errors::{ApiResponse, BingXError, ErrorResponse, Errors};
pub use private::{Balance, GetBalancesRequest, GetBalancesResponse};
// Use the new root-level clients as the main exports
pub use private_client::RestClient as PrivateRestClient;
// Alias for backward compatibility
pub use private_client::RestClient as BingXRestClient;
pub use public::{
    Get24hrTickerRequest, Get24hrTickerResponse, GetHistoricalKlineRequest,
    GetHistoricalKlineResponse, GetKlineRequest, GetKlineResponse, GetOldTradeRequest,
    GetOldTradeResponse, GetOrderBookAggregationRequest, GetOrderBookAggregationResponse,
    GetOrderBookRequest, GetOrderBookResponse, GetRecentTradesRequest, GetRecentTradesResponse,
    GetServerTimeRequest, GetServerTimeResponse, GetSymbolOrderBookTickerRequest,
    GetSymbolOrderBookTickerResponse, GetSymbolPriceTickerRequest, GetSymbolPriceTickerResponse,
    GetSymbolsRequest, GetSymbolsResponse, HistoricalKline, Kline, OldTrade, Symbol, Ticker24hr,
    Trade,
};
pub use public_client::RestClient as PublicRestClient;
pub use rate_limit::{EndpointType, RateLimit, RateLimitError, RateLimiter};
pub use rate_limiter_trait::BingXRateLimiter;

/// Type alias for results returned by BingX API operations
pub type RestResult<T> = Result<T, Errors>;
