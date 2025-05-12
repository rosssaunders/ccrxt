use reqwest::Client;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use hex;
use secrecy::{Secret, SecretString};
use super::rate_limit::BinanceCoinMRateLimiter;
use crate::common::encryption::{self, EncryptionError};

#[derive(Debug, thiserror::Error)]
pub enum BinanceCoinMPrivateRestError {
    #[error("Encryption error: {0}")]
    Encryption(#[from] EncryptionError),
    #[error("Failed to sign request: {0}")]
    SigningFailed(String),
}

/// A client for interacting with the Binance Coin-M Futures private REST API
/// 
/// This client handles encrypted API keys and secrets for enhanced security.
/// The API key and secret are stored in encrypted form and only decrypted when needed.
pub struct BinanceCoinMPrivateRest {
    pub(crate) client: Client,
    pub(crate) rate_limiter: BinanceCoinMRateLimiter,
    pub(crate) encrypted_api_key: SecretString,
    pub(crate) encrypted_api_secret: SecretString,
    pub(crate) base_url: String,
    encryption_key: Secret<[u8; 32]>,
}

impl BinanceCoinMPrivateRest {
    /// Creates a new BinanceCoinMPrivateRest client with encrypted API credentials
    /// 
    /// # Arguments
    /// * `encrypted_api_key` - The encrypted API key
    /// * `encrypted_api_secret` - The encrypted API secret
    /// * `base_url` - The base URL for the API
    /// * `encryption_key` - The key used for decrypting the API credentials
    /// 
    /// # Returns
    /// A new BinanceCoinMPrivateRest client instance
    pub fn new(
        encrypted_api_key: impl Into<SecretString>,
        encrypted_api_secret: impl Into<SecretString>,
        base_url: String,
        encryption_key: [u8; 32],
    ) -> Self {
        Self {
            client: Client::new(),
            rate_limiter: BinanceCoinMRateLimiter::new(),
            encrypted_api_key: encrypted_api_key.into(),
            encrypted_api_secret: encrypted_api_secret.into(),
            base_url,
            encryption_key: Secret::new(encryption_key),
        }
    }

    /// Gets the decrypted API key
    pub(crate) fn get_api_key(&self) -> Result<SecretString, BinanceCoinMPrivateRestError> {
        encryption::decrypt(self.encrypted_api_key.expose_secret(), self.encryption_key.expose_secret())
            .map(SecretString::new)
            .map_err(BinanceCoinMPrivateRestError::from)
    }

    /// Gets the decrypted API secret
    pub(crate) fn get_api_secret(&self) -> Result<SecretString, BinanceCoinMPrivateRestError> {
        encryption::decrypt(self.encrypted_api_secret.expose_secret(), self.encryption_key.expose_secret())
            .map(SecretString::new)
            .map_err(BinanceCoinMPrivateRestError::from)
    }

    /// Signs a request using the decrypted API secret
    pub fn sign_request(&self, query_string: &str) -> Result<String, BinanceCoinMPrivateRestError> {
        let api_secret = self.get_api_secret()?;
        let mut mac = Hmac::<Sha256>::new_from_slice(api_secret.expose_secret().as_bytes())
            .map_err(|e| BinanceCoinMPrivateRestError::SigningFailed(e.to_string()))?;
        mac.update(query_string.as_bytes());
        Ok(hex::encode(mac.finalize().into_bytes()))
    }
} 