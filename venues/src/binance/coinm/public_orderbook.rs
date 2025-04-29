use super::{
    types::{OrderBookSnapshot, BinanceResponse},
    errors::BinanceCoinMResult,
    public_rest::BinanceCoinMPublicRest,
    common::request::send_request,
};

impl BinanceCoinMPublicRest {
    pub async fn get_orderbook_snapshot(&self, symbol: &str, limit: Option<u32>) -> BinanceCoinMResult<BinanceResponse<OrderBookSnapshot>> {
        let limit = limit.unwrap_or(1000);
        let endpoint = format!("/dapi/v1/depth?symbol={}&limit={}", symbol, limit);
        
        send_request(
            &self.client,
            &self.base_url,
            &endpoint,
            reqwest::Method::GET,
            None,
            None,
            || self.rate_limiter.check_weight_limit("depth", 1)
        ).await
    }
} 