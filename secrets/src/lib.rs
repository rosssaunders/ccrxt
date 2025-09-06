//! Secrets crate: shared secret types and traits.

use secrecy::ExposeSecret;
// Re-export SecretString for convenience
pub use secrecy::SecretString;

/// A trait for types that can securely expose a secret value.
///
/// This trait provides a way to expose secrets while maintaining control over
/// how and when they are exposed. Implementors should ensure that the secret
/// is handled securely and cleared from memory when no longer needed.
pub trait ExposableSecret: Send + Sync {
    /// Exposes the secret value as a String.
    ///
    /// This method should be used with caution as it exposes the secret value.
    /// The secret should be cleared from memory as soon as possible after use.
    fn expose_secret(&self) -> String;
}

/// Implementation of ExposableSecret for SecretString from the secrecy crate
impl ExposableSecret for SecretString {
    fn expose_secret(&self) -> String {
        ExposeSecret::expose_secret(self).to_string()
    }
}

/// A simple implementation of ExposableSecret that wraps a SecretString.
///
/// This struct provides a basic implementation of ExposableSecret that can be used
/// when you have a SecretString that needs to be exposed through the ExposableSecret trait.
#[derive(Clone, Debug)]
pub struct SecretValue {
    /// The secret value, stored securely using SecretString
    secret: SecretString,
}

impl ExposableSecret for SecretValue {
    fn expose_secret(&self) -> String {
        ExposeSecret::expose_secret(&self.secret).to_string()
    }
}

impl SecretValue {
    /// Creates a new SecretValue with the given secret.
    ///
    /// # Arguments
    ///
    /// * `secret` - The secret value to store
    pub fn new(secret: SecretString) -> Self {
        Self { secret }
    }
}

// Test-only utilities for secrets
pub mod plain_text_secret;

// Re-export to preserve the original symbol path `secrets::PlainTextSecret`
pub use plain_text_secret::PlainTextSecret;
