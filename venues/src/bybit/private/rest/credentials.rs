//! ByBit API credentials

use secrets::SecretString;

/// Credentials for authenticating with ByBit private REST API.
///
/// All fields are securely stored using SecretString.
#[derive(Debug, Clone)]
pub struct Credentials {
    /// API key (required)
    pub api_key: SecretString,

    /// API secret (required)
    pub api_secret: SecretString,
}
