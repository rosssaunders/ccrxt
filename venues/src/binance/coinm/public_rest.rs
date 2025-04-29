use reqwest::Client;
use super::rate_limit::BinanceCoinMRateLimiter;

pub struct BinanceCoinMPublicRest {
    pub(crate) client: Client,
    pub(crate) rate_limiter: BinanceCoinMRateLimiter,
    pub(crate) base_url: String,
}

impl BinanceCoinMPublicRest {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            rate_limiter: BinanceCoinMRateLimiter::new(),
            base_url,
        }
    }
} 