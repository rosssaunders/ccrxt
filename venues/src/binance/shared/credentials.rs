//! Binance API credentials - shared across all Binance products

use secrets::SecretString;

/// Credentials for authenticating with Binance private REST API.
///
/// These credentials are shared across all Binance products (Spot, USDM, COINM, Options).
/// All fields are securely stored using SecretString.
#[derive(Debug, Clone)]
pub struct Credentials {
    /// API key (required)
    pub api_key: SecretString,

    /// API secret (required)
    pub api_secret: SecretString,
}
