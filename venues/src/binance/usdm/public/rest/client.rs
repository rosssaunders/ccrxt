// REST client for Binance USD-M public endpoints.
//
// Provides access to all public REST API endpoints for Binance USD-M Futures.
// All requests are unauthenticated and do not require API credentials.
use crate::binance::shared::client::PublicBinanceClient;

pub type RestClient = PublicBinanceClient;
