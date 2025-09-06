//! KuCoin API credentials

use secrets::SecretString;

/// Credentials for authenticating with KuCoin private REST API.
///
/// All fields are securely stored and expected as SecretString.
#[derive(Debug, Clone)]
pub struct Credentials {
    /// API key (required)
    pub api_key: SecretString,

    /// API secret (required)
    pub api_secret: SecretString,

    /// API passphrase (required)
    pub api_passphrase: SecretString,
}
