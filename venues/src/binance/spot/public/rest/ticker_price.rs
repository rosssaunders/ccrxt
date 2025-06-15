use crate::binance::spot::public::rest::RestClient;
use crate::binance::spot::RestResult;
use serde_json::Value;

impl RestClient {
    /// Symbol price ticker
    /// 
    /// Latest price for a symbol or symbols.
    /// Weight: 2 for single symbol, 4 for all symbols
    pub async fn ticker_price(&self, symbol: Option<&str>, symbols: Option<&[&str]>) -> RestResult<Value> {
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