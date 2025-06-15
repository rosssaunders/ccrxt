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
}