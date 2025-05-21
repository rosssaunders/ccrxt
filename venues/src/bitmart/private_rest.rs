use hmac::{Hmac, Mac};
use reqwest::Client;
use secrecy::{Secret, SecretString};
use sha2::Sha256;
use crate::common::encryption::{self, EncryptionError};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};

#[derive(Debug, thiserror::Error)]
pub enum BitMartPrivateRestError {
    #[error("Encryption error: {0}")]
    Encryption(#[from] EncryptionError),
    #[error("Failed to sign request: {0}")]
    SigningFailed(String),
}

/// A client for interacting with the BitMart private REST API
/// 
/// This client handles encrypted API keys and secrets for enhanced security.
/// The API key and secret are stored in encrypted form and only decrypted when needed.
pub struct BitMartPrivateRest {
    pub(crate) client: Client,
    pub(crate) encrypted_api_key: SecretString,
    pub(crate) encrypted_api_secret: SecretString,
    pub(crate) base_url: String,
    encryption_key: Secret<[u8; 32]>,
}

impl BitMartPrivateRest {
    /// Creates a new BitMartPrivateRest client with encrypted API credentials
    /// 
    /// # Arguments
    /// * `encrypted_api_key` - The encrypted API key
    /// * `encrypted_api_secret` - The encrypted API secret
    /// * `base_url` - The base URL for the API
    /// * `encryption_key` - The key used for decrypting the API credentials
    /// 
    /// # Returns
    /// A new BitMartPrivateRest client instance
    pub fn new(
        encrypted_api_key: impl Into<SecretString>,
        encrypted_api_secret: impl Into<SecretString>,
        base_url: String,
        encryption_key: [u8; 32],
    ) -> Self {
        Self {
            client: Client::new(),
            encrypted_api_key: encrypted_api_key.into(),
            encrypted_api_secret: encrypted_api_secret.into(),
            base_url,
            encryption_key: Secret::new(encryption_key),
        }
    }

    /// Gets the decrypted API key
    pub(crate) fn get_api_key(&self) -> Result<SecretString, BitMartPrivateRestError> {
        encryption::decrypt(self.encrypted_api_key.expose_secret(), self.encryption_key.expose_secret())
            .map(SecretString::new)
            .map_err(BitMartPrivateRestError::from)
    }

    /// Gets the decrypted API secret
    pub(crate) fn get_api_secret(&self) -> Result<SecretString, BitMartPrivateRestError> {
        encryption::decrypt(self.encrypted_api_secret.expose_secret(), self.encryption_key.expose_secret())
            .map(SecretString::new)
            .map_err(BitMartPrivateRestError::from)
    }

    /// Signs a request using the decrypted API secret according to BitMart's rules
    ///
    /// # Arguments
    /// * `timestamp` - Current timestamp in milliseconds
    /// * `request_path` - API endpoint path
    /// * `request_body` - Request body or query string (may be empty)
    ///
    /// # Returns
    /// Base64-encoded HMAC-SHA256 signature
    pub fn sign_request(
        &self,
        timestamp: &str, 
        request_path: &str, 
        request_body: &str
    ) -> Result<String, BitMartPrivateRestError> {
        let api_key = self.get_api_key()?;
        let api_secret = self.get_api_secret()?;
        
        // Construct message to sign: timestamp + API-Key + request_path + [query_string or request_body]
        let message = format!(
            "{}{}{}{}",
            timestamp,
            api_key.expose_secret(),
            request_path,
            request_body
        );
        
        // Create HMAC-SHA256 signature
        let mut mac = Hmac::<Sha256>::new_from_slice(api_secret.expose_secret().as_bytes())
            .map_err(|e| BitMartPrivateRestError::SigningFailed(e.to_string()))?;
            
        mac.update(message.as_bytes());
        
        // Base64 encode the signature (BitMart uses base64 instead of hex)
        let signature = BASE64.encode(mac.finalize().into_bytes());
        
        Ok(signature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::encryption;

    #[test]
    fn test_sign_request() {
        // Setup
        let test_key = [1u8; 32]; 
        let api_key = "test_api_key";
        let api_secret = "test_api_secret";
        
        // Encrypt API credentials for test
        let encrypted_api_key = encryption::encrypt(api_key, &test_key).unwrap();
        let encrypted_api_secret = encryption::encrypt(api_secret, &test_key).unwrap();
        
        let client = BitMartPrivateRest::new(
            encrypted_api_key,
            encrypted_api_secret,
            "https://api-cloud.bitmart.com".to_string(),
            test_key
        );
        
        // Test params
        let timestamp = "1747857098763";
        let request_path = "/spot/v1/ticker";
        let request_body = "symbol=BTC_USDT";
        
        // Call sign_request
        let signature = client.sign_request(timestamp, request_path, request_body).unwrap();
        
        // Verify signature is base64 encoded
        assert!(!signature.is_empty());
        
        // We can manually verify the signature is correctly formed
        let expected_message = format!("{}{}{}{}", timestamp, api_key, request_path, request_body);
        let mut expected_mac = Hmac::<Sha256>::new_from_slice(api_secret.as_bytes()).unwrap();
        expected_mac.update(expected_message.as_bytes());
        let expected_signature = BASE64.encode(expected_mac.finalize().into_bytes());
        
        // Compare with our implementation's result
        assert_eq!(signature, expected_signature);
    }
}