//! Test-only plain text secret implementation.
//! WARNING: This stores secrets in plain text and exists solely for tests.

use super::ExposableSecret;

#[derive(Clone, Debug)]
pub struct PlainTextSecret {
    secret: String,
}

impl ExposableSecret for PlainTextSecret {
    fn expose_secret(&self) -> String {
        self.secret.clone()
    }
}

impl PlainTextSecret {
    /// Creates a new PlainTextSecret with the given secret.
    ///
    /// # Arguments
    ///
    /// * `secret` - The secret value to store in plain text (for tests only)
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}
