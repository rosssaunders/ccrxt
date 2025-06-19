pub mod enums;
mod errors;

pub mod rate_limit;

pub mod public {
    pub mod rest;
    pub use self::rest::RestClient as PublicRestClient;
    pub use self::rest::{
        GetServerTimeRequest, GetServerTimeResponse,
        GetSymbolsRequest, GetSymbolsResponse, Symbol,
        GetRecentTradesRequest, GetRecentTradesResponse, Trade,
        GetOrderBookRequest, GetOrderBookResponse,
        GetKlineRequest, GetKlineResponse, Kline,
        Get24hrTickerRequest, Get24hrTickerResponse, Ticker24hr,
        GetOrderBookAggregationRequest, GetOrderBookAggregationResponse,
        GetSymbolPriceTickerRequest, GetSymbolPriceTickerResponse,
        GetSymbolOrderBookTickerRequest, GetSymbolOrderBookTickerResponse,
        GetHistoricalKlineRequest, GetHistoricalKlineResponse, HistoricalKline,
        GetOldTradeRequest, GetOldTradeResponse, OldTrade,
    };
}

pub mod private {
    mod rest;
    pub use self::rest::RestClient;
    pub use self::rest::{Balance, GetBalancesRequest, GetBalancesResponse};
}

pub use enums::*;
pub use errors::{ErrorResponse, Errors};
pub use public::PublicRestClient;
pub use private::RestClient as PrivateRestClient;
pub use private::{Balance, GetBalancesRequest, GetBalancesResponse};
pub use rate_limit::{EndpointType, RateLimit, RateLimitError, RateLimiter};

/// Type alias for results returned by BingX API operations
pub type RestResult<T> = Result<T, Errors>;
