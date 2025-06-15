use crate::binance::spot::public::rest::RestClient;
use crate::binance::spot::RestResult;
use serde_json::Value;

impl RestClient {
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
}