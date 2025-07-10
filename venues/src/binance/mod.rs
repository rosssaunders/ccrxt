pub mod coinm;
pub mod options;
pub mod shared;
pub mod spot;
pub mod usdm;

// Re-export modules for top-level access
pub use coinm::*;
pub use spot::*;

// Re-export spot public REST types for integration tests
pub use spot::{ApiError, Errors, PublicRestClient, RateLimiter, ResponseHeaders};

// Re-export request types
pub use spot::public::rest::agg_trades::AggTradesRequest;
pub use spot::public::rest::avg_price::AvgPriceRequest;
pub use spot::public::rest::depth::DepthRequest;
pub use spot::public::rest::exchange_info::ExchangeInfoRequest;
pub use spot::public::rest::historical_trades::HistoricalTradesRequest;
pub use spot::public::rest::klines::KlinesRequest;
pub use spot::public::rest::ticker::TickerRequest;
pub use spot::public::rest::ticker_24hr::Ticker24hrRequest;
pub use spot::public::rest::ticker_book::TickerBookRequest;
pub use spot::public::rest::ticker_price::TickerPriceRequest;
pub use spot::public::rest::ticker_trading_day::TickerTradingDayRequest;
pub use spot::public::rest::trades::TradesRequest;
pub use spot::public::rest::ui_klines::UiKlinesRequest;

// Re-export response types and structures
pub use spot::public::rest::agg_trades::AggTrade;
pub use spot::public::rest::avg_price::AvgPriceResponse;
pub use spot::public::rest::depth::DepthResponse;
pub use spot::public::rest::exchange_info::ExchangeInfoResponse;
pub use spot::public::rest::historical_trades::HistoricalTrade;
pub use spot::public::rest::klines::KlineData;
pub use spot::public::rest::ping::PingResponse;
pub use spot::public::rest::server_time::ServerTimeResponse;
pub use spot::public::rest::ticker::Ticker;
pub use spot::public::rest::ticker_24hr::Ticker24hr;
pub use spot::public::rest::ticker_book::TickerBook;
pub use spot::public::rest::ticker_price::TickerPrice;
pub use spot::public::rest::ticker_trading_day::TickerTradingDay;
pub use spot::public::rest::trades::Trade;
