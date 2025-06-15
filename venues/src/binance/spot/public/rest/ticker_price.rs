use crate::binance::spot::public::rest::RestClient;
use crate::binance::spot::RestResult;
use serde::Deserialize;

/// Represents the price ticker response for a single symbol.
///
/// See: <https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#symbol-price-ticker>
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerPriceResponse {
    /// Symbol
    pub symbol: String,
    /// Latest price
    pub price: String,
}

/// Enum to handle both single symbol and multiple symbols responses
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum TickerPriceResponseWrapper {
    /// Single symbol response
    Single(TickerPriceResponse),
    /// Multiple symbols response
    Multiple(Vec<TickerPriceResponse>),
}

impl RestClient {
    /// Symbol price ticker
    /// 
    /// Latest price for a symbol or symbols.
    /// Weight: 2 for single symbol, 4 for all symbols
    pub async fn ticker_price(&self, symbol: Option<&str>, symbols: Option<&[&str]>) -> RestResult<TickerPriceResponseWrapper> {
        let mut query_params = vec![];
        let symbol_str;
        let symbols_str;
        
        let weight = if symbol.is_some() {
            2
        } else if symbols.is_some() {
            4
        } else {
            4 // All symbols
        };
        
        if let Some(s) = symbol {
            symbol_str = s.to_string();
            query_params.push(("symbol", symbol_str.as_str()));
        }
        if let Some(s) = symbols {
            symbols_str = serde_json::to_string(s).unwrap_or_default();
            query_params.push(("symbols", &symbols_str));
        }
        
        let query_string = if query_params.is_empty() {
            None
        } else {
            Some(serde_urlencoded::to_string(&query_params)
                .map_err(|e| crate::binance::spot::Errors::Error(format!("URL encoding error: {}", e)))?)
        };
        
        self.send_request("/api/v3/ticker/price", reqwest::Method::GET, query_string.as_deref(), None, weight)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binance::spot::RateLimiter;

    #[tokio::test]
    async fn test_ticker_price_method_exists() {
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new();
        let rest_client = RestClient::new("https://api.binance.com", client, rate_limiter);

        // Test that the ticker_price method is accessible
        // We're not calling it to avoid network requests in tests
        let _ = &rest_client.ticker_price(Some("BTCUSDT"), None);
    }
}