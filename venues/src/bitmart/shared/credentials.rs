//! BitMart shared credentials module
//!
//! This module provides a unified credentials structure for both Spot and Contract products.

use rest::secrets::SecretString;

/// BitMart API credentials
///
/// Provides encrypted storage for API key and secret used across BitMart products.
/// Uses SecretString for secure credential storage.
#[derive(Debug, Clone)]
pub struct Credentials {
    /// The encrypted API key
    pub api_key: SecretString,

    /// The encrypted API secret for signing requests
    pub api_secret: SecretString,
}

impl Credentials {
    /// Creates new credentials with encrypted API key and secret
    ///
    /// # Arguments
    /// * `api_key` - The encrypted API key
    /// * `api_secret` - The encrypted API secret
    ///
    /// # Returns
    /// A new Credentials instance
    pub fn new(api_key: impl Into<SecretString>, api_secret: impl Into<SecretString>) -> Self {
        Self {
            api_key: api_key.into(),
            api_secret: api_secret.into(),
        }
    }
}
