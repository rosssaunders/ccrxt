use super::client::RestClient;
use crate::deribit::public::rest::RestResult;
use crate::deribit::rate_limit::EndpointType;

impl RestClient {
    /// Get current server time
    ///
    /// Retrieves the current time (in milliseconds). This API endpoint can be used to
    /// check the clock skew between your software and Deribit's systems.
    ///
    /// See: https://docs.deribit.com/#public-get_time
    ///
    /// # Returns
    /// Current timestamp (milliseconds since the UNIX epoch)
    pub async fn get_time(&self) -> RestResult<u64> {
        self.send_request::<u64, ()>(
            "public/get_time",
            None,
            EndpointType::PublicGetTime,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deribit::rate_limit::{AccountTier, RateLimiter};
    use reqwest::Client;

    #[tokio::test]
    async fn test_get_time_method_compilation() {
        // This test ensures the get_time method compiles and is accessible
        // without needing to make an actual HTTP request
        let client = Client::new();
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);
        let rest_client = RestClient::new("https://test.deribit.com", client, rate_limiter);

        // Verify the method exists and is properly typed
        let _ = RestClient::get_time;
        let _ = &rest_client;

        // This proves the method signature is correct without calling it
        println!("get_time method is accessible and properly typed");
    }

    #[test]
    fn test_endpoint_type_for_get_time() {
        let endpoint_type = EndpointType::from_path("public/get_time");
        assert_eq!(endpoint_type, EndpointType::PublicGetTime);
        assert_eq!(endpoint_type.credit_cost(), 0);
    }
}