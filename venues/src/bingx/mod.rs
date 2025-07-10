pub mod enums;
mod errors;

pub mod rate_limit;

pub mod public {
    pub mod rest;
    pub use self::rest::{
        Get24hrTickerRequest, Get24hrTickerResponse, GetHistoricalKlineRequest,
        GetHistoricalKlineResponse, GetKlineRequest, GetKlineResponse, GetOldTradeRequest,
        GetOldTradeResponse, GetOrderBookAggregationRequest, GetOrderBookAggregationResponse,
        GetOrderBookRequest, GetOrderBookResponse, GetRecentTradesRequest, GetRecentTradesResponse,
        GetServerTimeRequest, GetServerTimeResponse, GetSymbolOrderBookTickerRequest,
        GetSymbolOrderBookTickerResponse, GetSymbolPriceTickerRequest,
        GetSymbolPriceTickerResponse, GetSymbolsRequest, GetSymbolsResponse, HistoricalKline,
        Kline, OldTrade, RestClient as PublicRestClient, Symbol, Ticker24hr, Trade,
    };
}

pub mod private {
    mod rest;
    pub use self::rest::{Balance, GetBalancesRequest, GetBalancesResponse, RestClient};
}

pub use enums::*;
pub use errors::{BingXError, ErrorResponse, Errors};
// Alias for backward compatibility
pub use private::RestClient as BingXRestClient;
pub use private::{
    Balance, GetBalancesRequest, GetBalancesResponse, RestClient as PrivateRestClient,
};
pub use public::{
    Get24hrTickerRequest, Get24hrTickerResponse, GetHistoricalKlineRequest,
    GetHistoricalKlineResponse, GetKlineRequest, GetKlineResponse, GetOldTradeRequest,
    GetOldTradeResponse, GetOrderBookAggregationRequest, GetOrderBookAggregationResponse,
    GetOrderBookRequest, GetOrderBookResponse, GetRecentTradesRequest, GetRecentTradesResponse,
    GetServerTimeRequest, GetServerTimeResponse, GetSymbolOrderBookTickerRequest,
    GetSymbolOrderBookTickerResponse, GetSymbolPriceTickerRequest,
    GetSymbolPriceTickerResponse, GetSymbolsRequest, GetSymbolsResponse, HistoricalKline,
    Kline, OldTrade, PublicRestClient, Symbol, Ticker24hr, Trade,
};
pub use rate_limit::{EndpointType, RateLimit, RateLimitError, RateLimiter};

/// Type alias for results returned by BingX API operations
pub type RestResult<T> = Result<T, Errors>;
