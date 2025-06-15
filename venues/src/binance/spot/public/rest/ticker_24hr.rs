use crate::binance::spot::public::rest::RestClient;
use crate::binance::spot::RestResult;
use serde_json::Value;

impl RestClient {
    /// 24hr ticker price change statistics
    /// 
    /// Weight: 2 for single symbol, 80 for all symbols
    pub async fn ticker_24hr(&self, symbol: Option<&str>, symbols: Option<&[&str]>, ticker_type: Option<&str>) -> RestResult<Value> {
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