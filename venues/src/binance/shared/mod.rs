//! Shared Binance REST client logic for all Binance venues.
//!
//! This module provides the canonical implementation of send_signed_request for all Binance venues.

use hex;
use hmac::{Hmac, Mac};
use serde::Serialize;
use sha2::Sha256;

use rest::secrets::ExposableSecret;

/// Signs a query string using the decrypted API secret and returns the signature as a hex string.
pub fn sign_request(api_secret: &dyn ExposableSecret, query_string: &str) -> Result<String, String> {
    let api_secret = api_secret.expose_secret();
    let mut mac = Hmac::<Sha256>::new_from_slice(api_secret.as_bytes()).map_err(|_| "Invalid API key/secret".to_string())?;
    mac.update(query_string.as_bytes());
    Ok(hex::encode(mac.finalize().into_bytes()))
}

/// Canonical send_signed_request for all Binance venues.
///
/// - Serializes the request struct.
/// - Signs the request.
/// - Calls the venue's send_request implementation.
///
/// # Arguments
/// * `client` - The RestClient instance
/// * `endpoint` - The API endpoint path
/// * `method` - The HTTP method
/// * `request` - The request struct (will be serialized)
/// * `weight` - The request weight
/// * `is_order` - Whether this is an order endpoint
///
/// # Returns
/// The parsed response or error.
pub async fn send_signed_request<T, R, C>(client: &C, endpoint: &str, method: reqwest::Method, request: R, weight: u32, is_order: bool) -> Result<T, C::Error>
where
    T: serde::de::DeserializeOwned + Send + 'static,
    R: Serialize,
    C: BinanceRestClient,
{
    let serialized = serde_urlencoded::to_string(&request).map_err(|e| C::from_serialize(e))?;
    let signature = sign_request(client.api_secret(), &serialized).map_err(|e| C::from_signature(e))?;
    let signed = format!("{serialized}&signature={signature}");
    if method == reqwest::Method::GET {
        client
            .send_request(endpoint, method, Some(&signed), None, weight, is_order)
            .await
    } else {
        client
            .send_request(endpoint, method, None, Some(&signed), weight, is_order)
            .await
    }
}

/// Trait to be implemented by all Binance RestClient types for shared logic.
pub trait BinanceRestClient {
    type Error;
    
    fn api_secret(&self) -> &dyn ExposableSecret;
    
    fn send_request<T>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        query_string: Option<&str>,
        body: Option<&str>,
        weight: u32,
        is_order: bool,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, Self::Error>> + Send + '_>>
    where
        T: serde::de::DeserializeOwned + Send + 'static;
        
    fn from_serialize(e: serde_urlencoded::ser::Error) -> Self::Error;
    fn from_signature(e: String) -> Self::Error;
}
