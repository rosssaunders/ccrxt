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
