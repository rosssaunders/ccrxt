use crate::binance::spot::public::rest::RestClient;
use crate::binance::spot::RestResult;
use serde::Deserialize;

/// Response from the order book depth endpoint.
///
/// See: <https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#order-book>
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepthResponse {
    /// Last update ID
    pub last_update_id: i64,
    /// Bids (buyers) - price levels sorted from highest to lowest
    /// Each level is [price, quantity] as strings
    pub bids: Vec<(String, String)>,
    /// Asks (sellers) - price levels sorted from lowest to highest
    /// Each level is [price, quantity] as strings
    pub asks: Vec<(String, String)>,
}

impl RestClient {
    /// Order book depth
    /// 
    /// Valid limits: [5, 10, 20, 50, 100, 500, 1000, 5000]
    /// Weight: Based on limit (5-100: 5, 101-500: 25, 501-1000: 50, 1001-5000: 250)
    pub async fn depth(&self, symbol: &str, limit: Option<u16>) -> RestResult<DepthResponse> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binance::spot::RateLimiter;

    #[tokio::test]
    async fn test_depth_method_exists() {
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new();
        let rest_client = RestClient::new("https://api.binance.com", client, rate_limiter);

        // Test that the depth method is accessible
        // We're not calling it to avoid network requests in tests
        let _ = &rest_client.depth("BTCUSDT", Some(5));
    }
}