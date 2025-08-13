//! Coinbase Exchange API credentials

use rest::secrets::SecretString;

/// Credentials for authenticating with Coinbase Exchange private REST API.
///
/// All fields are securely stored using SecretString.
#[derive(Debug, Clone)]
pub struct Credentials {
    /// API key (required)
    pub api_key: SecretString,

    /// API secret (required, base64 encoded)
    pub api_secret: SecretString,

    /// API passphrase (required for Coinbase Exchange)
    pub api_passphrase: SecretString,
}