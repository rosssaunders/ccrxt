use std::borrow::Cow;
use std::collections::BTreeMap;

use chrono::Utc;
use hmac::{Hmac, Mac};
use rest::secrets::ExposableSecret;
use serde_json::{Value, json};
use sha2::Sha256;

use crate::cryptocom::Errors;

/// Signs a request using the Crypto.com signing algorithm
///
/// The algorithm follows the Crypto.com documentation:
/// 1. Sort request parameter keys in ascending order
/// 2. Combine all ordered parameter keys as key + value (no spaces, no delimiters)
/// 3. Create signature payload: method + id + api_key + parameter_string + nonce
/// 4. Use HMAC-SHA256 to hash using API Secret as cryptographic key
/// 5. Encode output as hex string
///
/// # Arguments
/// * `api_secret` - The API secret for HMAC signing
/// * `method` - The API method name
/// * `id` - The request ID
/// * `api_key` - The API key
/// * `params` - The request parameters as JSON Value
/// * `nonce` - The nonce value
///
/// # Returns
/// A result containing the signature as a hex string or an error if signing fails.
fn sign_request(api_secret: &dyn ExposableSecret, method: &str, id: u64, api_key: &str, params: &Value, nonce: u64) -> Result<String, Errors> {
    // Convert params to string using the Crypto.com algorithm
    let params_string = params_to_string(params);

    // Create the signature payload: method + id + api_key + params_string + nonce
    let sig_payload = format!("{method}{id}{api_key}{params_string}{nonce}");

    // Sign with HMAC-SHA256
    let api_secret = api_secret.expose_secret();
    let mut mac = Hmac::<Sha256>::new_from_slice(api_secret.as_bytes()).map_err(|_| Errors::InvalidApiKey())?;
    mac.update(sig_payload.as_bytes());

    Ok(hex::encode(mac.finalize().into_bytes()))
}

/// Converts a JSON Value to a string following Crypto.com's algorithm
///
/// This function implements the JavaScript algorithm from the documentation:
/// - For objects: sort keys alphabetically, concatenate key + value pairs
/// - For arrays: concatenate all elements
/// - For primitives: convert to string representation
fn params_to_string(value: &Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.clone(),
        Value::Array(arr) => arr
            .iter()
            .map(params_to_string)
            .collect::<Vec<_>>()
            .join(""),
        Value::Object(obj) => {
            // Sort keys alphabetically and concatenate key + value pairs
            let mut sorted_map = BTreeMap::new();
            for (k, v) in obj {
                sorted_map.insert(k, v);
            }

            sorted_map
                .iter()
                .map(|(k, v)| format!("{}{}", k, params_to_string(v)))
                .collect::<Vec<_>>()
                .join("")
        }
    }
}

/// A client for interacting with the Crypto.com private REST API
///
/// This client handles encrypted API keys and secrets for enhanced security.
/// The API key and secret are stored in encrypted form and only decrypted when needed.
pub struct RestClient {
    /// The underlying HTTP client used for making requests.
    pub(crate) client: reqwest::Client,
    /// The encrypted API key.
    pub(crate) api_key: Box<dyn ExposableSecret>,
    /// The encrypted API secret.
    pub(crate) api_secret: Box<dyn ExposableSecret>,
    /// The base URL for the API.
    pub(crate) base_url: Cow<'static, str>,
}

impl RestClient {
    /// Creates a new RestClient with encrypted API credentials
    ///
    /// # Arguments
    /// * `api_key` - The encrypted API key
    /// * `api_secret` - The encrypted API secret
    /// * `base_url` - The base URL for the API
    /// * `client` - The HTTP client to use
    ///
    /// # Returns
    /// A new RestClient instance
    pub fn new(
        api_key: Box<dyn ExposableSecret>,
        api_secret: Box<dyn ExposableSecret>,
        base_url: impl Into<Cow<'static, str>>,
        client: reqwest::Client,
    ) -> Self {
        Self {
            client,
            api_key,
            api_secret,
            base_url: base_url.into(),
        }
    }

