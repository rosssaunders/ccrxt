use crate::binance::spot::public::rest::RestClient;
use crate::binance::spot::RestResult;
use serde::Deserialize;

/// Represents 24hr ticker price change statistics.
///
/// See: <https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#24hr-ticker-price-change-statistics>
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker24hrResponse {
    /// Symbol
    pub symbol: String,
    /// Price change
    pub price_change: String,
    /// Price change percent
    pub price_change_percent: String,
    /// Weighted average price
    pub weighted_avg_price: String,
    /// Previous close price
    pub prev_close_price: String,
    /// Last price
    pub last_price: String,
    /// Last quantity
    pub last_qty: String,
    /// Best bid price
    pub bid_price: String,
    /// Best bid quantity
    pub bid_qty: String,
    /// Best ask price
    pub ask_price: String,
    /// Best ask quantity
    pub ask_qty: String,
    /// Open price
    pub open_price: String,
    /// High price
    pub high_price: String,
    /// Low price
    pub low_price: String,
    /// Total traded base asset volume
    pub volume: String,
    /// Total traded quote asset volume
    pub quote_volume: String,
    /// Statistics open time
    pub open_time: i64,
    /// Statistics close time
    pub close_time: i64,
    /// First trade ID
    pub first_id: i64,
    /// Last trade ID
    pub last_id: i64,
    /// Total number of trades
    pub count: i64,
}

/// Enum to handle both single symbol and multiple symbols responses
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Ticker24hrResponseWrapper {
    /// Single symbol response
    Single(Ticker24hrResponse),
    /// Multiple symbols response
    Multiple(Vec<Ticker24hrResponse>),
}

impl RestClient {
    /// 24hr ticker price change statistics
    /// 
    /// Weight: 2 for single symbol, 80 for all symbols
    pub async fn ticker_24hr(&self, symbol: Option<&str>, symbols: Option<&[&str]>, ticker_type: Option<&str>) -> RestResult<Ticker24hrResponseWrapper> {
        let mut query_params = vec![];
        let symbol_str;
        let symbols_str;
        let ticker_type_str;
        
        let weight = if symbol.is_some() {
            2
        } else if symbols.is_some() {
            2 // Weight based on number of symbols, simplified to 2 for now
        } else {
            80 // All symbols
        };
        
        if let Some(s) = symbol {
            symbol_str = s.to_string();
            query_params.push(("symbol", symbol_str.as_str()));
        }
        if let Some(s) = symbols {
            symbols_str = serde_json::to_string(s).unwrap_or_default();
            query_params.push(("symbols", &symbols_str));
        }
        if let Some(tt) = ticker_type {
            ticker_type_str = tt.to_string();
            query_params.push(("type", ticker_type_str.as_str()));
        }
        
        let query_string = if query_params.is_empty() {
            None
        } else {
            Some(serde_urlencoded::to_string(&query_params)
                .map_err(|e| crate::binance::spot::Errors::Error(format!("URL encoding error: {}", e)))?)
        };
        
        self.send_request("/api/v3/ticker/24hr", reqwest::Method::GET, query_string.as_deref(), None, weight)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binance::spot::RateLimiter;

    #[tokio::test]
    async fn test_ticker_24hr_method_exists() {
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new();
        let rest_client = RestClient::new("https://api.binance.com", client, rate_limiter);

        // Test that the ticker_24hr method is accessible
        // We're not calling it to avoid network requests in tests
        let _ = &rest_client.ticker_24hr(Some("BTCUSDT"), None, None);
    }
}