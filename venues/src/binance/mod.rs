pub mod coinm;
pub mod options;
pub mod shared;
pub mod spot;
pub mod usdm;

// Re-export modules for top-level access
pub use coinm::*;
// Re-export response types and structures
pub use spot::public::rest::agg_trades::AggTrade;
// Re-export request types
pub use spot::public::rest::agg_trades::AggTradesRequest;
// Re-export spot public REST types for integration tests
pub use spot::{ApiError, Errors, PublicRestClient, RateLimiter, ResponseHeaders};
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
    *,
};
