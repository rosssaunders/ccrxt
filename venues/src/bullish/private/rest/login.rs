use rest::{Method as HttpMethod, RequestBuilder};
use serde::{Deserialize, Serialize};

use crate::bullish::{EndpointType, Errors, PrivateRestClient as RestClient, RestResult};

/// Endpoint constant for login
const LOGIN_ENDPOINT: &str = "/v2/users/login";

/// Request parameters for login
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    /// Timestamp in seconds since epoch
    pub timestamp: i64,

    /// Nonce value
    pub nonce: i64,

    /// Authorizer string
    pub authorizer: String,

    /// Command string
    pub command: String,

    /// Optional public key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,

    /// Optional signature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,

    /// Optional login payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_payload: Option<LoginPayload>,
}

/// Login payload struct
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoginPayload {
    /// Bullish user ID
    pub user_id: String,

    /// Nonce (seconds since epoch)
    pub nonce: i64,

    /// Expiration time (seconds since epoch)
    pub expiration_time: i64,

    /// Biometrics flag (always false)
    pub biometrics_used: bool,

    /// Session key (always null)
    pub session_key: Option<String>,
}

/// Response for login
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    /// JWT authorizer for signing requests
    pub authorizer: String,

    /// JWT token
    pub token: String,
}

impl RestClient {
    /// Login
    ///
    /// Login and generate a new session associated with a JWT. Once you log in from an
    /// IP, the same IP must be used for the duration of the session for any subsequent
    /// requests.
    ///
    /// [docs](https://api.exchange.bullish.com/trading-api/v2/users/login)
    ///
    /// Rate limit: True
    ///
    /// # Arguments
    /// * `request` - The login request parameters
    ///
    /// # Returns
    /// JWT authorizer and token for authenticated requests
    pub async fn login(&mut self, request: LoginRequest) -> RestResult<LoginResponse> {
        // Check rate limits
        self.rate_limiter
            .check_limits(EndpointType::PrivateLogin)
            .await
            .map_err(|e| Errors::RateLimitError(e.to_string()))?;

        let url = format!("{}/trading-api{}", self.base_url, LOGIN_ENDPOINT);

        let body_vec = serde_json::to_vec(&request)?;
        let request = RequestBuilder::new(HttpMethod::Post, url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("Content-Length", body_vec.len().to_string())
            .body(body_vec)
            .build();
        let response = self
            .http_client
            .execute(request)
            .await
            .map_err(Errors::from)?;

        self.rate_limiter
            .increment_request(EndpointType::PrivateLogin)
            .await;

        if !response.is_success() {
            let error_text = response.text().unwrap_or_default();
            return Err(Errors::AuthenticationError(format!(
                "Login failed: {error_text}"
            )));
        }
        let login_response: LoginResponse = serde_json::from_slice(&response.body)?;
        self.jwt_token = Some(login_response.token.clone());
        Ok(login_response)
    }
}
