use crate::binance::spot::public::rest::RestClient;
use crate::binance::spot::RestResult;
use serde::Deserialize;

/// Represents a single trade from the recent trades list.
///
/// See: <https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#recent-trades-list>
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeResponse {
    /// Trade ID
    pub id: i64,
    /// Price of the trade
    pub price: String,
    /// Quantity of the trade
    #[serde(rename = "qty")]
    pub quantity: String,
    /// Quote asset quantity
    pub quote_qty: String,
    /// Trade timestamp
    pub time: i64,
    /// Whether the buyer was the maker
    pub is_buyer_maker: bool,
    /// Whether the trade was the best price match
    pub is_best_match: bool,
}

impl RestClient {
    /// Recent trades list
    /// 
    /// Get recent trades.
    /// Weight: 25
    pub async fn trades(&self, symbol: &str, limit: Option<u16>) -> RestResult<Vec<TradeResponse>> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binance::spot::RateLimiter;

    #[tokio::test]
    async fn test_trades_method_exists() {
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new();
        let rest_client = RestClient::new("https://api.binance.com", client, rate_limiter);

        // Test that the trades method is accessible
        // We're not calling it to avoid network requests in tests
        let _ = &rest_client.trades("BTCUSDT", Some(5));
    }
}