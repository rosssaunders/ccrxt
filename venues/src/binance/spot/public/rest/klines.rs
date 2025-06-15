use crate::binance::spot::public::rest::RestClient;
use crate::binance::spot::RestResult;
use serde_json::Value;

impl RestClient {
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
}