use crate::binance::spot::public::rest::RestClient;
use crate::binance::spot::RestResult;
use serde::Deserialize;

/// Represents a single kline/candlestick.
/// 
/// The Binance API returns klines as arrays with the following structure:
/// [open_time, open, high, low, close, volume, close_time, quote_asset_volume, number_of_trades, taker_buy_base_asset_volume, taker_buy_quote_asset_volume, unused_field]
///
/// See: <https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#klinecandlestick-data>
#[derive(Debug, Deserialize)]
pub struct KlineResponse(
    pub i64,    // Open time
    pub String, // Open price
    pub String, // High price
    pub String, // Low price
    pub String, // Close price
    pub String, // Volume
    pub i64,    // Close time
    pub String, // Quote asset volume
    pub i64,    // Number of trades
    pub String, // Taker buy base asset volume
    pub String, // Taker buy quote asset volume
    pub String, // Unused field, ignore
);

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
    ) -> RestResult<Vec<KlineResponse>> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binance::spot::RateLimiter;

    #[tokio::test]
    async fn test_klines_method_exists() {
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new();
        let rest_client = RestClient::new("https://api.binance.com", client, rate_limiter);

        // Test that the klines method is accessible
        // We're not calling it to avoid network requests in tests
        let _ = &rest_client.klines("BTCUSDT", "1h", None, None, None, None);
    }
}