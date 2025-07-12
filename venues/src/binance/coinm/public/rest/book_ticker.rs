use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, public::rest::RestClient};

/// Parameters for Symbol Order Book Ticker
#[derive(Debug, Clone, Serialize, Default)]
pub struct BookTickerRequestBySymbol {
    /// Symbol name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

/// Parameters for Symbol Order Book Ticker
#[derive(Debug, Clone, Serialize, Default)]
pub struct BookTickerRequestByPair {
    /// Pair name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
}

#[derive(Serialize)]
pub enum BookTickerRequest {
    BySymbol(BookTickerRequestBySymbol),
    ByPair(BookTickerRequestByPair),
}

/// Symbol order book ticker
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookTicker {
    /// Symbol name
    pub symbol: String,
    /// Pair name
    pub pair: String,
    /// Best bid price
    pub bid_price: Decimal,
    /// Best bid quantity
    pub bid_qty: Decimal,
    /// Best ask price
    pub ask_price: Decimal,
    /// Best ask quantity
    pub ask_qty: Decimal,
    /// Timestamp
    pub time: i64,
}

impl RestClient {
    /// Get symbol order book ticker by symbol
    ///
    /// Weight: 2 for a single symbol; 5 when the symbol parameter is omitted
    async fn get_book_ticker_by_symbol(
        &self,
        params: BookTickerRequestBySymbol,
    ) -> RestResult<Vec<BookTicker>> {
        let weight = if params.symbol.is_some() { 2 } else { 5 };

        self.send_request(
            "/dapi/v1/ticker/bookTicker",
            reqwest::Method::GET,
            Some(params),
            weight,
        )
        .await
    }

    /// Get symbol order book ticker by pair
    ///
    /// Weight: 2 for a single pair; 5 when the pair parameter is omitted
    async fn get_book_ticker_by_pair(
        &self,
        params: BookTickerRequestByPair,
    ) -> RestResult<Vec<BookTicker>> {
        let weight = if params.pair.is_some() { 2 } else { 5 };

        self.send_request(
            "/dapi/v1/ticker/bookTicker",
            reqwest::Method::GET,
            Some(params),
            weight,
        )
        .await
    }

    /// Get symbol order book ticker
    ///
    /// Weight: 2 for a single symbol; 5 when the symbol parameter is omitted
    pub async fn get_book_ticker(&self, params: BookTickerRequest) -> RestResult<Vec<BookTicker>> {
        match params {
            BookTickerRequest::BySymbol(by_symbol) => {
                self.get_book_ticker_by_symbol(by_symbol).await
            }
            BookTickerRequest::ByPair(by_pair) => self.get_book_ticker_by_pair(by_pair).await,
        }
    }
}
