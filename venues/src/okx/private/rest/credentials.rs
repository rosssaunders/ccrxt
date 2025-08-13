//! OKX API credentials

use rest::secrets::SecretString;

/// Credentials for authenticating with OKX private REST API.
///
/// All fields are securely stored using SecretString.
#[derive(Debug, Clone)]
pub struct Credentials {
    /// API key (required)
    pub api_key: SecretString,

    /// API secret (required)
    pub api_secret: SecretString,

    /// API passphrase (required for OKX)
    pub api_passphrase: SecretString,
}