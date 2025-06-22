pub mod enums;
mod errors;

pub mod rate_limit;

pub mod public {
    pub mod rest;
    pub use self::rest::RestClient as PublicRestClient;
    pub use self::rest::{
        Get24hrTickerRequest, Get24hrTickerResponse, GetHistoricalKlineRequest, GetHistoricalKlineResponse, GetKlineRequest, GetKlineResponse,
        GetOldTradeRequest, GetOldTradeResponse, GetOrderBookAggregationRequest, GetOrderBookAggregationResponse, GetOrderBookRequest, GetOrderBookResponse,
        GetRecentTradesRequest, GetRecentTradesResponse, GetServerTimeRequest, GetServerTimeResponse, GetSymbolOrderBookTickerRequest,
        GetSymbolOrderBookTickerResponse, GetSymbolPriceTickerRequest, GetSymbolPriceTickerResponse, GetSymbolsRequest, GetSymbolsResponse, HistoricalKline,
        Kline, OldTrade, Symbol, Ticker24hr, Trade,
    };
}

pub mod private {
    mod rest;
    pub use self::rest::RestClient;
    pub use self::rest::{Balance, GetBalancesRequest, GetBalancesResponse};
}

pub use enums::*;
pub use errors::{ErrorResponse, Errors};
pub use private::RestClient as PrivateRestClient;
pub use private::{Balance, GetBalancesRequest, GetBalancesResponse};
pub use public::PublicRestClient;
pub use rate_limit::{EndpointType, RateLimit, RateLimitError, RateLimiter};

/// Type alias for results returned by BingX API operations
pub type RestResult<T> = Result<T, Errors>;
