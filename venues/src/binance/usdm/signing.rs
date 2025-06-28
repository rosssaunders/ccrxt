//! Helper for Binance API request signing (HMAC SHA256)

use hmac::{Hmac, Mac};
use secrecy::{ExposeSecret, SecretString};
use sha2::Sha256;

/// Signs a query string using HMAC SHA256 and the given secret.
/// Returns the hex-encoded signature.
pub fn sign_query(query: &str, secret: &SecretString) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.expose_secret().as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(query.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}
