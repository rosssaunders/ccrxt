use rest::{Method as HttpMethod, RequestBuilder};

use crate::bullish::{EndpointType, Errors, PrivateRestClient as RestClient, RestResult};

/// Endpoint constant for logout
const LOGOUT_ENDPOINT: &str = "/v1/users/logout";

impl RestClient {
    /// Logout
    ///
    /// Logout of the session associated with the JWT. Requires bearer token in authorization header.
    ///
    /// [docs](https://api.exchange.bullish.com/trading-api/v1/users/logout)
    ///
    /// Rate limit: True
    ///
    /// # Returns
    /// Empty response (success indicated by HTTP status)
    pub async fn logout(&mut self) -> RestResult<()> {
        // Check rate limits
        self.rate_limiter
            .check_limits(EndpointType::PrivateLogin)
            .await
            .map_err(|e| Errors::RateLimitError(e.to_string()))?;

        // Ensure we have a valid JWT token
        let token = match self.jwt_token.as_ref() {
            Some(t) => t,
            None => {
                return Err(Errors::AuthenticationError(
                    "No JWT token available for logout".to_string(),
                ));
            }
        };

        let url = format!("{}/trading-api{}", self.base_url, LOGOUT_ENDPOINT);

        let request = RequestBuilder::new(HttpMethod::Get, url)
            .header("Authorization", format!("Bearer {}", token))
            .build();
        let response = self
            .http_client
            .execute(request)
            .await
            .map_err(Errors::from)?;

        self.rate_limiter
            .increment_request(EndpointType::PrivateLogin)
            .await;

        if response.is_success() {
            // Clear the JWT token on successful logout
            self.jwt_token = None;
            Ok(())
        } else {
            let error_text = response.text().unwrap_or_default();
            Err(Errors::AuthenticationError(format!(
                "Logout failed: {error_text}"
            )))
        }
    }
}
