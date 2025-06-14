// Basic placeholder client for USDM private endpoints
use reqwest::Client;
use std::borrow::Cow;

use crate::binance::usdm::RateLimiter;

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct RestClient {
    pub base_url: Cow<'static, str>,
    pub client: Client,
    pub rate_limiter: RateLimiter,
}

impl RestClient {
    /// Creates a new RestClient for USDM private endpoints
    pub fn new(base_url: impl Into<Cow<'static, str>>, client: Client) -> Self {
        Self {
            base_url: base_url.into(),
            client,
            rate_limiter: RateLimiter::new(),
        }
    }
}
