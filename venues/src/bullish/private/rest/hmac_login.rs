//! HMAC Login endpoint for Bullish

use base64::{Engine as _, engine::general_purpose};
use hmac::{Hmac, Mac};
use secrecy::ExposeSecret;
use serde::Deserialize;
use serde_json::Value;
use sha2::Sha256;

use crate::bullish::{EndpointType, Errors, RestResult, private::rest::RestClient};

/// Endpoint constant for HMAC login
const HMAC_LOGIN_ENDPOINT: &str = "/v1/users/hmac/login";

/// Response for HMAC login
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HmacLoginResponse {
    /// JWT authorizer for signing requests
    pub authorizer: String,

    /// JWT token
    pub token: String,
}

impl RestClient {
    /// HMAC Login
    ///
    /// Login and generate a new session associated with a JWT using HMAC. Once you log
    /// in from an IP, the same IP must be used for the duration of the session for any
    /// subsequent requests.
    ///
    /// [docs]: https://api.exchange.bullish.com/trading-api/v1/users/hmac/login
    ///
    /// Rate limit: True
    ///
    /// # Returns
    /// JWT authorizer and token for authenticated requests
    pub async fn hmac_login(&mut self) -> RestResult<HmacLoginResponse> {
        // Check rate limits
        self.rate_limiter
            .check_limits(EndpointType::PrivateLogin)
            .await
            .map_err(|e| Errors::RateLimitError(e.to_string()))?;

        let nonce = chrono::Utc::now().timestamp();
        let message = format!("GET/trading-api{}{}", HMAC_LOGIN_ENDPOINT, nonce);

        // Sign the message with HMAC-SHA256
        let mut mac =
            Hmac::<Sha256>::new_from_slice(self.credentials.api_secret.expose_secret().as_bytes())
                .map_err(|_| Errors::InvalidApiKey())?;
        mac.update(message.as_bytes());
        let signature = general_purpose::STANDARD.encode(mac.finalize().into_bytes());

        let url = format!("{}/trading-api{}", self.base_url, HMAC_LOGIN_ENDPOINT);

        let response = self
            .client
            .get(&url)
            .header("BX-KEY", self.credentials.api_key.expose_secret())
            .header("BX-SIGNATURE", signature)
            .header("BX-NONCE", nonce.to_string())
            .send()
            .await?;

        self.rate_limiter
            .increment_request(EndpointType::PrivateLogin)
            .await;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(Errors::AuthenticationError(format!(
                "HMAC login failed: {error_text}"
            )));
        }

        let result: Value = response.json().await?;

        // Map to strongly typed response
        let authorizer = result
            .get("authorizer")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let token = result
            .get("token")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        match (authorizer, token) {
            (Some(authorizer), Some(token)) => {
                self.jwt_token = Some(token.clone());
                Ok(HmacLoginResponse { authorizer, token })
            }
            _ => Err(Errors::AuthenticationError(
                "No authorizer or token in HMAC login response".to_string(),
            )),
        }
    }
}
