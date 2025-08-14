//! Crypto.com API credentials

use rest::secrets::SecretString;

/// Credentials for authenticating with Crypto.com private REST API.
///
/// All fields are securely stored using SecretString.
#[derive(Debug, Clone)]
pub struct Credentials {
    /// API key (required)
    pub api_key: SecretString,

    /// API secret (required)
    pub api_secret: SecretString,
}
