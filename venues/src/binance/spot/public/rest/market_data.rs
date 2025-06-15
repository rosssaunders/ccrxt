use crate::binance::spot::public::rest::RestClient;
use crate::binance::spot::RestResult;
use serde_json::Value;

impl RestClient {
    /// Order book depth
    /// 
    /// Valid limits: [5, 10, 20, 50, 100, 500, 1000, 5000]
    /// Weight: Based on limit (5-100: 5, 101-500: 25, 501-1000: 50, 1001-5000: 250)
    pub async fn depth(&self, symbol: &str, limit: Option<u16>) -> RestResult<Value> {
        let mut query_params = vec![("symbol", symbol)];
        let limit_str;
        let weight = if let Some(l) = limit {
            limit_str = l.to_string();
            query_params.push(("limit", &limit_str));
            match l {
                1..=100 => 5,
                101..=500 => 25,
                501..=1000 => 50,
                1001..=5000 => 250,
                _ => 5, // Default to 5 for invalid limits
            }
        } else {
            5 // Default weight for no limit (100 default)
        };
        
        let query_string = serde_urlencoded::to_string(&query_params)
            .map_err(|e| crate::binance::spot::Errors::Error(format!("URL encoding error: {}", e)))?;
        
        self.send_request("/api/v3/depth", reqwest::Method::GET, Some(&query_string), None, weight)
            .await
    }

    /// Recent trades list
    /// 
    /// Get recent trades.
    /// Weight: 25
    pub async fn trades(&self, symbol: &str, limit: Option<u16>) -> RestResult<Value> {
        let mut query_params = vec![("symbol", symbol)];
        let limit_str;
        if let Some(l) = limit {
            limit_str = l.to_string();
            query_params.push(("limit", &limit_str));
        }
        
        let query_string = serde_urlencoded::to_string(&query_params)
            .map_err(|e| crate::binance::spot::Errors::Error(format!("URL encoding error: {}", e)))?;
        
        self.send_request("/api/v3/trades", reqwest::Method::GET, Some(&query_string), None, 25)
            .await
    }

    /// Kline/Candlestick data
    /// 
    /// Kline/candlestick bars for a symbol.
    /// Weight: 2
    pub async fn klines(
        &self, 
        symbol: &str, 
        interval: &str, 
        start_time: Option<u64>,
        end_time: Option<u64>,
        time_zone: Option<&str>,
        limit: Option<u16>
    ) -> RestResult<Value> {
        let mut query_params = vec![("symbol", symbol), ("interval", interval)];
        let start_time_str;
        let end_time_str;
        let limit_str;
        
        if let Some(st) = start_time {
            start_time_str = st.to_string();
            query_params.push(("startTime", &start_time_str));
        }
        if let Some(et) = end_time {
            end_time_str = et.to_string();
            query_params.push(("endTime", &end_time_str));
        }
        if let Some(tz) = time_zone {
            query_params.push(("timeZone", tz));
        }
        if let Some(l) = limit {
            limit_str = l.to_string();
            query_params.push(("limit", &limit_str));
        }
        
        let query_string = serde_urlencoded::to_string(&query_params)
            .map_err(|e| crate::binance::spot::Errors::Error(format!("URL encoding error: {}", e)))?;
        
        self.send_request("/api/v3/klines", reqwest::Method::GET, Some(&query_string), None, 2)
            .await
    }

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