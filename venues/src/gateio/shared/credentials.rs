//! GateIO API credentials - shared across all GateIO products

use rest::secrets::SecretString;

/// Credentials for authenticating with GateIO private REST API.
///
/// These credentials are shared across all GateIO products (Spot, Perpetual, Delivery, Options, Unified).
/// All fields are securely stored using SecretString.
#[derive(Debug, Clone)]
pub struct Credentials {
    /// API key (required)
    pub api_key: SecretString,

    /// API secret (required)
    pub api_secret: SecretString,
}