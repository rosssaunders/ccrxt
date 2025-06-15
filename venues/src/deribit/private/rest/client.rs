use rest::secrets::ExposableSecret;
use reqwest::Client;
use serde_json::Value;
use std::borrow::Cow;

use crate::deribit::{RateLimiter, Errors};

/// REST client for Deribit private endpoints
pub struct RestClient {
    /// HTTP client for making requests
    pub client: Client,
    /// API key for authentication
    pub api_key: Box<dyn ExposableSecret>,
    /// API secret for signing requests
    pub api_secret: Box<dyn ExposableSecret>,
    /// Base URL for the API
    pub base_url: Cow<'static, str>,
    /// Rate limiter to respect API limits
    pub rate_limiter: RateLimiter,
}

impl std::fmt::Debug for RestClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RestClient")
            .field("client", &self.client)
            .field("api_key", &"[REDACTED]")
            .field("api_secret", &"[REDACTED]")
            .field("base_url", &self.base_url)
            .field("rate_limiter", &self.rate_limiter)
            .finish()
    }
}

impl RestClient {
    /// Creates a new RestClient with encrypted API credentials
    ///
    /// # Arguments
    /// * `api_key` - The encrypted API key
    /// * `api_secret` - The encrypted API secret
    /// * `base_url` - The base URL for the API (e.g., "https://www.deribit.com/api/v2")
    /// * `client` - The HTTP client to use
    /// * `rate_limiter` - Rate limiter for respecting API limits
    ///
    /// # Returns
    /// A new RestClient instance
    pub fn new(
        api_key: Box<dyn ExposableSecret>,
        api_secret: Box<dyn ExposableSecret>,
        base_url: impl Into<Cow<'static, str>>,
        client: Client,
        rate_limiter: RateLimiter,
    ) -> Self {
        Self {
            client,
            api_key,
            api_secret,
            base_url: base_url.into(),
            rate_limiter,
        }
    }

    /// Creates authentication headers for Deribit API requests
    ///
    /// For Deribit, we'll use basic API key authentication
    /// This is a simplified implementation - in a real scenario, you might need
    /// more sophisticated authentication like OAuth or HMAC signing
    ///
    /// # Arguments
    /// * `method` - The API method name
    /// * `params` - The request parameters as JSON Value
    ///
    /// # Returns
    /// A result containing the authentication token or an error
    pub fn create_auth_token(
        &self,
        _method: &str,
        _params: &Value,
    ) -> Result<String, Errors> {
        // For this implementation, we'll use the API key directly
        // In a real implementation, this might involve creating a JWT or similar
        Ok(self.api_key.expose_secret())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deribit::AccountTier;
    use serde_json::json;

    /// A plain text implementation of ExposableSecret for testing purposes.
    ///
    /// **WARNING**: This implementation stores the secret in plain text and should
    /// only be used for testing. Never use this in production code.
    #[derive(Clone)]
    struct PlainTextSecret {
        secret: String,
    }

    impl ExposableSecret for PlainTextSecret {
        fn expose_secret(&self) -> String {
            self.secret.clone()
        }
    }

    impl PlainTextSecret {
        /// Creates a new PlainTextSecret with the given secret.
        ///
        /// **WARNING**: This implementation should only be used for testing.
        fn new(secret: String) -> Self {
            Self { secret }
        }
    }

    #[test]
    fn test_rest_client_creation() {
        let api_key = Box::new(PlainTextSecret::new("test_key".to_string()));
        let api_secret = Box::new(PlainTextSecret::new("test_secret".to_string()));
        let client = Client::new();
        let rate_limiter = RateLimiter::new(AccountTier::Tier3);

        let rest_client = RestClient::new(
            api_key,
            api_secret,
            "https://www.deribit.com/api/v2",
            client,
            rate_limiter,
        );

        assert_eq!(rest_client.base_url, "https://www.deribit.com/api/v2");
    }

    #[test]
    fn test_auth_token_creation() {
        let api_key = Box::new(PlainTextSecret::new("test_key".to_string()));
        let api_secret = Box::new(PlainTextSecret::new("test_secret".to_string()));
        let client = Client::new();
        let rate_limiter = RateLimiter::new(AccountTier::Tier3);

        let rest_client = RestClient::new(
            api_key,
            api_secret,
            "https://www.deribit.com/api/v2",
            client,
            rate_limiter,
        );

        let params = json!({
            "currency": "BTC",
            "amount": 100,
            "destination": 12345
        });

        let result = rest_client.create_auth_token("private/submit_transfer_to_subaccount", &params);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_key");
    }
}