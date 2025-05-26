//! Module for handling secure storage and retrieval of API credentials.
//! 
//! This module provides types and traits for securely storing and retrieving
//! API credentials like keys and secrets. It uses the `secrecy` crate to ensure
//! credentials are handled securely and not accidentally exposed.

use secrecy::{ExposeSecret, SecretString};

/// A trait for types that can securely expose a secret value.
/// 
/// This trait provides a way to expose secrets while maintaining control over
/// how and when they are exposed. Implementors should ensure that the secret
/// is handled securely and not accidentally exposed.
pub trait ExposableSecret: Send + Sync {
    /// Exposes the secret value as a String.
    /// 
    /// # Security Note
    /// This method should be used with caution as it exposes the secret value.
    /// The secret should be cleared from memory as soon as possible after use.
    fn expose_secret(&self) -> String;
}

/// A simple implementation of ExposableSecret that wraps a SecretString.
/// 
/// This struct provides a basic implementation of ExposableSecret that can be used
/// when you have a SecretString that needs to be exposed through the ExposableSecret trait.
#[derive(Clone)]
pub struct SecretValue {
    /// The secret value, stored securely using SecretString
    secret: SecretString,
}

impl ExposableSecret for SecretValue {
    fn expose_secret(&self) -> String {
        self.secret.expose_secret().to_string()
    }
}

impl SecretValue {
    /// Creates a new SecretValue with the given secret.
    /// 
    /// # Arguments
    /// * `secret` - The secret value to store
    pub fn new(secret: SecretString) -> Self {
        Self { secret }
    }
} 