    /// Signs a request for Crypto.com private endpoints
    ///
    /// # Arguments
    /// * `method` - The API method name
    /// * `id` - The request ID  
    /// * `params` - The request parameters as JSON Value
    /// * `nonce` - The nonce value
    ///
    /// # Returns
    /// A result containing the signature as a hex string or an error
    pub fn sign_request(&self, method: &str, id: u64, params: &Value, nonce: u64) -> Result<String, Errors> {
        let api_key = self.api_key.expose_secret();
        sign_request(
            self.api_secret.as_ref(),
            method,
            id,
            &api_key,
            params,
            nonce,
        )
    }

    /// Sends a signed request to the Crypto.com private REST API
    ///
    /// Builds, signs, and sends the request, returning the parsed response as the specified type.
    ///
    /// # Type Parameters
    /// * `Req` - The request type, must implement `Serialize`.
    /// * `Resp` - The response type, must implement `DeserializeOwned`.
    ///
    /// # Arguments
    /// * `method` - The API method name (e.g., "private/amend-order")
    /// * `request` - The request parameters as a struct
    ///
    /// # Returns
    /// Parsed response of type `Resp`.
    pub async fn send_signed_request<Req, Resp>(&self, method: &str, request: Req) -> crate::cryptocom::RestResult<Resp>
    where
        Req: serde::Serialize,
        Resp: serde::de::DeserializeOwned,
    {
        let params = serde_json::to_value(&request).map_err(|e| crate::cryptocom::Errors::Error(format!("Serialization error: {e}")))?;
        self.send_signed_request_int::<Resp>(method, params).await
    }

    /// Sends a signed request to the Crypto.com private REST API
    ///
    /// Builds, signs, and sends the request, returning the parsed JSON value.
    ///
    /// # Arguments
    /// * `method` - The API method name (e.g., "private/amend-order")
    /// * `params` - The request parameters as JSON Value
    ///
    /// # Returns
    /// Parsed JSON value of the response.
    pub async fn send_signed_request_int<T>(&self, method: &str, params: Value) -> crate::cryptocom::RestResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let nonce = Utc::now().timestamp_millis() as u64;
        let id = 1;
        let signature = self.sign_request(method, id, &params, nonce)?;

        let request_body = json!({
            "id": id,
            "method": method,
            "params": params,
            "nonce": nonce,
            "sig": signature,
            "api_key": self.api_key.expose_secret(),
        });

        let response = self
            .client
            .post(format!("{}/v1/{}", self.base_url, method))
            .json(&request_body)
            .send()
            .await?;

        let result = response.json().await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    /// A plain text implementation of ExposableSecret for testing purposes.
    ///
    /// **WARNING**: This implementation stores the secret in plain text and should
    /// only be used for testing. Never use this in production code.
    #[derive(Clone)]
    #[allow(dead_code)]
    struct PlainTextSecret {
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
        /// **WARNING**: This implementation should only be used for testing.
        ///
        /// # Arguments
        /// * `secret` - The secret value to store in plain text
        #[allow(dead_code)]
        fn new(secret: String) -> Self {
            Self { secret }
        }
    }

    #[test]
    fn test_params_to_string_null() {
        let value = Value::Null;
        assert_eq!(params_to_string(&value), "");
    }

    #[test]
    fn test_params_to_string_bool() {
        let value = json!(true);
        assert_eq!(params_to_string(&value), "true");

        let value = json!(false);
        assert_eq!(params_to_string(&value), "false");
    }

    #[test]
    fn test_params_to_string_number() {
        let value = json!(42);
        assert_eq!(params_to_string(&value), "42");

        let value = json!(3.15);
        assert_eq!(params_to_string(&value), "3.15");
    }

    #[test]
    fn test_params_to_string_string() {
        let value = json!("hello");
        assert_eq!(params_to_string(&value), "hello");
    }

    #[test]
    fn test_params_to_string_array() {
        let value = json!([1, 2, 3]);
        assert_eq!(params_to_string(&value), "123");

        let value = json!(["a", "b", "c"]);
        assert_eq!(params_to_string(&value), "abc");
    }

    #[test]
    fn test_params_to_string_object() {
        let value = json!({
            "b": 2,
            "a": 1,
            "c": 3
        });
        // Keys should be sorted alphabetically: a, b, c
        assert_eq!(params_to_string(&value), "a1b2c3");
    }

    #[test]
    fn test_params_to_string_nested_object() {
        let value = json!({
            "order_id": 53287421324_u64,
            "symbol": "BTC_USDT"
        });
        // Keys sorted: order_id, symbol
        assert_eq!(
            params_to_string(&value),
            "order_id53287421324symbolBTC_USDT"
        );
    }

