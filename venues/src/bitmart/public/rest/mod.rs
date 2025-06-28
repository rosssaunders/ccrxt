mod client;
mod get_currency_list;
mod get_depth;
mod get_history_kline;
mod get_latest_kline;
mod get_recent_trades;
mod get_ticker;
mod get_ticker_all_pairs;
mod get_trading_pair_details;
mod get_trading_pairs_list;

pub use client::RestClient;
pub use get_currency_list::{Currency, GetCurrencyListRequest, GetCurrencyListResponse};
pub use get_depth::{DepthData, GetDepthRequest, GetDepthResponse, OrderBookEntry};
pub use get_history_kline::{
    GetHistoryKlineRequest, GetHistoryKlineResponse, KlineData as HistoryKlineData,
};
pub use get_latest_kline::{
    GetLatestKlineRequest, GetLatestKlineResponse, KlineData as LatestKlineData,
};
pub use get_recent_trades::{GetRecentTradesRequest, GetRecentTradesResponse, TradeData};
pub use get_ticker::{GetTickerRequest, GetTickerResponse, TickerData};
pub use get_ticker_all_pairs::{
    GetTickerAllPairsRequest, GetTickerAllPairsResponse, TickerData as TickerArrayData,
};
pub use get_trading_pair_details::{
    GetTradingPairDetailsRequest, GetTradingPairDetailsResponse, TradingPairDetail,
};
pub use get_trading_pairs_list::{GetTradingPairsListRequest, GetTradingPairsListResponse};
