use reqwest::Client;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use hex;
use super::rate_limit::BinanceCoinMRateLimiter;

pub struct BinanceCoinMPrivateRest {
    pub(crate) client: Client,
    pub(crate) rate_limiter: BinanceCoinMRateLimiter,
    pub(crate) api_key: String,
    pub(crate) api_secret: String,
    pub(crate) base_url: String,
}

impl BinanceCoinMPrivateRest {
    pub fn new(api_key: String, api_secret: String, base_url: String) -> Self {
        Self {
            client: Client::new(),
            rate_limiter: BinanceCoinMRateLimiter::new(),
            api_key,
            api_secret,
            base_url,
        }
    }

    pub fn sign_request(&self, query_string: &str) -> String {
        let mut mac = Hmac::<Sha256>::new_from_slice(self.api_secret.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(query_string.as_bytes());
        hex::encode(mac.finalize().into_bytes())
    }
} 