    #[test]
    fn test_params_to_string_complex() {
        let value = json!({
            "array": [1, 2, 3],
            "nested": {
                "inner": "value"
            },
            "simple": "test"
        });
        // Keys sorted: array, nested, simple
        // array becomes "123", nested becomes "innervalue", simple becomes "test"
        assert_eq!(
            params_to_string(&value),
            "array123nestedinnervaluesimpletest"
        );
    }

    #[test]
    fn test_sign_request() {
        let _api_key = Box::new(PlainTextSecret::new("test_api_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret = Box::new(PlainTextSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;

        let method = "private/get-order-detail";
        let id = 11;
        let params = json!({
            "order_id": 53287421324_u64
        });
        let nonce = 1587846358253_u64;

        let signature = sign_request(
            api_secret.as_ref(),
            method,
            id,
            "test_api_key",
            &params,
            nonce,
        )
        .unwrap();

        // Verify the signature is a hex string of the expected length (64 chars for SHA256)
        assert_eq!(signature.len(), 64);
        assert!(signature.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_client_creation() {
        let api_key = Box::new(PlainTextSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret = Box::new(PlainTextSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
        let client = reqwest::Client::new();

        let rest_client = RestClient::new(api_key, api_secret, "https://api.crypto.com", client);

        assert_eq!(rest_client.base_url, "https://api.crypto.com");
    }

    #[test]
    fn test_client_sign_request() {
        let api_key = Box::new(PlainTextSecret::new("test_api_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret = Box::new(PlainTextSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
        let client = reqwest::Client::new();

        let rest_client = RestClient::new(api_key, api_secret, "https://api.crypto.com", client);

        let params = json!({
            "order_id": 53287421324_u64
        });

        let signature = rest_client
            .sign_request("private/get-order-detail", 11, &params, 1587846358253)
            .unwrap();

        assert_eq!(signature.len(), 64);
        assert!(signature.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_crypto_com_example_signing() {
        // Test the exact example from the Crypto.com documentation
        let api_key = "test_api_key";
        let api_secret = Box::new(PlainTextSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;

        let method = "private/get-order-detail";
        let id = 11;
        let params = json!({
            "order_id": 53287421324_u64
        });
        let nonce = 1587846358253_u64;

        let signature = sign_request(api_secret.as_ref(), method, id, api_key, &params, nonce).unwrap();

        // Verify the signature is a valid hex string
        assert_eq!(signature.len(), 64);
        assert!(signature.chars().all(|c| c.is_ascii_hexdigit()));

        // Verify the signature payload construction
        let params_string = params_to_string(&params);
        assert_eq!(params_string, "order_id53287421324");

        let expected_payload = format!("{}{}{}{}{}", method, id, api_key, params_string, nonce);
        assert_eq!(
            expected_payload,
            "private/get-order-detail11test_api_keyorder_id532874213241587846358253"
        );
    }

    #[test]
    fn test_empty_params_signing() {
        let api_secret = Box::new(PlainTextSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;

        let signature = sign_request(
            api_secret.as_ref(),
            "private/get-account-summary",
            1,
            "test_key",
            &json!({}),
            1234567890,
        )
        .unwrap();

        assert_eq!(signature.len(), 64);
        assert!(signature.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_complex_params_signing() {
        let api_secret = Box::new(PlainTextSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;

        let params = json!({
            "instrument_name": "BTC_USDT",
            "side": "BUY",
            "type": "LIMIT",
            "quantity": "1.5",
            "price": "50000.00",
            "metadata": {
                "client_id": "client123",
                "tags": ["trading", "btc"]
            }
        });

        let signature = sign_request(
            api_secret.as_ref(),
            "private/create-order",
            42,
            "api_key_123",
            &params,
            1640995200000,
        )
        .unwrap();

        // Verify signature format
        assert_eq!(signature.len(), 64);
        assert!(signature.chars().all(|c| c.is_ascii_hexdigit()));

        // Verify params string construction with sorted keys
        let params_string = params_to_string(&params);
        // Keys should be sorted: instrument_name, metadata, price, quantity, side, type
        let expected_start = "instrument_nameBTC_USDTmetadata";
        assert!(params_string.starts_with(expected_start));
    }
}
