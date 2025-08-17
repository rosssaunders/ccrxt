use crate::bullish::{EndpointType, Errors, RestResult, private::rest::RestClient};

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

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;

        self.rate_limiter
            .increment_request(EndpointType::PrivateLogin)
            .await;

        if response.status().is_success() {
            // Clear the JWT token on successful logout
            self.jwt_token = None;
            Ok(())
        } else {
            let error_text = response.text().await?;
            Err(Errors::AuthenticationError(format!(
                "Logout failed: {error_text}"
            )))
        }
    }
}
