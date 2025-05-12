use reqwest::Client;
use super::{
    api_errors::BinanceCoinMResult,
    rate_limit::BinanceCoinMRateLimiter,
    types::BinanceResponse,
    common::request::send_request,
};
pub struct BinanceCoinMPublicRest {
    client: Client,
    rate_limiter: BinanceCoinMRateLimiter,
    base_url: String,
}

impl BinanceCoinMPublicRest {
    pub async fn get_ticker_24h(&self, symbol: Option<&str>) -> BinanceCoinMResult<BinanceResponse<serde_json::Value>> {
        let endpoint = match symbol {
            Some(s) => format!("/dapi/v1/ticker/24hr?symbol={}", s),
            None => "/dapi/v1/ticker/24hr".to_string(),
        };
        
        send_request(
            &self.client,
            &self.base_url,
            &endpoint,
            reqwest::Method::GET,
            None,
            None,
            || self.rate_limiter.check_weight_limit("ticker/24hr", 1)
        ).await
    }
} 