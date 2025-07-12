pub mod coinm;
pub mod options;
pub mod shared;
pub mod spot;
pub mod usdm;

// Re-export specific types from coinm to avoid conflicts  
pub use coinm::{
    PrivateRestClient as CoinmPrivateRestClient,
    RateLimiter as CoinmRateLimiter,
    ResponseHeaders as CoinmResponseHeaders,
    RestResponse as CoinmRestResponse,
    RestResult as CoinmRestResult,
};
// Re-export response types and structures
pub use spot::public::rest::agg_trades::AggTrade;
// Re-export request types
pub use spot::public::rest::agg_trades::AggTradesRequest;
// Re-export spot types and modules
pub use spot::{
    public::rest::{
        avg_price::{AvgPriceRequest, AvgPriceResponse},
        depth::{DepthRequest, DepthResponse},
        exchange_info::{ExchangeInfoRequest, ExchangeInfoResponse},
        historical_trades::{HistoricalTrade, HistoricalTradesRequest},
        klines::{KlineData, KlinesRequest},
        ping::PingResponse,
        server_time::ServerTimeResponse,
        ticker::{Ticker, TickerRequest},
        ticker_24hr::{Ticker24hr, Ticker24hrRequest},
        ticker_book::{TickerBook, TickerBookRequest},
        ticker_price::{TickerPrice, TickerPriceRequest},
        ticker_trading_day::{TickerTradingDay, TickerTradingDayRequest},
        trades::{Trade, TradesRequest},
        ui_klines::UiKlinesRequest,
    },
    // Specific re-exports from spot to avoid ambiguous glob re-exports
    ApiError, Errors, PublicRestClient, RateLimiter, ResponseHeaders,
    RestResult, RestResponse, ErrorResponse,
    // Re-export enums and order types
    OrderSide, OrderType, TimeInForce, OrderStatus, OrderResponseType, 
    SelfTradePreventionMode, SymbolStatus, KlineInterval,
};
